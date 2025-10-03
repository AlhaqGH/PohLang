use crate::parser::{Program, Stmt, Expr, Param, CmpOp};
use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Clone, Debug)]
enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Func(Func),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
}

#[derive(Clone, Debug)]
struct Func {
    name: String,
    params: Vec<Param>,
    body: Expr,
    captured: Vec<HashMap<String, Value>>, // lexical chain from inner to outer
}

pub struct Vm {
    globals: HashMap<String, Value>,
    base_dir: PathBuf,
    loading_stack: Vec<String>,
    loaded_modules: HashSet<String>,
    loaded_system: HashSet<String>,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            globals: HashMap::new(),
            base_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            loading_stack: Vec::new(),
            loaded_modules: HashSet::new(),
            loaded_system: HashSet::new(),
        }
    }
}

impl Vm {
    pub fn with_base_dir(base: PathBuf) -> Self {
        Vm { globals: HashMap::new(), base_dir: base, loading_stack: Vec::new(), loaded_modules: HashSet::new(), loaded_system: HashSet::new() }
    }
}

impl Vm {
    pub fn execute(&mut self, prog: &Program) -> Result<()> {
        for stmt in prog {
            match stmt {
                Stmt::Write(e) => {
                    let v = self.eval(e)?;
                    println!("{}", to_string(&v));
                }
                Stmt::AskFor { var_name } => {
                    use std::io::{self, BufRead};
                    let stdin = io::stdin();
                    let mut line = String::new();
                    stdin.lock().read_line(&mut line)?;
                    // Trim newline
                    let input = line.trim_end().to_string();
                    // Try to parse as number, otherwise store as string
                    let value = if let Ok(n) = input.parse::<f64>() {
                        Value::Num(n)
                    } else {
                        Value::Str(input)
                    };
                    self.globals.insert(var_name.clone(), value);
                }
                Stmt::IfInline { cond, then_write, otherwise_write } => {
                    let c = self.truthy(&self.eval(cond)?)?;
                    if c {
                        let v = self.eval(then_write)?;
                        println!("{}", to_string(&v));
                    } else if let Some(e) = otherwise_write {
                        let v = self.eval(e)?;
                        println!("{}", to_string(&v));
                    }
                }
                Stmt::IfBlock { cond, then_body, otherwise_body } => {
                    let c = self.truthy(&self.eval(cond)?)?;
                    if c { self.execute(then_body)?; } else if let Some(eb) = otherwise_body { self.execute(eb)?; }
                }
                Stmt::FuncInline { name, params, body } => {
                    let f = Func { name: name.clone(), params: params.clone(), body: body.clone(), captured: vec![self.globals.clone()] };
                    self.globals.insert(name.clone(), Value::Func(f));
                }
                Stmt::FuncBlock { name, params, body } => {
                    // Represent as a Func with a synthetic body: we will store a special marker by encoding the body as a call to an internal evaluator.
                    // For simplicity, store block body in a separate map keyed by function name.
                    let f = Func { name: name.clone(), params: params.clone(), body: Expr::Ident(format!("__fn_body__{}", name)), captured: vec![self.globals.clone()] };
                    self.globals.insert(name.clone(), Value::Func(f));
                    // Also record the body in globals under a special key as a Value::Func with no params meaning executable block
                    // We'll store Program as a serialized form using a pointer-like trick via Box in a separate field.
                    // To avoid a big refactor, store the block body as a special global value string key mapping to a boxed Program in an auxiliary table.
                    // For now, we attach it to a static once cell (not ideal) - keep it minimal:
                    FN_BLOCKS.with(|m| { m.borrow_mut().insert(format!("__fn_body__{}", name), body.clone()); });
                }
                Stmt::WhileBlock { cond, body } => {
                    // Evaluate while the condition is truthy; prevent infinite tight loop by a simple iteration cap for safety (optional)
                    let mut guard = 0usize;
                    while self.truthy(&self.eval(cond)?)? {
                        self.execute(body)?;
                        guard += 1;
                        if guard > 1_000_000 { break; }
                    }
                }
                Stmt::RepeatBlock { count, body } => {
                    let n = match self.eval(count)? { Value::Num(x) => x.max(0.0) as i64, v => {
                        // Non-number: coerce via length-like string len or treat as 0; keep it simple: 0
                        let _ = v; 0 }
                    };
                    for _ in 0..n { self.execute(body)?; }
                }
                Stmt::ImportLocal { path } => {
                    self.import_local(path)?;
                }
                Stmt::ImportSystem { name } => {
                    self.import_system(name)?;
                }
                Stmt::Use { name, args } => {
                    let argv = args.iter().map(|e| self.eval(e)).collect::<Result<Vec<_>>>()?;
                    let _ = self.call_function(name, &argv)?;
                }
                Stmt::Set { name, value } => {
                    let v = self.eval(value)?;
                    self.globals.insert(name.clone(), v);
                }
                Stmt::Return(_) => { /* top-level Return ignored */ }
            }
        }
        Ok(())
    }

    pub fn execute_bytecode(&mut self, bc: &[u8]) -> Result<()> {
        // Very small bytecode interpreter: lines of UTF-8 text with op and tab-separated payload
        for line in bc.split(|&b| b == b'\n') {
            if line.is_empty() { continue; }
            let s = String::from_utf8_lossy(line);
            let mut parts = s.splitn(2, '\t');
            let op = parts.next().unwrap_or("");
            let arg = parts.next().unwrap_or("");
            match op {
                "WRITE" => println!("{}", arg),
                _ => {}
            }
        }
        Ok(())
    }

    fn eval(&self, e: &Expr) -> Result<Value> {
        match e {
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Num(n) => Ok(Value::Num(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Ident(name) => {
                if let Some(v) = self.globals.get(name) { return Ok(v.clone()); }
                Ok(Value::Str(format!("<{}>", name)))
            }
            Expr::Plus(a, b) => {
                let sa = self.eval(a)?;
                let sb = self.eval(b)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y))))
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval(a)?)?;
                if !la { return Ok(Value::Num(0.0)); }
                let lb = self.truthy(&self.eval(b)?)?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval(a)?)?;
                if la { return Ok(Value::Num(1.0)); }
                let lb = self.truthy(&self.eval(b)?)?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval(a)?)?;
                Ok(if !la { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval(l)?;
                let rv = self.eval(r)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv { Value::Num(n) => Some(n), _ => None };
                        let rn = match rv { Value::Num(n) => Some(n), _ => None };
                        if let (Some(a), Some(b)) = (ln, rn) {
                            match op {
                                CmpOp::Lt => a < b,
                                CmpOp::Le => a <= b,
                                CmpOp::Gt => a > b,
                                CmpOp::Ge => a >= b,
                                _ => unreachable!(),
                            }
                        } else {
                            false
                        }
                    }
                };
                Ok(if res { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Call { name, args } => {
                let argv = args.iter().map(|e| self.eval(e)).collect::<Result<Vec<_>>>()?;
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items { out.push(self.eval(it)?); }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs { map.insert(k.clone(), self.eval(ve)?); }
                Ok(Value::Dict(map))
            }
        }
    }

    fn truthy(&self, v: &Value) -> Result<bool> {
        match v {
            Value::Num(n) => Ok(*n != 0.0),
            Value::Str(s) => Ok(!s.is_empty()),
            Value::Bool(b) => Ok(*b),
            Value::Null => Ok(false),
            Value::Func(_) => Ok(true),
            Value::List(v) => Ok(!v.is_empty()),
            Value::Dict(m) => Ok(!m.is_empty()),
        }
    }

    fn call_function(&self, name: &str, args: &[Value]) -> Result<Value> {
        // Built-ins
        match name {
            "now" if args.is_empty() => { return Ok(Value::Str(iso_now())); }
            "range" => { return builtin_range(args); }
            "join" => { return builtin_join(args); }
            "split" => { return builtin_split(args); }
            "length" => { return builtin_length(args); }
            "len" => { return builtin_length(args); }
            _ => {}
        }
        // User-defined
        match self.globals.get(name) {
            Some(Value::Func(f)) => {
                self.call_func_value(f, args)
            }
            _ => Err(anyhow!("Error: Function '{}' is not defined", name)),
        }
    }

    fn call_func_value(&self, f: &Func, args: &[Value]) -> Result<Value> {
        // Arity with defaults
        let required = f.params.iter().filter(|p| p.default.is_none()).count();
        if args.len() < required || args.len() > f.params.len() {
            return Err(anyhow!("Function '{}' expects {}..{} args but got {}", f.name, required, f.params.len(), args.len()));
        }
        // Locals map
        let mut locals: HashMap<String, Value> = HashMap::new();
        for (i, p) in f.params.iter().enumerate() {
            if i < args.len() {
                locals.insert(p.name.clone(), args[i].clone());
            } else if let Some(def) = &p.default {
                let v = self.eval(def)?; // evaluate default at call-time
                locals.insert(p.name.clone(), v);
            }
        }
        // If body is a synthetic Ident to a stored block, execute that block with a frame
        if let Expr::Ident(synth) = &f.body {
            if synth.starts_with("__fn_body__") {
                if let Some(body_prog) = FN_BLOCKS.with(|m| m.borrow().get(synth).cloned()) {
                    return self.execute_function_block(body_prog, &locals, &f.captured);
                }
            }
        }
        self.eval_in_scope_with_capture(&f.body, &locals, &f.captured)
    }

    fn execute_function_block(&self, body: Program, locals: &HashMap<String, Value>, captured: &Vec<HashMap<String, Value>>) -> Result<Value> {
        // Create a scope stack: locals (top), captured (next), globals (fallback)
        // Execute statements until Return encountered; return its value or 'nothing' (empty string) if none.
    let mut frame = Frame { locals: locals.clone(), captured: captured.clone() };
        match self.exec_block_with_frame(&body, &mut frame) {
            ControlFlow::Return(v) => Ok(v.unwrap_or(Value::Str(String::new()))),
            ControlFlow::Continue => Ok(Value::Str(String::new())),
        }
    }

    fn exec_block_with_frame(&self, body: &Program, frame: &mut Frame) -> ControlFlow {
        for stmt in body {
            match stmt {
                Stmt::Write(e) => {
                    if let Ok(v) = self.eval_in_frame(e, frame) {
                        println!("{}", to_string(&v));
                    }
                }
                Stmt::AskFor { var_name } => {
                    use std::io::{self, BufRead};
                    let stdin = io::stdin();
                    let mut line = String::new();
                    if stdin.lock().read_line(&mut line).is_ok() {
                        let input = line.trim_end().to_string();
                        let value = if let Ok(n) = input.parse::<f64>() {
                            Value::Num(n)
                        } else {
                            Value::Str(input)
                        };
                        frame.locals.insert(var_name.clone(), value);
                    }
                }
                Stmt::Set { name, value } => {
                    if let Ok(v) = self.eval_in_frame(value, frame) {
                        frame.locals.insert(name.clone(), v);
                    }
                }
                Stmt::IfInline { cond, then_write, otherwise_write } => {
                    if let Ok(c) = self.truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0))) {
                        if c {
                            if let Ok(v) = self.eval_in_frame(then_write, frame) { println!("{}", to_string(&v)); }
                        } else if let Some(e) = otherwise_write {
                            if let Ok(v) = self.eval_in_frame(e, frame) { println!("{}", to_string(&v)); }
                        }
                    }
                }
                Stmt::IfBlock { cond, then_body, otherwise_body } => {
                    if let Ok(c) = self.truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0))) {
                        let cf = if c { self.exec_block_with_frame(then_body, frame) } else if let Some(eb) = otherwise_body { self.exec_block_with_frame(eb, frame) } else { ControlFlow::Continue };
                        if let ControlFlow::Return(_) = cf { return cf; }
                    }
                }
                Stmt::WhileBlock { cond, body } => {
                    let mut guard = 0usize;
                    while self.truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0))).unwrap_or(false) {
                        let cf = self.exec_block_with_frame(body, frame);
                        if let ControlFlow::Return(_) = cf { return cf; }
                        guard += 1; if guard > 1_000_000 { break; }
                    }
                }
                Stmt::RepeatBlock { count, body } => {
                    let n = match self.eval_in_frame(count, frame).ok() { Some(Value::Num(x)) => x.max(0.0) as i64, _ => 0 };
                    for _ in 0..n {
                        let cf = self.exec_block_with_frame(body, frame);
                        if let ControlFlow::Return(_) = cf { return cf; }
                    }
                }
                Stmt::ImportLocal { .. } => {
                    // Imports inside function frames are ignored at runtime-frame level;
                    // they should be handled at module load/top-level execution.
                }
                Stmt::ImportSystem { .. } => {
                    // Same as above: ignore within function frames.
                }
                Stmt::FuncInline { name, params, body } => {
                    let mut captured = frame.captured.clone();
                    // capture chain + current locals as the first (innermost) env
                    captured.insert(0, frame.locals.clone());
                    let f = Func { name: name.clone(), params: params.clone(), body: body.clone(), captured };
                    frame.locals.insert(name.clone(), Value::Func(f));
                }
                Stmt::FuncBlock { name, params, body: b } => {
                    let mut captured = frame.captured.clone();
                    captured.insert(0, frame.locals.clone());
                    let f = Func { name: name.clone(), params: params.clone(), body: Expr::Ident(format!("__fn_body__{}", name)), captured };
                    FN_BLOCKS.with(|m| { m.borrow_mut().insert(format!("__fn_body__{}", name), b.clone()); });
                    frame.locals.insert(name.clone(), Value::Func(f));
                }
                Stmt::Use { name, args } => {
                    let argv = args.iter().filter_map(|e| self.eval_in_frame(e, frame).ok()).collect::<Vec<_>>();
                    // Resolve function from current frame first, then captured, then globals
                    if let Some(Value::Func(f)) = frame.locals.get(name) {
                        let _ = self.call_func_value(f, &argv);
                    } else if let Some(Value::Func(f)) = self.globals.get(name) {
                        let _ = self.call_func_value(f, &argv);
                    } else {
                        // search in captured chain
                        let mut done = false;
                        for env in frame.captured.iter() {
                            if let Some(Value::Func(f)) = env.get(name) { let _ = self.call_func_value(f, &argv); done = true; break; }
                        }
                        if !done { let _ = self.call_function(name, &argv); }
                    }
                }
                Stmt::Return(expr) => {
                    let v = match expr { Some(e) => self.eval_in_frame(e, frame).ok(), None => None };
                    return ControlFlow::Return(v);
                }
            }
        }
        ControlFlow::Continue
    }

    fn eval_in_frame(&self, e: &Expr, frame: &Frame) -> Result<Value> {
        match e {
            Expr::Ident(n) => {
                if let Some(v) = frame.locals.get(n) { return Ok(v.clone()); }
                for env in frame.captured.iter() { if let Some(v) = env.get(n) { return Ok(v.clone()); } }
                if let Some(v) = self.globals.get(n) { return Ok(v.clone()); }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_frame(a, frame)?;
                let sb = self.eval_in_frame(b, frame)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y))))
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                if !la { return Ok(Value::Num(0.0)); }
                let lb = self.truthy(&self.eval_in_frame(b, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                if la { return Ok(Value::Num(1.0)); }
                let lb = self.truthy(&self.eval_in_frame(b, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if !la { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_frame(l, frame)?;
                let rv = self.eval_in_frame(r, frame)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv { Value::Num(n) => Some(n), _ => None };
                        let rn = match rv { Value::Num(n) => Some(n), _ => None };
                        if let (Some(a), Some(b)) = (ln, rn) {
                            match op {
                                CmpOp::Lt => a < b,
                                CmpOp::Le => a <= b,
                                CmpOp::Gt => a > b,
                                CmpOp::Ge => a >= b,
                                _ => unreachable!(),
                            }
                        } else { false }
                    }
                };
                Ok(if res { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Call { name, args } => {
                let argv = args.iter().map(|e| self.eval_in_frame(e, frame)).collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() { return Ok(Value::Str(iso_now())); }
                // Try resolve function in local frame hierarchy first
                if let Some(Value::Func(f)) = frame.locals.get(name) { return self.call_func_value(f, &argv); }
                for env in frame.captured.iter() { if let Some(Value::Func(f)) = env.get(name) { return self.call_func_value(f, &argv); } }
                if let Some(Value::Func(f)) = self.globals.get(name) { return self.call_func_value(f, &argv); }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items { out.push(self.eval_in_frame(it, frame)?); }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs { map.insert(k.clone(), self.eval_in_frame(ve, frame)?); }
                Ok(Value::Dict(map))
            }
        }
    }

    fn eval_in_scope(&self, e: &Expr, locals: &HashMap<String, Value>) -> Result<Value> {
        match e {
            Expr::Ident(n) => {
                if let Some(v) = locals.get(n) { return Ok(v.clone()); }
                if let Some(v) = self.globals.get(n) { return Ok(v.clone()); }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_scope(a, locals)?;
                let sb = self.eval_in_scope(b, locals)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y))))
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                if !la { return Ok(Value::Num(0.0)); }
                let lb = self.truthy(&self.eval_in_scope(b, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                if la { return Ok(Value::Num(1.0)); }
                let lb = self.truthy(&self.eval_in_scope(b, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if !la { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_scope(l, locals)?;
                let rv = self.eval_in_scope(r, locals)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv { Value::Num(n) => Some(n), _ => None };
                        let rn = match rv { Value::Num(n) => Some(n), _ => None };
                        if let (Some(a), Some(b)) = (ln, rn) {
                            match op {
                                CmpOp::Lt => a < b,
                                CmpOp::Le => a <= b,
                                CmpOp::Gt => a > b,
                                CmpOp::Ge => a >= b,
                                _ => unreachable!(),
                            }
                        } else { false }
                    }
                };
                Ok(if res { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Call { name, args } => {
                let argv = args.iter().map(|e| self.eval_in_scope(e, locals)).collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() { return Ok(Value::Str(iso_now())); }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items { out.push(self.eval_in_scope(it, locals)?); }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs { map.insert(k.clone(), self.eval_in_scope(ve, locals)?); }
                Ok(Value::Dict(map))
            }
        }
    }

    fn eval_in_scope_with_capture(&self, e: &Expr, locals: &HashMap<String, Value>, captured: &Vec<HashMap<String, Value>>) -> Result<Value> {
        match e {
            Expr::Ident(n) => {
                if let Some(v) = locals.get(n) { return Ok(v.clone()); }
                for env in captured.iter() { if let Some(v) = env.get(n) { return Ok(v.clone()); } }
                if let Some(v) = self.globals.get(n) { return Ok(v.clone()); }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_scope_with_capture(a, locals, captured)?;
                let sb = self.eval_in_scope_with_capture(b, locals, captured)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y))))
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval_in_scope_with_capture(a, locals, captured).unwrap_or(Value::Num(0.0)))?;
                if !la { return Ok(Value::Num(0.0)); }
                let lb = self.truthy(&self.eval_in_scope_with_capture(b, locals, captured).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval_in_scope_with_capture(a, locals, captured).unwrap_or(Value::Num(0.0)))?;
                if la { return Ok(Value::Num(1.0)); }
                let lb = self.truthy(&self.eval_in_scope_with_capture(b, locals, captured).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval_in_scope_with_capture(a, locals, captured).unwrap_or(Value::Num(0.0)))?;
                Ok(if !la { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_scope_with_capture(l, locals, captured)?;
                let rv = self.eval_in_scope_with_capture(r, locals, captured)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv { Value::Num(n) => Some(n), _ => None };
                        let rn = match rv { Value::Num(n) => Some(n), _ => None };
                        if let (Some(a), Some(b)) = (ln, rn) {
                            match op {
                                CmpOp::Lt => a < b,
                                CmpOp::Le => a <= b,
                                CmpOp::Gt => a > b,
                                CmpOp::Ge => a >= b,
                                _ => unreachable!(),
                            }
                        } else { false }
                    }
                };
                Ok(if res { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Call { name, args } => {
                let argv = args.iter().map(|e| self.eval_in_scope_with_capture(e, locals, captured)).collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() { return Ok(Value::Str(iso_now())); }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items { out.push(self.eval_in_scope_with_capture(it, locals, captured)?); }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs { map.insert(k.clone(), self.eval_in_scope_with_capture(ve, locals, captured)?); }
                Ok(Value::Dict(map))
            }
        }
    }
}

pub fn compile(prog: &Program) -> Vec<u8> {
    // Extremely small placeholder bytecode: lines of opcodes in UTF-8
    // WRITE\t<text>\n
    // ASK\t<var_name>\n
    let mut out = Vec::new();
    for stmt in prog {
        match stmt {
            Stmt::Write(e) => {
                out.extend_from_slice(b"WRITE\t");
                let s = dump_expr(e);
                out.extend_from_slice(s.as_bytes());
                out.push(b'\n');
            }
            Stmt::AskFor { var_name } => {
                out.extend_from_slice(b"ASK\t");
                out.extend_from_slice(var_name.as_bytes());
                out.push(b'\n');
            }
            _ => {}
        }
    }
    out
}

fn dump_expr(e: &Expr) -> String {
    match e {
        Expr::Str(s) => s.clone(),
        Expr::Num(n) => format_number(*n),
        Expr::Bool(b) => if *b { "True".to_string() } else { "False".to_string() },
    Expr::Null => "None".to_string(),
        Expr::Ident(x) => x.clone(),
        Expr::Plus(a, b) => format!("{} plus {}", dump_expr(a), dump_expr(b)),
        Expr::And(a, b) => format!("{} And {}", dump_expr(a), dump_expr(b)),
        Expr::Or(a, b) => format!("{} Or {}", dump_expr(a), dump_expr(b)),
        Expr::Not(a) => format!("Not {}", dump_expr(a)),
        Expr::Cmp(op, l, r) => {
            let sym = match op { CmpOp::Lt => "<", CmpOp::Le => "<=", CmpOp::Gt => ">", CmpOp::Ge => ">=", CmpOp::Eq => "=", CmpOp::Ne => "!=", };
            format!("{} {} {}", dump_expr(l), sym, dump_expr(r))
        }
        Expr::Call { name, args } => if args.is_empty() { name.clone() } else { format!("{} with {}", name, args.iter().map(dump_expr).collect::<Vec<_>>().join(", ")) },
        Expr::ListLit(items) => format!("List contains {}", items.iter().map(dump_expr).collect::<Vec<_>>().join(", ")),
        Expr::DictLit(pairs) => pairs.iter().map(|(k, v)| format!("\"{}\" set to {}", k, dump_expr(v))).collect::<Vec<_>>().join(", "),
    }
}

fn format_number(n: f64) -> String {
    if (n.fract()).abs() < f64::EPSILON {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

fn iso_now() -> String {
    // Avoid adding a chrono dependency; simple ISO-ish string
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    format!("{}", now.as_secs())
}

fn to_string(v: &Value) -> String {
    match v {
        Value::Str(s) => s.clone(),
        Value::Num(n) => format_number(*n),
        Value::Bool(b) => if *b { "True".to_string() } else { "False".to_string() },
    Value::Null => "None".to_string(),
        Value::Func(f) => format!("<function {}>", f.name),
        Value::List(xs) => format!("[{}]", xs.iter().map(|x| to_string(x)).collect::<Vec<_>>().join(", ")),
        Value::Dict(m) => {
            let mut parts: Vec<String> = Vec::new();
            for (k, v) in m.iter() { parts.push(format!("\"{}\": {}", k, to_string(v))); }
            format!("{{{}}}", parts.join(", "))
        }
    }
}

thread_local! {
    static FN_BLOCKS: std::cell::RefCell<HashMap<String, Program>> = std::cell::RefCell::new(HashMap::new());
}

#[derive(Clone, Debug)]
struct Frame {
    locals: HashMap<String, Value>,
    captured: Vec<HashMap<String, Value>>,
}

#[derive(Clone, Debug)]
enum ControlFlow {
    Return(Option<Value>),
    Continue,
}

// ------------------------ BUILT-INS ------------------------

fn builtin_range(args: &[Value]) -> Result<Value> {
    let nums: Vec<f64> = match args.len() {
        1 => {
            if let Value::Num(n) = args[0] { (0..n.max(0.0) as i64).map(|i| i as f64).collect() } else { vec![] }
        }
        2 => {
            if let (Value::Num(a), Value::Num(b)) = (&args[0], &args[1]) {
                let (a, b) = (*a as i64, *b as i64);
                (a..b).map(|i| i as f64).collect()
            } else { vec![] }
        }
        3 => {
            if let (Value::Num(a), Value::Num(b), Value::Num(s)) = (&args[0], &args[1], &args[2]) {
                let (mut i, end, step) = (*a as i64, *b as i64, *s as i64);
                if step == 0 { return Ok(Value::List(Vec::new())); }
                let mut out = Vec::new();
                if step > 0 { while i < end { out.push(i as f64); i += step; } }
                else { while i > end { out.push(i as f64); i += step; } }
                out
            } else { vec![] }
        }
        _ => vec![],
    };
    Ok(Value::List(nums.into_iter().map(Value::Num).collect()))
}

fn builtin_join(args: &[Value]) -> Result<Value> {
    if args.is_empty() { return Ok(Value::Str(String::new())); }
    let sep = if args.len() >= 2 { to_string(&args[1]) } else { String::new() };
    match &args[0] {
        Value::List(xs) => Ok(Value::Str(xs.iter().map(|v| to_string(v)).collect::<Vec<_>>().join(&sep))),
        other => Ok(Value::Str(to_string(other))),
    }
}

fn builtin_split(args: &[Value]) -> Result<Value> {
    if args.len() < 2 { return Ok(Value::List(Vec::new())); }
    let s = to_string(&args[0]);
    let sep = to_string(&args[1]);
    let parts = if sep.is_empty() { s.chars().map(|c| Value::Str(c.to_string())).collect() }
        else { s.split(&sep).map(|p| Value::Str(p.to_string())).collect() };
    Ok(Value::List(parts))
}

fn builtin_length(args: &[Value]) -> Result<Value> {
    if args.is_empty() { return Ok(Value::Num(0.0)); }
    match &args[0] {
        Value::List(xs) => Ok(Value::Num(xs.len() as f64)),
        Value::Str(s) => Ok(Value::Num(s.chars().count() as f64)),
        Value::Dict(m) => Ok(Value::Num(m.len() as f64)),
        _ => Ok(Value::Num(0.0)),
    }
}

// ------------------------ IMPORTS ------------------------

impl Vm {
    fn import_local(&mut self, rel: &str) -> Result<()> {
        let mut path = self.base_dir.join(rel);
        if path.extension().is_none() { path.set_extension("poh"); }
        let canon = fs::canonicalize(&path).unwrap_or(path.clone());
        let key = canon.to_string_lossy().to_string();
        if self.loaded_modules.contains(&key) { return Ok(()); }
        if self.loading_stack.contains(&key) { return Err(anyhow!("Circular import detected: {}", key)); }
        let src = fs::read_to_string(&canon)?;
        let program = crate::parser::parse(&src)?;
        // push base dir
        let prev_base = self.base_dir.clone();
        self.base_dir = canon.parent().unwrap_or(Path::new(".")).to_path_buf();
        self.loading_stack.push(key.clone());
        self.execute(&program)?;
        self.loading_stack.pop();
        self.loaded_modules.insert(key);
        self.base_dir = prev_base;
        Ok(())
    }

    fn import_system(&mut self, name: &str) -> Result<()> {
        if self.loaded_system.contains(name) { return Ok(()); }
        // Resolve stdlib path
        let file_name = format!("{}.poh", name);
        if let Some(path) = self.find_stdlib_module(&file_name) {
            let canon = fs::canonicalize(&path).unwrap_or(path.clone());
            let src = fs::read_to_string(&canon)?;
            let program = crate::parser::parse(&src)?;
            // Execute with base_dir set to stdlib module dir
            let prev_base = self.base_dir.clone();
            self.base_dir = canon.parent().unwrap_or(Path::new(".")).to_path_buf();
            self.loading_stack.push(format!("<system:{}>", name));
            self.execute(&program)?;
            self.loading_stack.pop();
            self.base_dir = prev_base;
            self.loaded_system.insert(name.to_string());
            Ok(())
        } else {
            // Backward-compatible: if not found, treat as no-op stub
            self.loaded_system.insert(name.to_string());
            Ok(())
        }
    }

    fn find_stdlib_module(&self, file_name: &str) -> Option<PathBuf> {
        // 1) Env override
        if let Ok(root) = std::env::var("POHLANG_STDLIB") {
            let p = PathBuf::from(root).join(file_name);
            if p.exists() { return Some(p); }
        }
        // 2) Search from base_dir upwards for Interpreter/stdlib/<file>
        let mut cur: Option<&Path> = Some(self.base_dir.as_path());
        while let Some(dir) = cur {
            let cand = dir.join("Interpreter").join("stdlib").join(file_name);
            if cand.exists() { return Some(cand); }
            cur = dir.parent();
        }
        // 3) Try CWD fallback
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let cand = cwd.join("Interpreter").join("stdlib").join(file_name);
        if cand.exists() { return Some(cand); }
        None
    }
}
