use super::instructions::Instruction;
use crate::core::io as core_io;
use crate::parser::ast::{CmpOp, Expr, Param, Program, Stmt};
use crate::stdlib::errors::{ErrorKind, PohError, StackFrame};
use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn enhance_error(msg: &str) -> String {
    if msg.contains("out of range") {
        format!(
            "{}.\nHint: Check array bounds. Use negative indexing (-1) for last element",
            msg
        )
    } else if msg.contains("not found") {
        format!(
            "{}.\nHint: Verify the key exists in the dictionary or check for typos",
            msg
        )
    } else if msg.contains("division by zero") {
        format!(
            "{}.\nHint: Ensure denominator is not zero before dividing",
            msg
        )
    } else if msg.contains("not defined") {
        format!(
            "{}.\nHint: Make sure the function is defined before calling it",
            msg
        )
    } else {
        msg.to_string()
    }
}

#[derive(Clone, Debug)]
enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Func(Func),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Error(PohError),
    WebServer(std::sync::Arc<std::sync::Mutex<crate::stdlib::http::WebServer>>),
    HttpRequest(crate::stdlib::http::HttpRequest),
    HttpResponse(crate::stdlib::http::HttpResponse),
    LiveReloadTracker(crate::stdlib::livereload::LiveReloadTracker),
}

#[derive(Clone, Debug)]
struct Func {
    name: String,
    params: Vec<Param>,
    body: Expr,
    captured: Vec<HashMap<String, Value>>, // lexical chain from inner to outer
}

#[derive(Clone, Debug)]
struct CallFrame {
    function_name: String,
    file: String,
    line: usize,
}

pub struct Vm {
    globals: HashMap<String, Value>,
    base_dir: PathBuf,
    loading_stack: Vec<String>,
    loaded_modules: HashSet<String>,
    loaded_system: HashSet<String>,
    system_exports: HashMap<String, HashMap<String, Value>>,
    module_aliases: HashMap<String, String>,
    exposed_symbols: HashMap<String, String>,
    call_stack: Vec<CallFrame>,
    current_file: String,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            globals: HashMap::new(),
            base_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            loading_stack: Vec::new(),
            loaded_modules: HashSet::new(),
            loaded_system: HashSet::new(),
            system_exports: HashMap::new(),
            module_aliases: HashMap::new(),
            exposed_symbols: HashMap::new(),
            call_stack: Vec::new(),
            current_file: String::from("<main>"),
        }
    }
}

impl Vm {
    pub fn with_base_dir(base: PathBuf) -> Self {
        Vm {
            globals: HashMap::new(),
            base_dir: base,
            loading_stack: Vec::new(),
            loaded_modules: HashSet::new(),
            loaded_system: HashSet::new(),
            system_exports: HashMap::new(),
            module_aliases: HashMap::new(),
            exposed_symbols: HashMap::new(),
            call_stack: Vec::new(),
            current_file: String::from("<main>"),
        }
    }
}

impl Vm {
    /// Set the current file being executed (for error reporting)
    pub fn set_current_file(&mut self, file: String) {
        self.current_file = file;
    }

    /// Get the current file being executed
    pub fn current_file(&self) -> &str {
        &self.current_file
    }

    /// Enable hot reload with file watching
    pub fn enable_hot_reload(&mut self, watch_paths: Vec<PathBuf>) {
        let tracker = crate::stdlib::livereload::LiveReloadTracker::new(watch_paths);
        self.globals.insert(
            "__livereload".to_string(),
            Value::LiveReloadTracker(tracker),
        );
    }

    /// Create an error with file location context
    fn error_with_location(&self, message: impl Into<String>) -> anyhow::Error {
        let msg = message.into();
        if self.current_file.is_empty() {
            anyhow::anyhow!("{}", msg)
        } else {
            anyhow::anyhow!("{}\n  in file: {}", msg, self.current_file)
        }
    }

    pub fn execute(&mut self, prog: &Program) -> Result<()> {
        for stmt in prog {
            match stmt {
                Stmt::Write(e) => {
                    let v = match e {
                        // If Write is given a bare function ident, attempt to call it with no args
                        Expr::Ident(name) => {
                            if let Some(Value::Func(f)) = self.globals.get(name) {
                                self.call_func_value(f, &[])?
                            } else {
                                self.eval(e)?
                            }
                        }
                        _ => self.eval(e)?,
                    };
                    core_io::write(&to_string(&v));
                }
                Stmt::AskFor { var_name } => {
                    let input = core_io::ask("");
                    // Try to parse as number, otherwise store as string
                    let value = if let Ok(n) = input.parse::<f64>() {
                        Value::Num(n)
                    } else {
                        Value::Str(input)
                    };
                    self.globals.insert(var_name.clone(), value);
                }
                Stmt::IfInline {
                    cond,
                    then_write,
                    otherwise_write,
                } => {
                    let c = self.truthy(&self.eval(cond)?)?;
                    if c {
                        let v = self.eval(then_write)?;
                        println!("{}", to_string(&v));
                    } else if let Some(e) = otherwise_write {
                        let v = self.eval(e)?;
                        println!("{}", to_string(&v));
                    }
                }
                Stmt::IfBlock {
                    cond,
                    then_body,
                    otherwise_body,
                } => {
                    let c = self.truthy(&self.eval(cond)?)?;
                    if c {
                        self.execute(then_body)?;
                    } else if let Some(eb) = otherwise_body {
                        self.execute(eb)?;
                    }
                }
                Stmt::FuncInline { name, params, body } => {
                    let f = Func {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        captured: vec![self.globals.clone()],
                    };
                    self.globals.insert(name.clone(), Value::Func(f));
                }
                Stmt::FuncBlock { name, params, body } => {
                    // Represent as a Func with a synthetic body: we will store a special marker by encoding the body as a call to an internal evaluator.
                    // For simplicity, store block body in a separate map keyed by function name.
                    let f = Func {
                        name: name.clone(),
                        params: params.clone(),
                        body: Expr::Ident(format!("__fn_body__{}", name)),
                        captured: vec![self.globals.clone()],
                    };
                    self.globals.insert(name.clone(), Value::Func(f));
                    // Also record the body in globals under a special key as a Value::Func with no params meaning executable block
                    // We'll store Program as a serialized form using a pointer-like trick via Box in a separate field.
                    // To avoid a big refactor, store the block body as a special global value string key mapping to a boxed Program in an auxiliary table.
                    // For now, we attach it to a static once cell (not ideal) - keep it minimal:
                    FN_BLOCKS.with(|m| {
                        m.borrow_mut()
                            .insert(format!("__fn_body__{}", name), body.clone());
                    });
                }
                Stmt::WhileBlock { cond, body } => {
                    // Evaluate while the condition is truthy; prevent infinite tight loop by a simple iteration cap for safety (optional)
                    let mut guard = 0usize;
                    while self.truthy(&self.eval(cond)?)? {
                        self.execute(body)?;
                        guard += 1;
                        if guard > 1_000_000 {
                            break;
                        }
                    }
                }
                Stmt::RepeatBlock { count, body } => {
                    let n = match self.eval(count)? {
                        Value::Num(x) => x.max(0.0) as i64,
                        v => {
                            // Non-number: coerce via length-like string len or treat as 0; keep it simple: 0
                            let _ = v;
                            0
                        }
                    };
                    for _ in 0..n {
                        self.execute(body)?;
                    }
                }
                Stmt::ImportLocal { path } => {
                    self.import_local(path)?;
                }
                Stmt::ImportSystem {
                    name,
                    alias,
                    exposing,
                } => {
                    self.import_system(name, alias.as_deref(), exposing)?;
                }
                Stmt::Use { name, args } => {
                    let argv = args
                        .iter()
                        .map(|e| self.eval(e))
                        .collect::<Result<Vec<_>>>()?;
                    let ret = self.call_function(name, &argv)?;
                    // Print return if not None/empty
                    core_io::write(&to_string(&ret));
                }
                Stmt::Set { name, value } => {
                    let v = self.eval(value)?;
                    self.globals.insert(name.clone(), v);
                }
                Stmt::Return(_) => { /* top-level Return ignored */ }
                Stmt::TryCatch {
                    try_block,
                    catch_handlers,
                    finally_block,
                } => {
                    // Execute try block
                    let try_result = self.execute(try_block);

                    // If try succeeded or no catch handlers, run finally and return
                    if try_result.is_ok() || catch_handlers.is_empty() {
                        if let Some(fin) = finally_block {
                            self.execute(fin)?;
                        }
                        try_result?;
                        continue;
                    }

                    // Try block failed - extract error message
                    let err_msg = try_result.unwrap_err().to_string();

                    // Extract error type from [TypeName] marker if present
                    let error_type_from_msg = if let Some(start) = err_msg.find('[') {
                        if let Some(end) = err_msg.find(']') {
                            if start < end {
                                Some(err_msg[start + 1..end].to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    // Try to match a catch handler
                    let mut handled = false;
                    for handler in catch_handlers {
                        // Check if error type matches (if specified)
                        let type_matches = if let Some(ref error_type) = handler.error_type {
                            // Match against extracted type marker first, then fallback to message search
                            if let Some(ref msg_type) = error_type_from_msg {
                                msg_type.eq_ignore_ascii_case(error_type)
                            } else {
                                // Fallback: case-insensitive message search
                                err_msg.to_lowercase().contains(&error_type.to_lowercase())
                            }
                        } else {
                            true // No type specified = catch all
                        };

                        if type_matches {
                            // Bind error message to variable if specified
                            if let Some(ref var_name) = handler.var_name {
                                // Remove [TypeName] marker before binding for cleaner user display
                                let clean_msg = if let Some(end) = err_msg.find(']') {
                                    err_msg[end + 1..].trim().to_string()
                                } else {
                                    err_msg.clone()
                                };
                                self.globals.insert(var_name.clone(), Value::Str(clean_msg));
                            }

                            // Execute catch block
                            let catch_result = self.execute(&handler.block);

                            // Always execute finally block
                            if let Some(fin) = finally_block {
                                self.execute(fin)?;
                            }

                            catch_result?;
                            handled = true;
                            break;
                        }
                    }

                    // No matching catch handler - execute finally and re-raise
                    if !handled {
                        if let Some(fin) = finally_block {
                            self.execute(fin)?;
                        }
                        bail!("{}", err_msg);
                    }
                }
                Stmt::Throw(expr) => {
                    let val = self.eval(expr)?;
                    let error_msg = match val {
                        Value::Error(e) => e.format_with_trace(),
                        _ => {
                            let msg = to_string(&val);
                            let error = self.create_error(ErrorKind::RuntimeError, msg);
                            error.format_with_trace()
                        }
                    };
                    return Err(self.error_with_location(error_msg));
                }
                Stmt::AddRoute {
                    path,
                    method,
                    handler,
                } => {
                    // Get the server from globals (should be stored with key "server")
                    let server_val = self.globals.get("server").cloned()
                        .ok_or_else(|| anyhow!("No web server found. Create a server first with: Make server to Create web server on port <port>"))?;

                    let path_val = self.eval(path)?;
                    let method_val = self.eval(method)?;

                    let path_str = match path_val {
                        Value::Str(s) => s,
                        _ => bail!("add route: path must be a string"),
                    };

                    let method_str = match method_val {
                        Value::Str(s) => s.to_uppercase(),
                        _ => bail!("add route: method must be a string (GET, POST, PUT, DELETE)"),
                    };

                    // Clone the handler program to execute in the route
                    let handler_program = handler.clone();
                    let globals_snapshot = self.globals.clone();
                    let base_dir_snapshot = self.base_dir.clone();

                    // Create handler function that executes the PohLang code
                    let handler_fn = std::sync::Arc::new(
                        move |request: crate::stdlib::http::HttpRequest| {
                            // Create a new VM instance for this request
                            let mut vm = Vm::with_base_dir(base_dir_snapshot.clone());
                            vm.globals = globals_snapshot.clone();
                            
                            // Store request in VM context for access by handler
                            vm.globals.insert("__request".to_string(), Value::HttpRequest(request));

                            // Execute each statement and check for response values
                            for stmt in &handler_program {
                                match stmt {
                                    crate::parser::ast::Stmt::Write(expr) => {
                                        // Evaluate the expression
                                        match vm.eval(expr) {
                                            Ok(Value::HttpResponse(resp)) => {
                                                // Capture the response and return immediately
                                                return Ok(resp);
                                            }
                                            Ok(val) => {
                                                // Regular write - just evaluate it
                                                let _ = val;
                                            }
                                            Err(e) => {
                                                return Ok(crate::stdlib::http::error_response(
                                                    500,
                                                    format!("Handler error: {}", e),
                                                ));
                                            }
                                        }
                                    }
                                    _ => {
                                        // Execute other statements normally
                                        if let Err(e) = vm.execute(&vec![stmt.clone()]) {
                                            return Ok(crate::stdlib::http::error_response(
                                                500,
                                                format!("Handler error: {}", e),
                                            ));
                                        }
                                    }
                                }
                            }

                            // If no response was captured, return a default message
                            Ok(crate::stdlib::http::html_response(
                            "<h1>Handler executed</h1><p>No response returned. Use 'Write html response with ...' or 'Write json response with ...'</p>".to_string()
                        ))
                        },
                    );

                    // Add route to server
                    match server_val {
                        Value::WebServer(server_arc) => {
                            let mut server = server_arc.lock().unwrap();
                            server.add_route(path_str.clone(), method_str.clone(), handler_fn);
                            core_io::write(&format!("✓ Route added: {} {}", method_str, path_str));
                        }
                        _ => bail!("server variable is not a web server"),
                    }
                }
                Stmt::StartServer => {
                    // Get the server from globals
                    let server_val = self.globals.remove("server")
                        .ok_or_else(|| anyhow!("No web server found. Create a server first with: Make server to Create web server on port <port>"))?;

                    match server_val {
                        Value::WebServer(server_arc) => {
                            // Add hot reload route if LiveReloadTracker exists
                            if let Some(Value::LiveReloadTracker(tracker)) =
                                self.globals.get("__livereload")
                            {
                                let tracker_clone = tracker.clone();

                                // Create the /__reload_check route handler
                                let reload_handler = Arc::new(move |_req: crate::stdlib::http::HttpRequest| -> Result<crate::stdlib::http::HttpResponse> {
                                    let changed = tracker_clone.check_for_changes();
                                    let response_json = serde_json::json!({
                                        "changed": changed,
                                        "timestamp": std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_millis()
                                    });
                                    Ok(crate::stdlib::http::json_response(response_json))
                                });

                                // Add the route
                                let route = crate::stdlib::http::Route {
                                    path: "/__reload_check".to_string(),
                                    method: "GET".to_string(),
                                    handler: reload_handler,
                                };

                                server_arc.lock().unwrap().add_route_direct(route);
                                eprintln!("🔄 Hot reload enabled at /__reload_check");
                            }

                            eprintln!("[DEBUG] About to call server.start()...");

                            // Use the special function that doesn't hold the lock
                            crate::stdlib::http::start_server_from_arc(server_arc)?;

                            eprintln!("[DEBUG] Server.start() returned (unexpected!)");
                        }
                        _ => bail!("server variable is not a web server"),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn execute_bytecode(&mut self, bc: &[u8]) -> Result<()> {
        // Decode program into instruction vector
        let mut prog: Vec<Instruction> = Vec::new();
        for line in bc.split(|&b| b == b'\n') {
            if line.is_empty() {
                continue;
            }
            let s = String::from_utf8_lossy(line);
            if let Some(instr) = Instruction::decode(&s) {
                prog.push(instr);
            }
        }
        let mut stack: Vec<Value> = Vec::new();
        let mut ip: isize = 0;
        while let Some(instr) = prog.get(ip as usize).cloned() {
            match instr {
                Instruction::PushBool(b) => stack.push(Value::Bool(b)),
                Instruction::PushNull => stack.push(Value::Null),
                Instruction::PushNum(n) => stack.push(Value::Num(n)),
                Instruction::PushStr(s) => stack.push(Value::Str(s)),
                Instruction::LoadVar(name) => {
                    let v = self
                        .globals
                        .get(&name)
                        .cloned()
                        .unwrap_or(Value::Str(format!("<{}>", name)));
                    stack.push(v);
                }
                Instruction::StoreVar(name) => {
                    let v = stack.pop().unwrap_or(Value::Null);
                    self.globals.insert(name, v);
                }
                Instruction::Add => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num(to_num(a)? + to_num(b)?));
                }
                Instruction::Sub => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num(to_num(a)? - to_num(b)?));
                }
                Instruction::Mul => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num(to_num(a)? * to_num(b)?));
                }
                Instruction::Div => {
                    let b = stack.pop().unwrap_or(Value::Num(1.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    let db = to_num(b)?;
                    if db == 0.0 {
                        return Err(self.error_with_location("Division by zero"));
                    }
                    stack.push(Value::Num(to_num(a)? / db));
                }
                Instruction::Eq => {
                    let b = stack.pop().unwrap_or(Value::Null);
                    let a = stack.pop().unwrap_or(Value::Null);
                    stack.push(Value::Num((to_string(&a) == to_string(&b)) as i32 as f64));
                }
                Instruction::Ne => {
                    let b = stack.pop().unwrap_or(Value::Null);
                    let a = stack.pop().unwrap_or(Value::Null);
                    stack.push(Value::Num((to_string(&a) != to_string(&b)) as i32 as f64));
                }
                Instruction::Lt => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num((to_num(a)? < to_num(b)?) as i32 as f64));
                }
                Instruction::Le => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num((to_num(a)? <= to_num(b)?) as i32 as f64));
                }
                Instruction::Gt => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num((to_num(a)? > to_num(b)?) as i32 as f64));
                }
                Instruction::Ge => {
                    let b = stack.pop().unwrap_or(Value::Num(0.0));
                    let a = stack.pop().unwrap_or(Value::Num(0.0));
                    stack.push(Value::Num((to_num(a)? >= to_num(b)?) as i32 as f64));
                }
                Instruction::Jump(tgt) => {
                    ip = tgt as isize;
                    continue;
                }
                Instruction::JumpIfFalse(tgt) => {
                    let v = stack.pop().unwrap_or(Value::Num(0.0));
                    let truthy = match v {
                        Value::Num(n) => n != 0.0,
                        Value::Bool(b) => b,
                        Value::Str(s) => !s.is_empty(),
                        Value::Null => false,
                        _ => true,
                    };
                    if !truthy {
                        ip = tgt as isize;
                        continue;
                    }
                }
                Instruction::WriteTop => {
                    let v = stack.last().cloned().unwrap_or(Value::Null);
                    core_io::write(&to_string(&v));
                }
                Instruction::AskVar(name) => {
                    let input = core_io::ask("");
                    let value = if let Ok(n) = input.parse::<f64>() {
                        Value::Num(n)
                    } else {
                        Value::Str(input)
                    };
                    self.globals.insert(name, value);
                }
            }
            ip += 1;
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
                if let Some(v) = self.resolve_value(name) {
                    return Ok(v);
                }
                Ok(Value::Str(format!("<{}>", name)))
            }
            Expr::Plus(a, b) => {
                let sa = self.eval(a)?;
                let sb = self.eval(b)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y)))),
                }
            }
            Expr::Minus(a, b) => {
                let sa = self.eval(a)?;
                let sb = self.eval(b)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na - nb)),
                    _ => Err(anyhow!("Cannot subtract non-numeric values")),
                }
            }
            Expr::Times(a, b) => {
                let sa = self.eval(a)?;
                let sb = self.eval(b)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na * nb)),
                    _ => Err(anyhow!("Cannot multiply non-numeric values")),
                }
            }
            Expr::DividedBy(a, b) => {
                let sa = self.eval(a)?;
                let sb = self.eval(b)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => {
                        if nb == 0.0 {
                            Err(self.error_with_location("Division by zero"))
                        } else {
                            Ok(Value::Num(na / nb))
                        }
                    }
                    _ => Err(anyhow!("Cannot divide non-numeric values")),
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval(a)?)?;
                if !la {
                    return Ok(Value::Num(0.0));
                }
                let lb = self.truthy(&self.eval(b)?)?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval(a)?)?;
                if la {
                    return Ok(Value::Num(1.0));
                }
                let lb = self.truthy(&self.eval(b)?)?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval(a)?)?;
                Ok(if !la {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval(l)?;
                let rv = self.eval(r)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
                        let rn = match rv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
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
                Ok(if res {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Call { name, args } => {
                let argv = args
                    .iter()
                    .map(|e| self.eval(e))
                    .collect::<Result<Vec<_>>>()?;
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items {
                    out.push(self.eval(it)?);
                }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs {
                    map.insert(k.clone(), self.eval(ve)?);
                }
                Ok(Value::Dict(map))
            }
            Expr::Index(base, index) => {
                let base_val = self.eval(base)?;
                let index_val = self.eval(index)?;

                match (&base_val, &index_val) {
                    (Value::List(items), Value::Num(n)) => {
                        let idx = *n as i32;
                        let len = items.len() as i32;
                        // Support negative indexing
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            let msg =
                                format!("List index out of range: {} (list length: {})", idx, len);
                            return Err(anyhow!("{}", enhance_error(&msg)));
                        }
                        Ok(items[actual_idx as usize].clone())
                    }
                    (Value::Dict(map), Value::Str(key)) => map.get(key).cloned().ok_or_else(|| {
                        let msg = format!("Key not found in dictionary: \"{}\"", key);
                        anyhow!("{}", enhance_error(&msg))
                    }),
                    (Value::Str(s), Value::Num(n)) => {
                        let idx = *n as i32;
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "String index out of range: {} (string length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(Value::Str(chars[actual_idx as usize].to_string()))
                    }
                    _ => Err(anyhow!("Cannot index {:?} with {:?}", base_val, index_val)),
                }
            }
            // Phrasal built-in expressions
            Expr::TotalOf(expr) => {
                let val = self.eval(expr)?;
                builtin_sum(&[val])
            }
            Expr::SmallestIn(expr) => {
                let val = self.eval(expr)?;
                builtin_min(&[val])
            }
            Expr::LargestIn(expr) => {
                let val = self.eval(expr)?;
                builtin_max(&[val])
            }
            Expr::AbsoluteValueOf(expr) => {
                let val = self.eval(expr)?;
                builtin_abs(&[val])
            }
            Expr::Round(expr) => {
                let val = self.eval(expr)?;
                builtin_round(&[val])
            }
            Expr::RoundDown(expr) => {
                let val = self.eval(expr)?;
                builtin_floor(&[val])
            }
            Expr::RoundUp(expr) => {
                let val = self.eval(expr)?;
                builtin_ceil(&[val])
            }
            Expr::MakeUppercase(expr) => {
                let val = self.eval(expr)?;
                builtin_uppercase(&[val])
            }
            Expr::MakeLowercase(expr) => {
                let val = self.eval(expr)?;
                builtin_lowercase(&[val])
            }
            Expr::TrimSpaces(expr) => {
                let val = self.eval(expr)?;
                builtin_trim(&[val])
            }
            Expr::FirstIn(expr) => {
                let val = self.eval(expr)?;
                builtin_first(&[val])
            }
            Expr::LastIn(expr) => {
                let val = self.eval(expr)?;
                builtin_last(&[val])
            }
            Expr::ReverseOf(expr) => {
                let val = self.eval(expr)?;
                builtin_reverse(&[val])
            }
            Expr::CountOf(expr) => {
                let val = self.eval(expr)?;
                builtin_length(&[val])
            }
            Expr::JoinWith(a, b) => {
                let va = self.eval(a)?;
                let vb = self.eval(b)?;
                builtin_join(&[va, vb])
            }
            Expr::SplitBy(a, b) => {
                let va = self.eval(a)?;
                let vb = self.eval(b)?;
                builtin_split(&[va, vb])
            }
            // Collection operations
            Expr::Contains(item, collection) => {
                let item_val = self.eval(item)?;
                let coll_val = self.eval(collection)?;
                builtin_contains(&[item_val, coll_val])
            }
            Expr::Remove(item, list) => {
                let item_val = self.eval(item)?;
                let list_val = self.eval(list)?;
                builtin_remove(&[item_val, list_val])
            }
            Expr::Append(item, list) => {
                let item_val = self.eval(item)?;
                let list_val = self.eval(list)?;
                builtin_append(&[item_val, list_val])
            }
            Expr::InsertAt(item, index, list) => {
                let item_val = self.eval(item)?;
                let index_val = self.eval(index)?;
                let list_val = self.eval(list)?;
                builtin_insert_at(&[item_val, index_val, list_val])
            }
            // File I/O operations
            Expr::ReadFile(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("read file at: path must be a string"),
                };
                match crate::stdlib::file::read_file(&path) {
                    Ok(content) => Ok(Value::Str(content)),
                    Err(e) => bail!("Failed to read file '{}': {}", path, e),
                }
            }
            Expr::WriteFile(content_expr, path_expr) => {
                let content_val = self.eval(content_expr)?;
                let path_val = self.eval(path_expr)?;
                let content = match content_val {
                    Value::Str(s) => s,
                    Value::Num(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => bail!("write to file: content must be string, number, or boolean"),
                };
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("write to file at: path must be a string"),
                };
                match crate::stdlib::file::write_file(&path, &content) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to write file '{}': {}", path, e),
                }
            }
            Expr::AppendFile(content_expr, path_expr) => {
                let content_val = self.eval(content_expr)?;
                let path_val = self.eval(path_expr)?;
                let content = match content_val {
                    Value::Str(s) => s,
                    Value::Num(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => bail!("append to file: content must be string, number, or boolean"),
                };
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("append to file at: path must be a string"),
                };
                match crate::stdlib::file::append_file(&path, &content) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to append to file '{}': {}", path, e),
                }
            }
            Expr::FileExists(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("file exists at: path must be a string"),
                };
                Ok(Value::Bool(crate::stdlib::file::file_exists(&path)))
            }
            Expr::DeleteFile(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("delete file at: path must be a string"),
                };
                match crate::stdlib::file::delete_file(&path) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to delete file '{}': {}", path, e),
                }
            }
            Expr::CreateDir(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("create directory at: path must be a string"),
                };
                match crate::stdlib::file::create_directory(&path) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to create directory '{}': {}", path, e),
                }
            }
            Expr::ListDir(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("list files in: path must be a string"),
                };
                match crate::stdlib::file::list_directory(&path) {
                    Ok(files) => Ok(Value::List(files.into_iter().map(Value::Str).collect())),
                    Err(e) => bail!("Failed to list directory '{}': {}", path, e),
                }
            }
            Expr::ReadLines(path_expr) => {
                let path_val = self.eval(path_expr)?;
                let path = match path_val {
                    Value::Str(s) => s,
                    _ => bail!("read lines from: path must be a string"),
                };
                match crate::stdlib::file::read_lines(&path) {
                    Ok(lines) => Ok(Value::List(lines.into_iter().map(Value::Str).collect())),
                    Err(e) => bail!("Failed to read lines from '{}': {}", path, e),
                }
            }
            Expr::CopyFile(source_expr, dest_expr) => {
                let source_val = self.eval(source_expr)?;
                let dest_val = self.eval(dest_expr)?;
                let source = match source_val {
                    Value::Str(s) => s,
                    _ => bail!("copy file from: source path must be a string"),
                };
                let dest = match dest_val {
                    Value::Str(s) => s,
                    _ => bail!("copy file to: destination path must be a string"),
                };
                match crate::stdlib::file::copy_file(&source, &dest) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to copy file from '{}' to '{}': {}", source, dest, e),
                }
            }
            Expr::MoveFile(source_expr, dest_expr) => {
                let source_val = self.eval(source_expr)?;
                let dest_val = self.eval(dest_expr)?;
                let source = match source_val {
                    Value::Str(s) => s,
                    _ => bail!("move file from: source path must be a string"),
                };
                let dest = match dest_val {
                    Value::Str(s) => s,
                    _ => bail!("move file to: destination path must be a string"),
                };
                match crate::stdlib::file::move_file(&source, &dest) {
                    Ok(_) => Ok(Value::Null),
                    Err(e) => bail!("Failed to move file from '{}' to '{}': {}", source, dest, e),
                }
            }
            // JSON operations
            Expr::ParseJson(json_str_expr) => {
                let json_str_val = self.eval(json_str_expr)?;
                let json_str = match json_str_val {
                    Value::Str(s) => s,
                    _ => bail!("parse json from: input must be a string"),
                };
                match crate::stdlib::network::parse_json(&json_str) {
                    Ok(json_value) => {
                        // Convert serde_json::Value to our Value
                        self.json_to_value(&json_value)
                    }
                    Err(e) => bail!("Failed to parse JSON: {}", e),
                }
            }
            Expr::ToJson(value_expr) => {
                let value = self.eval(value_expr)?;
                let json_value = self.value_to_json(&value)?;
                match crate::stdlib::network::json_stringify(&json_value) {
                    Ok(json_str) => Ok(Value::Str(json_str)),
                    Err(e) => bail!("Failed to convert to JSON: {}", e),
                }
            }
            Expr::ToJsonPretty(value_expr) => {
                let value = self.eval(value_expr)?;
                let json_value = self.value_to_json(&value)?;
                match crate::stdlib::network::json_stringify_pretty(&json_value) {
                    Ok(json_str) => Ok(Value::Str(json_str)),
                    Err(e) => bail!("Failed to convert to pretty JSON: {}", e),
                }
            }
            Expr::GetPathParam(param_name_expr) => {
                let param_name_val = self.eval(param_name_expr)?;
                let param_name = match param_name_val {
                    Value::Str(s) => s,
                    _ => bail!("get path parameter: parameter name must be a string"),
                };
                
                // Get the request from VM context
                let request_val = self.globals.get("__request")
                    .ok_or_else(|| anyhow!("get path parameter: no request context available"))?;
                    
                match request_val {
                    Value::HttpRequest(ref req) => {
                        req.path_params.get(&param_name)
                            .cloned()
                            .map(Value::Str)
                            .ok_or_else(|| anyhow!("Path parameter '{}' not found", param_name))
                    }
                    _ => bail!("get path parameter: invalid request context"),
                }
            }
            Expr::JsonGet(json_expr, key_expr) => {
                let json_val = self.eval(json_expr)?;
                let key_val = self.eval(key_expr)?;
                let key = match key_val {
                    Value::Str(s) => s,
                    _ => bail!("get from json: key must be a string"),
                };
                match json_val {
                    Value::Dict(ref map) => map
                        .get(&key)
                        .cloned()
                        .ok_or_else(|| anyhow!("Key '{}' not found in JSON object", key)),
                    _ => bail!("get from json: first argument must be a JSON object (dictionary)"),
                }
            }
            Expr::JsonSet(json_expr, key_expr, value_expr) => {
                let json_val = self.eval(json_expr)?;
                let key_val = self.eval(key_expr)?;
                let value_val = self.eval(value_expr)?;
                let key = match key_val {
                    Value::Str(s) => s,
                    _ => bail!("set in json: key must be a string"),
                };
                match json_val {
                    Value::Dict(mut map) => {
                        map.insert(key, value_val);
                        Ok(Value::Dict(map))
                    }
                    _ => bail!("set in json: first argument must be a JSON object (dictionary)"),
                }
            }
            Expr::NewJsonObject => Ok(Value::Dict(HashMap::new())),
            Expr::NewJsonArray => Ok(Value::List(Vec::new())),
            Expr::JsonPush(json_expr, item_expr) => {
                let json_val = self.eval(json_expr)?;
                let item_val = self.eval(item_expr)?;
                match json_val {
                    Value::List(mut vec) => {
                        vec.push(item_val);
                        Ok(Value::List(vec))
                    }
                    _ => bail!("push to json: first argument must be a JSON array (list)"),
                }
            }
            Expr::JsonLength(json_expr) => {
                let json_val = self.eval(json_expr)?;
                match json_val {
                    Value::List(ref vec) => Ok(Value::Num(vec.len() as f64)),
                    Value::Dict(ref map) => Ok(Value::Num(map.len() as f64)),
                    _ => bail!("json length of: argument must be a JSON array or object"),
                }
            }
            Expr::ErrorMessage(err_expr) => {
                let err_val = self.eval(err_expr)?;
                match err_val {
                    Value::Error(e) => Ok(Value::Str(e.message.clone())),
                    _ => bail!("error message of: argument must be an error value"),
                }
            }
            Expr::ErrorType(err_expr) => {
                let err_val = self.eval(err_expr)?;
                match err_val {
                    Value::Error(e) => Ok(Value::Str(e.type_string())),
                    _ => bail!("error type of: argument must be an error value"),
                }
            }
            Expr::NewError {
                error_type,
                message,
            } => {
                let msg_val = self.eval(message)?;
                let msg_str = to_string(&msg_val);
                let kind = ErrorKind::from_string(error_type);
                let error = self.create_error(kind, msg_str);
                Ok(Value::Error(error))
            }
            // Web server operations
            Expr::CreateWebServer(port_expr) => {
                let port_val = self.eval(port_expr)?;
                let port = match port_val {
                    Value::Num(n) if n >= 0.0 && n <= 65535.0 => n as u16,
                    _ => bail!(
                        "create web server on port: port must be a number between 0 and 65535"
                    ),
                };
                let server = crate::stdlib::http::WebServer::new(port);
                Ok(Value::WebServer(std::sync::Arc::new(
                    std::sync::Mutex::new(server),
                )))
            }
            Expr::HtmlResponse(content_expr) => {
                let content_val = self.eval(content_expr)?;
                let content = to_string(&content_val);
                let response = crate::stdlib::http::html_response(content);
                Ok(Value::HttpResponse(response))
            }
            Expr::JsonResponse(data_expr) => {
                let data_val = self.eval(data_expr)?;
                let json_value = self.value_to_json(&data_val)?;
                let response = crate::stdlib::http::json_response(json_value);
                Ok(Value::HttpResponse(response))
            }
            Expr::JsonResponseStatus(data_expr, status_expr) => {
                let data_val = self.eval(data_expr)?;
                let status_val = self.eval(status_expr)?;
                let status = match status_val {
                    Value::Num(n) if n >= 100.0 && n < 600.0 => n as u16,
                    _ => bail!(
                        "json response with status: status must be a number between 100 and 599"
                    ),
                };
                let json_value = self.value_to_json(&data_val)?;
                let response = crate::stdlib::http::json_response_with_status(json_value, status);
                Ok(Value::HttpResponse(response))
            }
            Expr::RenderTemplate(template_expr, data_expr) => {
                let template_val = self.eval(template_expr)?;
                let data_val = self.eval(data_expr)?;
                let template = match template_val {
                    Value::Str(s) => s,
                    _ => bail!("render template: template must be a string"),
                };
                let json_data = self.value_to_json(&data_val)?;
                match crate::stdlib::template::render_full(&template, &json_data) {
                    Ok(rendered) => Ok(Value::Str(rendered)),
                    Err(e) => bail!("Failed to render template: {}", e),
                }
            }
            Expr::ErrorResponse(status_expr, message_expr) => {
                let status_val = self.eval(status_expr)?;
                let message_val = self.eval(message_expr)?;
                let status = match status_val {
                    Value::Num(n) if n >= 100.0 && n < 600.0 => n as u16,
                    _ => bail!("error response: status must be a number between 100 and 599"),
                };
                let message = to_string(&message_val);
                let response = crate::stdlib::http::error_response(status, message);
                Ok(Value::HttpResponse(response))
            }
            Expr::RequestField(request_expr, field_expr) => {
                let request_val = self.eval(request_expr)?;
                let field_val = self.eval(field_expr)?;
                let field = match field_val {
                    Value::Str(s) => s,
                    _ => bail!("request field: field name must be a string"),
                };
                // request should be a dictionary (converted from HttpRequest)
                match request_val {
                    Value::Dict(ref map) => map
                        .get(&field)
                        .cloned()
                        .ok_or_else(|| anyhow!("Request field '{}' not found", field)),
                    _ => bail!("request field: request must be a dictionary"),
                }
            }
        }
    }

    // Helper to convert serde_json::Value to our Value
    #[allow(clippy::only_used_in_recursion)]
    fn json_to_value(&self, json: &serde_json::Value) -> Result<Value> {
        use serde_json::Value as JsonValue;
        match json {
            JsonValue::Null => Ok(Value::Null),
            JsonValue::Bool(b) => Ok(Value::Bool(*b)),
            JsonValue::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(Value::Num(f))
                } else {
                    bail!("JSON number out of range")
                }
            }
            JsonValue::String(s) => Ok(Value::Str(s.clone())),
            JsonValue::Array(arr) => {
                let mut values = Vec::new();
                for item in arr {
                    values.push(self.json_to_value(item)?);
                }
                Ok(Value::List(values))
            }
            JsonValue::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), self.json_to_value(v)?);
                }
                Ok(Value::Dict(map))
            }
        }
    }

    // Helper to convert our Value to serde_json::Value
    #[allow(clippy::only_used_in_recursion)]
    fn value_to_json(&self, value: &Value) -> Result<serde_json::Value> {
        use serde_json::Value as JsonValue;
        match value {
            Value::Null => Ok(JsonValue::Null),
            Value::Bool(b) => Ok(JsonValue::Bool(*b)),
            Value::Num(n) => Ok(serde_json::Number::from_f64(*n)
                .map(JsonValue::Number)
                .unwrap_or(JsonValue::Null)),
            Value::Str(s) => Ok(JsonValue::String(s.clone())),
            Value::List(vec) => {
                let mut arr = Vec::new();
                for item in vec {
                    arr.push(self.value_to_json(item)?);
                }
                Ok(JsonValue::Array(arr))
            }
            Value::Dict(map) => {
                let mut obj = serde_json::Map::new();
                for (k, v) in map {
                    obj.insert(k.clone(), self.value_to_json(v)?);
                }
                Ok(JsonValue::Object(obj))
            }
            Value::Func(_) => bail!("Cannot convert function to JSON"),
            Value::Error(e) => bail!("Cannot convert error to JSON: {}", e.message),
            Value::WebServer(_) => bail!("Cannot convert web server to JSON"),
            Value::HttpRequest(_) => bail!("Cannot convert HTTP request to JSON"),
            Value::HttpResponse(_) => bail!("Cannot convert HTTP response to JSON"),
            Value::LiveReloadTracker(_) => bail!("Cannot convert LiveReloadTracker to JSON"),
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
            Value::Error(_) => Ok(true), // Errors are truthy (presence indicates something went wrong)
            Value::WebServer(_) => Ok(true), // Web servers are truthy
            Value::HttpRequest(_) => Ok(true), // HTTP requests are truthy
            Value::HttpResponse(_) => Ok(true), // HTTP responses are truthy
            Value::LiveReloadTracker(_) => Ok(true), // LiveReloadTracker is truthy
        }
    }

    /// Push a new frame onto the call stack for error tracking
    #[allow(dead_code)]
    fn push_call_frame(&mut self, function_name: impl Into<String>) {
        self.call_stack.push(CallFrame {
            function_name: function_name.into(),
            file: self.current_file.clone(),
            line: 0, // Line tracking can be added later if needed
        });
    }

    /// Pop the topmost call frame when returning from a function
    #[allow(dead_code)]
    fn pop_call_frame(&mut self) {
        self.call_stack.pop();
    }

    /// Build a stack trace from the current call stack
    fn build_stack_trace(&self) -> Vec<StackFrame> {
        self.call_stack
            .iter()
            .map(|frame| {
                StackFrame::new(frame.function_name.clone(), frame.file.clone(), frame.line)
            })
            .collect()
    }

    /// Create a PohError with the current stack trace
    fn create_error(&self, kind: ErrorKind, message: impl Into<String>) -> PohError {
        PohError::with_stack_trace(kind, message, self.build_stack_trace())
    }

    fn call_function(&self, name: &str, args: &[Value]) -> Result<Value> {
        if name.contains("::") {
            return self.call_qualified_function(name, args);
        }
        // Built-ins
        match name {
            "now" if args.is_empty() => {
                return Ok(Value::Str(iso_now()));
            }
            "range" => {
                return builtin_range(args);
            }
            "join" => {
                return builtin_join(args);
            }
            "split" => {
                return builtin_split(args);
            }
            "length" => {
                return builtin_length(args);
            }
            "len" => {
                return builtin_length(args);
            }
            // Mathematical functions
            "sum" => {
                return builtin_sum(args);
            }
            "min" => {
                return builtin_min(args);
            }
            "max" => {
                return builtin_max(args);
            }
            "abs" => {
                return builtin_abs(args);
            }
            "round" => {
                return builtin_round(args);
            }
            "floor" => {
                return builtin_floor(args);
            }
            "ceil" => {
                return builtin_ceil(args);
            }
            // String functions
            "uppercase" => {
                return builtin_uppercase(args);
            }
            "lowercase" => {
                return builtin_lowercase(args);
            }
            "trim" => {
                return builtin_trim(args);
            }
            // Collection functions
            "first" => {
                return builtin_first(args);
            }
            "last" => {
                return builtin_last(args);
            }
            "reverse" => {
                return builtin_reverse(args);
            }
            _ => {}
        }
        // User-defined
        if let Some(Value::Func(f)) = self.globals.get(name) {
            return self.call_func_value(f, args);
        }
        if let Some(Value::Func(f)) = self.resolve_value(name) {
            return self.call_func_value(&f, args);
        }
        Err(anyhow!("Error: Function '{}' is not defined", name))
    }

    fn call_func_value(&self, f: &Func, args: &[Value]) -> Result<Value> {
        // Arity with defaults
        let required = f.params.iter().filter(|p| p.default.is_none()).count();
        if args.len() < required || args.len() > f.params.len() {
            return Err(anyhow!(
                "Function '{}' expects {}..{} args but got {}",
                f.name,
                required,
                f.params.len(),
                args.len()
            ));
        }
        // Locals map
        let mut locals: HashMap<String, Value> = HashMap::new();
        for (i, p) in f.params.iter().enumerate() {
            if i < args.len() {
                locals.insert(p.name.clone(), args[i].clone());
            } else if let Some(def) = &p.default {
                let v = self.eval_in_scope_with_capture(def, &locals, &f.captured)?; // evaluate default at call-time
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

    fn execute_function_block(
        &self,
        body: Program,
        locals: &HashMap<String, Value>,
        captured: &[HashMap<String, Value>],
    ) -> Result<Value> {
        // Create a scope stack: locals (top), captured (next), globals (fallback)
        // Execute statements until Return encountered; return its value or 'nothing' (empty string) if none.
        let mut frame = Frame {
            locals: locals.clone(),
            captured: captured.to_owned(),
        };
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
                    let input = core_io::ask("");
                    let value = if let Ok(n) = input.parse::<f64>() {
                        Value::Num(n)
                    } else {
                        Value::Str(input)
                    };
                    frame.locals.insert(var_name.clone(), value);
                }
                Stmt::Set { name, value } => {
                    if let Ok(v) = self.eval_in_frame(value, frame) {
                        frame.locals.insert(name.clone(), v);
                    }
                }
                Stmt::IfInline {
                    cond,
                    then_write,
                    otherwise_write,
                } => {
                    if let Ok(c) =
                        self.truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0)))
                    {
                        if c {
                            if let Ok(v) = self.eval_in_frame(then_write, frame) {
                                core_io::write(&to_string(&v));
                            }
                        } else if let Some(e) = otherwise_write {
                            if let Ok(v) = self.eval_in_frame(e, frame) {
                                core_io::write(&to_string(&v));
                            }
                        }
                    }
                }
                Stmt::IfBlock {
                    cond,
                    then_body,
                    otherwise_body,
                } => {
                    if let Ok(c) =
                        self.truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0)))
                    {
                        let cf = if c {
                            self.exec_block_with_frame(then_body, frame)
                        } else if let Some(eb) = otherwise_body {
                            self.exec_block_with_frame(eb, frame)
                        } else {
                            ControlFlow::Continue
                        };
                        if let ControlFlow::Return(_) = cf {
                            return cf;
                        }
                    }
                }
                Stmt::WhileBlock { cond, body } => {
                    let mut guard = 0usize;
                    while self
                        .truthy(&self.eval_in_frame(cond, frame).unwrap_or(Value::Num(0.0)))
                        .unwrap_or(false)
                    {
                        let cf = self.exec_block_with_frame(body, frame);
                        if let ControlFlow::Return(_) = cf {
                            return cf;
                        }
                        guard += 1;
                        if guard > 1_000_000 {
                            break;
                        }
                    }
                }
                Stmt::RepeatBlock { count, body } => {
                    let n = match self.eval_in_frame(count, frame).ok() {
                        Some(Value::Num(x)) => x.max(0.0) as i64,
                        _ => 0,
                    };
                    for _ in 0..n {
                        let cf = self.exec_block_with_frame(body, frame);
                        if let ControlFlow::Return(_) = cf {
                            return cf;
                        }
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
                    let f = Func {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        captured,
                    };
                    frame.locals.insert(name.clone(), Value::Func(f));
                }
                Stmt::FuncBlock {
                    name,
                    params,
                    body: b,
                } => {
                    let mut captured = frame.captured.clone();
                    captured.insert(0, frame.locals.clone());
                    let f = Func {
                        name: name.clone(),
                        params: params.clone(),
                        body: Expr::Ident(format!("__fn_body__{}", name)),
                        captured,
                    };
                    FN_BLOCKS.with(|m| {
                        m.borrow_mut()
                            .insert(format!("__fn_body__{}", name), b.clone());
                    });
                    frame.locals.insert(name.clone(), Value::Func(f));
                }
                Stmt::Use { name, args } => {
                    let argv = args
                        .iter()
                        .filter_map(|e| self.eval_in_frame(e, frame).ok())
                        .collect::<Vec<_>>();
                    // Resolve function from current frame first, then captured, then globals
                    if let Some(Value::Func(f)) = frame.locals.get(name) {
                        if let Ok(v) = self.call_func_value(f, &argv) {
                            core_io::write(&to_string(&v));
                        }
                    } else if let Some(Value::Func(f)) = self.globals.get(name) {
                        if let Ok(v) = self.call_func_value(f, &argv) {
                            core_io::write(&to_string(&v));
                        }
                    } else {
                        // search in captured chain
                        let mut done = false;
                        for env in frame.captured.iter() {
                            if let Some(Value::Func(f)) = env.get(name) {
                                if let Ok(v) = self.call_func_value(f, &argv) {
                                    core_io::write(&to_string(&v));
                                }
                                done = true;
                                break;
                            }
                        }
                        if !done {
                            if let Ok(v) = self.call_function(name, &argv) {
                                core_io::write(&to_string(&v));
                            }
                        }
                    }
                }
                Stmt::Return(expr) => {
                    let v = match expr {
                        Some(e) => self.eval_in_frame(e, frame).ok(),
                        None => None,
                    };
                    return ControlFlow::Return(v);
                }
                Stmt::TryCatch { .. } => {
                    // TODO: Implement proper try/catch in function context
                    // For now, just continue execution
                    // Full implementation requires propagating errors through ControlFlow
                    return ControlFlow::Continue;
                }
                Stmt::Throw(expr) => {
                    // Evaluate throw expression in function context
                    if let Ok(val) = self.eval_in_frame(expr, frame) {
                        match val {
                            Value::Error(e) => {
                                eprintln!("{}", e.format_with_trace());
                            }
                            _ => {
                                let msg = to_string(&val);
                                let error = self.create_error(ErrorKind::RuntimeError, msg);
                                eprintln!("{}", error.format_with_trace());
                            }
                        }
                    }
                    return ControlFlow::Return(None);
                }
                Stmt::AddRoute { .. } => {
                    // Web server routes cannot be defined inside functions
                    // They must be defined at module level
                    eprintln!("Warning: AddRoute statement inside function is not supported");
                    return ControlFlow::Continue;
                }
                Stmt::StartServer => {
                    // Start server cannot be called inside functions
                    eprintln!("Warning: StartServer statement inside function is not supported");
                    return ControlFlow::Continue;
                }
            }
        }
        ControlFlow::Continue
    }

    fn eval_in_frame(&self, e: &Expr, frame: &Frame) -> Result<Value> {
        match e {
            Expr::Ident(n) => {
                if let Some(v) = frame.locals.get(n) {
                    return Ok(v.clone());
                }
                for env in frame.captured.iter() {
                    if let Some(v) = env.get(n) {
                        return Ok(v.clone());
                    }
                }
                if let Some(v) = self.resolve_value(n) {
                    return Ok(v);
                }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_frame(a, frame)?;
                let sb = self.eval_in_frame(b, frame)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y)))),
                }
            }
            Expr::Minus(a, b) => {
                let sa = self.eval_in_frame(a, frame)?;
                let sb = self.eval_in_frame(b, frame)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na - nb)),
                    _ => Err(anyhow!("Cannot subtract non-numeric values")),
                }
            }
            Expr::Times(a, b) => {
                let sa = self.eval_in_frame(a, frame)?;
                let sb = self.eval_in_frame(b, frame)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na * nb)),
                    _ => Err(anyhow!("Cannot multiply non-numeric values")),
                }
            }
            Expr::DividedBy(a, b) => {
                let sa = self.eval_in_frame(a, frame)?;
                let sb = self.eval_in_frame(b, frame)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => {
                        if nb == 0.0 {
                            Err(self.error_with_location("Division by zero"))
                        } else {
                            Ok(Value::Num(na / nb))
                        }
                    }
                    _ => Err(anyhow!("Cannot divide non-numeric values")),
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                if !la {
                    return Ok(Value::Num(0.0));
                }
                let lb = self.truthy(&self.eval_in_frame(b, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                if la {
                    return Ok(Value::Num(1.0));
                }
                let lb = self.truthy(&self.eval_in_frame(b, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval_in_frame(a, frame).unwrap_or(Value::Num(0.0)))?;
                Ok(if !la {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_frame(l, frame)?;
                let rv = self.eval_in_frame(r, frame)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
                        let rn = match rv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
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
                Ok(if res {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Call { name, args } => {
                let argv = args
                    .iter()
                    .map(|e| self.eval_in_frame(e, frame))
                    .collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() {
                    return Ok(Value::Str(iso_now()));
                }
                // Try resolve function in local frame hierarchy first
                if let Some(Value::Func(f)) = frame.locals.get(name) {
                    return self.call_func_value(f, &argv);
                }
                for env in frame.captured.iter() {
                    if let Some(Value::Func(f)) = env.get(name) {
                        return self.call_func_value(f, &argv);
                    }
                }
                if let Some(Value::Func(f)) = self.globals.get(name) {
                    return self.call_func_value(f, &argv);
                }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items {
                    out.push(self.eval_in_frame(it, frame)?);
                }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs {
                    map.insert(k.clone(), self.eval_in_frame(ve, frame)?);
                }
                Ok(Value::Dict(map))
            }
            Expr::Index(base, index) => {
                let base_val = self.eval_in_frame(base, frame)?;
                let index_val = self.eval_in_frame(index, frame)?;

                match (&base_val, &index_val) {
                    (Value::List(items), Value::Num(n)) => {
                        let idx = *n as i32;
                        let len = items.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "List index out of range: {} (list length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(items[actual_idx as usize].clone())
                    }
                    (Value::Dict(map), Value::Str(key)) => map
                        .get(key)
                        .cloned()
                        .ok_or_else(|| anyhow!("Key not found in dictionary: \"{}\"", key)),
                    (Value::Str(s), Value::Num(n)) => {
                        let idx = *n as i32;
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "String index out of range: {} (string length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(Value::Str(chars[actual_idx as usize].to_string()))
                    }
                    _ => Err(anyhow!("Cannot index {:?} with {:?}", base_val, index_val)),
                }
            }
            // Phrasal built-in expressions
            Expr::TotalOf(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_sum(&[val])
            }
            Expr::SmallestIn(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_min(&[val])
            }
            Expr::LargestIn(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_max(&[val])
            }
            Expr::AbsoluteValueOf(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_abs(&[val])
            }
            Expr::Round(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_round(&[val])
            }
            Expr::RoundDown(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_floor(&[val])
            }
            Expr::RoundUp(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_ceil(&[val])
            }
            Expr::MakeUppercase(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_uppercase(&[val])
            }
            Expr::MakeLowercase(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_lowercase(&[val])
            }
            Expr::TrimSpaces(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_trim(&[val])
            }
            Expr::FirstIn(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_first(&[val])
            }
            Expr::LastIn(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_last(&[val])
            }
            Expr::ReverseOf(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_reverse(&[val])
            }
            Expr::CountOf(expr) => {
                let val = self.eval_in_frame(expr, frame)?;
                builtin_length(&[val])
            }
            Expr::JoinWith(a, b) => {
                let va = self.eval_in_frame(a, frame)?;
                let vb = self.eval_in_frame(b, frame)?;
                builtin_join(&[va, vb])
            }
            Expr::SplitBy(a, b) => {
                let va = self.eval_in_frame(a, frame)?;
                let vb = self.eval_in_frame(b, frame)?;
                builtin_split(&[va, vb])
            }
            Expr::Contains(item, collection) => {
                let item_val = self.eval_in_frame(item, frame)?;
                let coll_val = self.eval_in_frame(collection, frame)?;
                builtin_contains(&[item_val, coll_val])
            }
            Expr::Remove(item, list) => {
                let item_val = self.eval_in_frame(item, frame)?;
                let list_val = self.eval_in_frame(list, frame)?;
                builtin_remove(&[item_val, list_val])
            }
            Expr::Append(item, list) => {
                let item_val = self.eval_in_frame(item, frame)?;
                let list_val = self.eval_in_frame(list, frame)?;
                builtin_append(&[item_val, list_val])
            }
            Expr::InsertAt(item, index, list) => {
                let item_val = self.eval_in_frame(item, frame)?;
                let index_val = self.eval_in_frame(index, frame)?;
                let list_val = self.eval_in_frame(list, frame)?;
                builtin_insert_at(&[item_val, index_val, list_val])
            }
            // File I/O operations - delegate to eval since they don't use local scope
            Expr::ReadFile(_)
            | Expr::WriteFile(_, _)
            | Expr::AppendFile(_, _)
            | Expr::FileExists(_)
            | Expr::DeleteFile(_)
            | Expr::CreateDir(_)
            | Expr::ListDir(_)
            | Expr::ReadLines(_)
            | Expr::CopyFile(_, _)
            | Expr::MoveFile(_, _)
            // JSON operations - also delegate to eval
            | Expr::ParseJson(_)
            | Expr::ToJson(_)
            | Expr::ToJsonPretty(_)
            | Expr::GetPathParam(_)
            | Expr::JsonGet(_, _)
            | Expr::JsonSet(_, _, _)
            | Expr::NewJsonObject
            | Expr::NewJsonArray
            | Expr::JsonPush(_, _)
            | Expr::JsonLength(_)
            // Error operations - also delegate to eval
            | Expr::ErrorMessage(_)
            | Expr::ErrorType(_)
            | Expr::NewError { .. }
            // Web server operations - also delegate to eval
            | Expr::CreateWebServer(_)
            | Expr::HtmlResponse(_)
            | Expr::JsonResponse(_)
            | Expr::JsonResponseStatus(_, _)
            | Expr::RenderTemplate(_, _)
            | Expr::ErrorResponse(_, _)
            | Expr::RequestField(_, _) => self.eval(e),
        }
    }

    fn eval_in_scope(&self, e: &Expr, locals: &HashMap<String, Value>) -> Result<Value> {
        match e {
            Expr::Ident(n) => {
                if let Some(v) = locals.get(n) {
                    return Ok(v.clone());
                }
                if let Some(v) = self.resolve_value(n) {
                    return Ok(v);
                }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_scope(a, locals)?;
                let sb = self.eval_in_scope(b, locals)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y)))),
                }
            }
            Expr::Minus(a, b) => {
                let sa = self.eval_in_scope(a, locals)?;
                let sb = self.eval_in_scope(b, locals)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na - nb)),
                    _ => Err(anyhow!("Cannot subtract non-numeric values")),
                }
            }
            Expr::Times(a, b) => {
                let sa = self.eval_in_scope(a, locals)?;
                let sb = self.eval_in_scope(b, locals)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na * nb)),
                    _ => Err(anyhow!("Cannot multiply non-numeric values")),
                }
            }
            Expr::DividedBy(a, b) => {
                let sa = self.eval_in_scope(a, locals)?;
                let sb = self.eval_in_scope(b, locals)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => {
                        if nb == 0.0 {
                            Err(self.error_with_location("Division by zero"))
                        } else {
                            Ok(Value::Num(na / nb))
                        }
                    }
                    _ => Err(anyhow!("Cannot divide non-numeric values")),
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                if !la {
                    return Ok(Value::Num(0.0));
                }
                let lb = self.truthy(&self.eval_in_scope(b, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                if la {
                    return Ok(Value::Num(1.0));
                }
                let lb = self.truthy(&self.eval_in_scope(b, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(&self.eval_in_scope(a, locals).unwrap_or(Value::Num(0.0)))?;
                Ok(if !la {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_scope(l, locals)?;
                let rv = self.eval_in_scope(r, locals)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
                        let rn = match rv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
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
                Ok(if res {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Call { name, args } => {
                let argv = args
                    .iter()
                    .map(|e| self.eval_in_scope(e, locals))
                    .collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() {
                    return Ok(Value::Str(iso_now()));
                }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items {
                    out.push(self.eval_in_scope(it, locals)?);
                }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs {
                    map.insert(k.clone(), self.eval_in_scope(ve, locals)?);
                }
                Ok(Value::Dict(map))
            }
            Expr::Index(base, index) => {
                let base_val = self.eval_in_scope(base, locals)?;
                let index_val = self.eval_in_scope(index, locals)?;

                match (&base_val, &index_val) {
                    (Value::List(items), Value::Num(n)) => {
                        let idx = *n as i32;
                        let len = items.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "List index out of range: {} (list length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(items[actual_idx as usize].clone())
                    }
                    (Value::Dict(map), Value::Str(key)) => map
                        .get(key)
                        .cloned()
                        .ok_or_else(|| anyhow!("Key not found in dictionary: \"{}\"", key)),
                    (Value::Str(s), Value::Num(n)) => {
                        let idx = *n as i32;
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "String index out of range: {} (string length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(Value::Str(chars[actual_idx as usize].to_string()))
                    }
                    _ => Err(anyhow!("Cannot index {:?} with {:?}", base_val, index_val)),
                }
            }
            // Phrasal built-in expressions
            Expr::TotalOf(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_sum(&[val])
            }
            Expr::SmallestIn(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_min(&[val])
            }
            Expr::LargestIn(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_max(&[val])
            }
            Expr::AbsoluteValueOf(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_abs(&[val])
            }
            Expr::Round(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_round(&[val])
            }
            Expr::RoundDown(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_floor(&[val])
            }
            Expr::RoundUp(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_ceil(&[val])
            }
            Expr::MakeUppercase(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_uppercase(&[val])
            }
            Expr::MakeLowercase(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_lowercase(&[val])
            }
            Expr::TrimSpaces(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_trim(&[val])
            }
            Expr::FirstIn(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_first(&[val])
            }
            Expr::LastIn(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_last(&[val])
            }
            Expr::ReverseOf(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_reverse(&[val])
            }
            Expr::CountOf(expr) => {
                let val = self.eval_in_scope(expr, locals)?;
                builtin_length(&[val])
            }
            Expr::JoinWith(a, b) => {
                let va = self.eval_in_scope(a, locals)?;
                let vb = self.eval_in_scope(b, locals)?;
                builtin_join(&[va, vb])
            }
            Expr::SplitBy(a, b) => {
                let va = self.eval_in_scope(a, locals)?;
                let vb = self.eval_in_scope(b, locals)?;
                builtin_split(&[va, vb])
            }
            Expr::Contains(item, collection) => {
                let item_val = self.eval_in_scope(item, locals)?;
                let coll_val = self.eval_in_scope(collection, locals)?;
                builtin_contains(&[item_val, coll_val])
            }
            Expr::Remove(item, list) => {
                let item_val = self.eval_in_scope(item, locals)?;
                let list_val = self.eval_in_scope(list, locals)?;
                builtin_remove(&[item_val, list_val])
            }
            Expr::Append(item, list) => {
                let item_val = self.eval_in_scope(item, locals)?;
                let list_val = self.eval_in_scope(list, locals)?;
                builtin_append(&[item_val, list_val])
            }
            Expr::InsertAt(item, index, list) => {
                let item_val = self.eval_in_scope(item, locals)?;
                let index_val = self.eval_in_scope(index, locals)?;
                let list_val = self.eval_in_scope(list, locals)?;
                builtin_insert_at(&[item_val, index_val, list_val])
            }
            // File I/O operations - delegate to eval since they don't use local scope
            Expr::ReadFile(_)
            | Expr::WriteFile(_, _)
            | Expr::AppendFile(_, _)
            | Expr::FileExists(_)
            | Expr::DeleteFile(_)
            | Expr::CreateDir(_)
            | Expr::ListDir(_)
            | Expr::ReadLines(_)
            | Expr::CopyFile(_, _)
            | Expr::MoveFile(_, _)
            // JSON operations - also delegate to eval
            | Expr::ParseJson(_)
            | Expr::ToJson(_)
            | Expr::ToJsonPretty(_)
            | Expr::GetPathParam(_)
            | Expr::JsonGet(_, _)
            | Expr::JsonSet(_, _, _)
            | Expr::NewJsonObject
            | Expr::NewJsonArray
            | Expr::JsonPush(_, _)
            | Expr::JsonLength(_)
            // Error operations - also delegate to eval
            | Expr::ErrorMessage(_)
            | Expr::ErrorType(_)
            | Expr::NewError { .. }
            // Web server operations - also delegate to eval
            | Expr::CreateWebServer(_)
            | Expr::HtmlResponse(_)
            | Expr::JsonResponse(_)
            | Expr::JsonResponseStatus(_, _)
            | Expr::RenderTemplate(_, _)
            | Expr::ErrorResponse(_, _)
            | Expr::RequestField(_, _) => self.eval(e),
        }
    }

    fn eval_in_scope_with_capture(
        &self,
        e: &Expr,
        locals: &HashMap<String, Value>,
        captured: &Vec<HashMap<String, Value>>,
    ) -> Result<Value> {
        if captured.is_empty() {
            return self.eval_in_scope(e, locals);
        }
        match e {
            Expr::Ident(n) => {
                if let Some(v) = locals.get(n) {
                    return Ok(v.clone());
                }
                for env in captured.iter() {
                    if let Some(v) = env.get(n) {
                        return Ok(v.clone());
                    }
                }
                if let Some(v) = self.resolve_value(n) {
                    return Ok(v);
                }
                Ok(Value::Str(format!("<{}>", n)))
            }
            Expr::Str(_) | Expr::Num(_) | Expr::Bool(_) | Expr::Null => self.eval(e),
            Expr::Plus(a, b) => {
                let sa = self.eval_in_scope_with_capture(a, locals, captured)?;
                let sb = self.eval_in_scope_with_capture(b, locals, captured)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na + nb)),
                    (x, y) => Ok(Value::Str(format!("{}{}", to_string(&x), to_string(&y)))),
                }
            }
            Expr::Minus(a, b) => {
                let sa = self.eval_in_scope_with_capture(a, locals, captured)?;
                let sb = self.eval_in_scope_with_capture(b, locals, captured)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na - nb)),
                    _ => Err(anyhow!("Cannot subtract non-numeric values")),
                }
            }
            Expr::Times(a, b) => {
                let sa = self.eval_in_scope_with_capture(a, locals, captured)?;
                let sb = self.eval_in_scope_with_capture(b, locals, captured)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => Ok(Value::Num(na * nb)),
                    _ => Err(anyhow!("Cannot multiply non-numeric values")),
                }
            }
            Expr::DividedBy(a, b) => {
                let sa = self.eval_in_scope_with_capture(a, locals, captured)?;
                let sb = self.eval_in_scope_with_capture(b, locals, captured)?;
                match (sa, sb) {
                    (Value::Num(na), Value::Num(nb)) => {
                        if nb == 0.0 {
                            Err(self.error_with_location("Division by zero"))
                        } else {
                            Ok(Value::Num(na / nb))
                        }
                    }
                    _ => Err(anyhow!("Cannot divide non-numeric values")),
                }
            }
            Expr::And(a, b) => {
                let la = self.truthy(
                    &self
                        .eval_in_scope_with_capture(a, locals, captured)
                        .unwrap_or(Value::Num(0.0)),
                )?;
                if !la {
                    return Ok(Value::Num(0.0));
                }
                let lb = self.truthy(
                    &self
                        .eval_in_scope_with_capture(b, locals, captured)
                        .unwrap_or(Value::Num(0.0)),
                )?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Or(a, b) => {
                let la = self.truthy(
                    &self
                        .eval_in_scope_with_capture(a, locals, captured)
                        .unwrap_or(Value::Num(0.0)),
                )?;
                if la {
                    return Ok(Value::Num(1.0));
                }
                let lb = self.truthy(
                    &self
                        .eval_in_scope_with_capture(b, locals, captured)
                        .unwrap_or(Value::Num(0.0)),
                )?;
                Ok(if lb { Value::Num(1.0) } else { Value::Num(0.0) })
            }
            Expr::Not(a) => {
                let la = self.truthy(
                    &self
                        .eval_in_scope_with_capture(a, locals, captured)
                        .unwrap_or(Value::Num(0.0)),
                )?;
                Ok(if !la {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Cmp(op, l, r) => {
                let lv = self.eval_in_scope_with_capture(l, locals, captured)?;
                let rv = self.eval_in_scope_with_capture(r, locals, captured)?;
                let res = match op {
                    CmpOp::Eq => to_string(&lv) == to_string(&rv),
                    CmpOp::Ne => to_string(&lv) != to_string(&rv),
                    CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                        let ln = match lv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
                        let rn = match rv {
                            Value::Num(n) => Some(n),
                            _ => None,
                        };
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
                Ok(if res {
                    Value::Num(1.0)
                } else {
                    Value::Num(0.0)
                })
            }
            Expr::Call { name, args } => {
                let argv = args
                    .iter()
                    .map(|e| self.eval_in_scope_with_capture(e, locals, captured))
                    .collect::<Result<Vec<_>>>()?;
                if name == "now" && argv.is_empty() {
                    return Ok(Value::Str(iso_now()));
                }
                self.call_function(name, &argv)
            }
            Expr::ListLit(items) => {
                let mut out = Vec::new();
                for it in items {
                    out.push(self.eval_in_scope_with_capture(it, locals, captured)?);
                }
                Ok(Value::List(out))
            }
            Expr::DictLit(pairs) => {
                let mut map = HashMap::new();
                for (k, ve) in pairs {
                    map.insert(
                        k.clone(),
                        self.eval_in_scope_with_capture(ve, locals, captured)?,
                    );
                }
                Ok(Value::Dict(map))
            }
            Expr::Index(base, index) => {
                let base_val = self.eval_in_scope_with_capture(base, locals, captured)?;
                let index_val = self.eval_in_scope_with_capture(index, locals, captured)?;

                match (&base_val, &index_val) {
                    (Value::List(items), Value::Num(n)) => {
                        let idx = *n as i32;
                        let len = items.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "List index out of range: {} (list length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(items[actual_idx as usize].clone())
                    }
                    (Value::Dict(map), Value::Str(key)) => map
                        .get(key)
                        .cloned()
                        .ok_or_else(|| anyhow!("Key not found in dictionary: \"{}\"", key)),
                    (Value::Str(s), Value::Num(n)) => {
                        let idx = *n as i32;
                        let chars: Vec<char> = s.chars().collect();
                        let len = chars.len() as i32;
                        let actual_idx = if idx < 0 { len + idx } else { idx };

                        if actual_idx < 0 || actual_idx >= len {
                            return Err(anyhow!(
                                "String index out of range: {} (string length: {})",
                                idx,
                                len
                            ));
                        }
                        Ok(Value::Str(chars[actual_idx as usize].to_string()))
                    }
                    _ => Err(anyhow!("Cannot index {:?} with {:?}", base_val, index_val)),
                }
            }
            // Phrasal built-in expressions
            Expr::TotalOf(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_sum(&[val])
            }
            Expr::SmallestIn(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_min(&[val])
            }
            Expr::LargestIn(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_max(&[val])
            }
            Expr::AbsoluteValueOf(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_abs(&[val])
            }
            Expr::Round(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_round(&[val])
            }
            Expr::RoundDown(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_floor(&[val])
            }
            Expr::RoundUp(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_ceil(&[val])
            }
            Expr::MakeUppercase(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_uppercase(&[val])
            }
            Expr::MakeLowercase(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_lowercase(&[val])
            }
            Expr::TrimSpaces(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_trim(&[val])
            }
            Expr::FirstIn(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_first(&[val])
            }
            Expr::LastIn(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_last(&[val])
            }
            Expr::ReverseOf(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_reverse(&[val])
            }
            Expr::CountOf(expr) => {
                let val = self.eval_in_scope_with_capture(expr, locals, captured)?;
                builtin_length(&[val])
            }
            Expr::JoinWith(a, b) => {
                let va = self.eval_in_scope_with_capture(a, locals, captured)?;
                let vb = self.eval_in_scope_with_capture(b, locals, captured)?;
                builtin_join(&[va, vb])
            }
            Expr::SplitBy(a, b) => {
                let va = self.eval_in_scope_with_capture(a, locals, captured)?;
                let vb = self.eval_in_scope_with_capture(b, locals, captured)?;
                builtin_split(&[va, vb])
            }
            Expr::Contains(item, collection) => {
                let item_val = self.eval_in_scope_with_capture(item, locals, captured)?;
                let coll_val = self.eval_in_scope_with_capture(collection, locals, captured)?;
                builtin_contains(&[item_val, coll_val])
            }
            Expr::Remove(item, list) => {
                let item_val = self.eval_in_scope_with_capture(item, locals, captured)?;
                let list_val = self.eval_in_scope_with_capture(list, locals, captured)?;
                builtin_remove(&[item_val, list_val])
            }
            Expr::Append(item, list) => {
                let item_val = self.eval_in_scope_with_capture(item, locals, captured)?;
                let list_val = self.eval_in_scope_with_capture(list, locals, captured)?;
                builtin_append(&[item_val, list_val])
            }
            Expr::InsertAt(item, index, list) => {
                let item_val = self.eval_in_scope_with_capture(item, locals, captured)?;
                let index_val = self.eval_in_scope_with_capture(index, locals, captured)?;
                let list_val = self.eval_in_scope_with_capture(list, locals, captured)?;
                builtin_insert_at(&[item_val, index_val, list_val])
            }
            // File I/O operations - delegate to eval since they don't use local scope
            Expr::ReadFile(_)
            | Expr::WriteFile(_, _)
            | Expr::AppendFile(_, _)
            | Expr::FileExists(_)
            | Expr::DeleteFile(_)
            | Expr::CreateDir(_)
            | Expr::ListDir(_)
            | Expr::ReadLines(_)
            | Expr::CopyFile(_, _)
            | Expr::MoveFile(_, _)
            // JSON operations - also delegate to eval
            | Expr::ParseJson(_)
            | Expr::ToJson(_)
            | Expr::ToJsonPretty(_)
            | Expr::GetPathParam(_)
            | Expr::JsonGet(_, _)
            | Expr::JsonSet(_, _, _)
            | Expr::NewJsonObject
            | Expr::NewJsonArray
            | Expr::JsonPush(_, _)
            | Expr::JsonLength(_)
            // Error operations - also delegate to eval
            | Expr::ErrorMessage(_)
            | Expr::ErrorType(_)
            | Expr::NewError { .. }
            // Web server operations - also delegate to eval
            | Expr::CreateWebServer(_)
            | Expr::HtmlResponse(_)
            | Expr::JsonResponse(_)
            | Expr::JsonResponseStatus(_, _)
            | Expr::RenderTemplate(_, _)
            | Expr::ErrorResponse(_, _)
            | Expr::RequestField(_, _) => self.eval(e),
        }
    }
}

pub fn compile(prog: &Program) -> Vec<u8> {
    let mut out: Vec<String> = Vec::new();
    for stmt in prog {
        match stmt {
            Stmt::Write(e) => {
                compile_expr(e, &mut out);
                out.push(Instruction::WriteTop.encode());
            }
            Stmt::AskFor { var_name } => {
                out.push(Instruction::AskVar(var_name.clone()).encode());
            }
            Stmt::Set { name, value } => {
                compile_expr(value, &mut out);
                out.push(Instruction::StoreVar(name.clone()).encode());
            }
            Stmt::IfBlock {
                cond,
                then_body,
                otherwise_body,
            } => {
                // cond
                compile_expr(cond, &mut out);
                // placeholder for jump-if-false to else/end
                let jmpf_index = out.len();
                out.push(Instruction::JumpIfFalse(usize::MAX).encode());
                // then body
                for s in then_body {
                    compile_stmt(s, &mut out);
                }
                // jump to end (skip else)
                let jmp_end_index = out.len();
                out.push(Instruction::Jump(usize::MAX).encode());
                // patch JMPF to here (start of else)
                let else_target = out.len();
                patch_jump(&mut out[jmpf_index], else_target);
                // else body if present
                if let Some(eb) = otherwise_body {
                    for s in eb {
                        compile_stmt(s, &mut out);
                    }
                }
                // patch end jump to here
                let end_target = out.len();
                patch_jump(&mut out[jmp_end_index], end_target);
            }
            Stmt::WhileBlock { cond, body } => {
                let loop_start = out.len();
                compile_expr(cond, &mut out);
                let jmpf_index = out.len();
                out.push(Instruction::JumpIfFalse(usize::MAX).encode());
                for s in body {
                    compile_stmt(s, &mut out);
                }
                // jump back to start
                out.push(Instruction::Jump(loop_start).encode());
                // patch false to loop end
                let loop_end = out.len();
                patch_jump(&mut out[jmpf_index], loop_end);
            }
            Stmt::RepeatBlock { count, body } => {
                // Desugar: Repeat n times -> i = 0; while i < n { body; i = i + 1 }
                // We'll compile as: PUSH_NUM 0; STORE_VAR __i; <start> LOAD_VAR __i; <count>; LT; JMPF end; body; LOAD_VAR __i; PUSH_NUM 1; ADD; STORE_VAR __i; JMP start; end:
                out.push(Instruction::PushNum(0.0).encode());
                out.push(Instruction::StoreVar("__i".to_string()).encode());
                let start = out.len();
                out.push(Instruction::LoadVar("__i".to_string()).encode());
                compile_expr(count, &mut out);
                out.push(Instruction::Lt.encode());
                let jmpf = out.len();
                out.push(Instruction::JumpIfFalse(usize::MAX).encode());
                for s in body {
                    compile_stmt(s, &mut out);
                }
                out.push(Instruction::LoadVar("__i".to_string()).encode());
                out.push(Instruction::PushNum(1.0).encode());
                out.push(Instruction::Add.encode());
                out.push(Instruction::StoreVar("__i".to_string()).encode());
                out.push(Instruction::Jump(start).encode());
                let end = out.len();
                patch_jump(&mut out[jmpf], end);
            }
            _ => {}
        }
    }
    let text = out.join("\n") + "\n";
    text.into_bytes()
}

fn compile_expr(e: &Expr, out: &mut Vec<String>) {
    match e {
        Expr::Num(n) => out.push(Instruction::PushNum(*n).encode()),
        Expr::Str(s) => out.push(Instruction::PushStr(s.clone()).encode()),
        Expr::Ident(n) => out.push(Instruction::LoadVar(n.clone()).encode()),
        Expr::Plus(a, b) => {
            compile_expr(a, out);
            compile_expr(b, out);
            out.push(Instruction::Add.encode());
        }
        Expr::Minus(a, b) => {
            compile_expr(a, out);
            compile_expr(b, out);
            out.push(Instruction::Sub.encode());
        }
        Expr::Times(a, b) => {
            compile_expr(a, out);
            compile_expr(b, out);
            out.push(Instruction::Mul.encode());
        }
        Expr::DividedBy(a, b) => {
            compile_expr(a, out);
            compile_expr(b, out);
            out.push(Instruction::Div.encode());
        }
        Expr::Cmp(op, l, r) => {
            compile_expr(l, out);
            compile_expr(r, out);
            match op {
                CmpOp::Eq => out.push(Instruction::Eq.encode()),
                CmpOp::Ne => out.push(Instruction::Ne.encode()),
                CmpOp::Lt => out.push(Instruction::Lt.encode()),
                CmpOp::Le => out.push(Instruction::Le.encode()),
                CmpOp::Gt => out.push(Instruction::Gt.encode()),
                CmpOp::Ge => out.push(Instruction::Ge.encode()),
            }
        }
        Expr::And(a, b) => {
            // Boolean AND producing 1 or 0
            compile_expr(a, out);
            let jmp_false = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            compile_expr(b, out);
            let jmp_false2 = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            // both truthy -> 1
            out.push(Instruction::PushNum(1.0).encode());
            let jmp_end = out.len();
            out.push(Instruction::Jump(usize::MAX).encode());
            // patch falses to push 0
            let falsy_tgt = out.len();
            patch_jump(&mut out[jmp_false], falsy_tgt);
            patch_jump(&mut out[jmp_false2], falsy_tgt);
            out.push(Instruction::PushNum(0.0).encode());
            let end = out.len();
            patch_jump(&mut out[jmp_end], end);
        }
        Expr::Or(a, b) => {
            // Boolean OR producing 1 or 0
            compile_expr(a, out);
            let jmp_false = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            // a truthy -> 1
            out.push(Instruction::PushNum(1.0).encode());
            let jmp_end = out.len();
            out.push(Instruction::Jump(usize::MAX).encode());
            // a falsy -> evaluate b
            let else_tgt = out.len();
            patch_jump(&mut out[jmp_false], else_tgt);
            compile_expr(b, out);
            let jmp_false_b = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            out.push(Instruction::PushNum(1.0).encode());
            let jmp_end2 = out.len();
            out.push(Instruction::Jump(usize::MAX).encode());
            let falsy_tgt = out.len();
            patch_jump(&mut out[jmp_false_b], falsy_tgt);
            out.push(Instruction::PushNum(0.0).encode());
            let end = out.len();
            patch_jump(&mut out[jmp_end], end);
            patch_jump(&mut out[jmp_end2], end);
        }
        Expr::Not(a) => {
            // Not a -> if a is falsy push 1 else 0
            compile_expr(a, out);
            let jmp_false = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            // a truthy
            out.push(Instruction::PushNum(0.0).encode());
            let jmp_end = out.len();
            out.push(Instruction::Jump(usize::MAX).encode());
            // a falsy
            let t = out.len();
            patch_jump(&mut out[jmp_false], t);
            out.push(Instruction::PushNum(1.0).encode());
            let end = out.len();
            patch_jump(&mut out[jmp_end], end);
        }
        other => {
            // Fallback to string dump for complex expressions for now
            out.push(Instruction::PushStr(dump_expr(other)).encode());
        }
    }
}

fn compile_stmt(s: &Stmt, out: &mut Vec<String>) {
    match s {
        Stmt::Write(e) => {
            compile_expr(e, out);
            out.push(Instruction::WriteTop.encode());
        }
        Stmt::Set { name, value } => {
            compile_expr(value, out);
            out.push(Instruction::StoreVar(name.clone()).encode());
        }
        Stmt::AskFor { var_name } => {
            out.push(Instruction::AskVar(var_name.clone()).encode());
        }
        Stmt::IfBlock {
            cond,
            then_body,
            otherwise_body,
        } => {
            // Recursively handled by compile() loop, but support here for nested calls
            // cond
            compile_expr(cond, out);
            let jmpf_index = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            for st in then_body.iter() {
                compile_stmt(st, out);
            }
            let jmp_end_index = out.len();
            out.push(Instruction::Jump(usize::MAX).encode());
            let else_target = out.len();
            patch_jump(&mut out[jmpf_index], else_target);
            if let Some(eb) = otherwise_body {
                for st in eb.iter() {
                    compile_stmt(st, out);
                }
            }
            let end_target = out.len();
            patch_jump(&mut out[jmp_end_index], end_target);
        }
        Stmt::WhileBlock { cond, body } => {
            let loop_start = out.len();
            compile_expr(cond, out);
            let jmpf_index = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            for st in body.clone() {
                compile_stmt(&st, out);
            }
            out.push(Instruction::Jump(loop_start).encode());
            let loop_end = out.len();
            patch_jump(&mut out[jmpf_index], loop_end);
        }
        Stmt::RepeatBlock { count, body } => {
            // Fallback to top-level logic using dedicated comparisons
            out.push(Instruction::PushNum(0.0).encode());
            out.push(Instruction::StoreVar("__i".to_string()).encode());
            let start = out.len();
            out.push(Instruction::LoadVar("__i".to_string()).encode());
            compile_expr(count, out);
            out.push(Instruction::Lt.encode());
            let jmpf = out.len();
            out.push(Instruction::JumpIfFalse(usize::MAX).encode());
            for st in body {
                compile_stmt(st, out);
            }
            out.push(Instruction::LoadVar("__i".to_string()).encode());
            out.push(Instruction::PushNum(1.0).encode());
            out.push(Instruction::Add.encode());
            out.push(Instruction::StoreVar("__i".to_string()).encode());
            out.push(Instruction::Jump(start).encode());
            let end = out.len();
            patch_jump(&mut out[jmpf], end);
        }
        _ => {}
    }
}

fn patch_jump(encoded: &mut String, target: usize) {
    // Replace the placeholder usize::MAX with real target in textual encoding
    if encoded.starts_with("JMP\t") || encoded.starts_with("JMPF\t") {
        let parts: Vec<&str> = encoded.splitn(2, '\t').collect();
        *encoded = format!("{}\t{}", parts[0], target);
    }
}

fn dump_expr(e: &Expr) -> String {
    match e {
        Expr::Str(s) => s.clone(),
        Expr::Num(n) => format_number(*n),
        Expr::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        Expr::Null => "None".to_string(),
        Expr::Ident(x) => x.clone(),
        Expr::Plus(a, b) => format!("{} plus {}", dump_expr(a), dump_expr(b)),
        Expr::Minus(a, b) => format!("{} minus {}", dump_expr(a), dump_expr(b)),
        Expr::Times(a, b) => format!("{} times {}", dump_expr(a), dump_expr(b)),
        Expr::DividedBy(a, b) => format!("{} divided by {}", dump_expr(a), dump_expr(b)),
        Expr::And(a, b) => format!("{} And {}", dump_expr(a), dump_expr(b)),
        Expr::Or(a, b) => format!("{} Or {}", dump_expr(a), dump_expr(b)),
        Expr::Not(a) => format!("Not {}", dump_expr(a)),
        Expr::Cmp(op, l, r) => {
            let sym = match op {
                CmpOp::Lt => "<",
                CmpOp::Le => "<=",
                CmpOp::Gt => ">",
                CmpOp::Ge => ">=",
                CmpOp::Eq => "=",
                CmpOp::Ne => "!=",
            };
            format!("{} {} {}", dump_expr(l), sym, dump_expr(r))
        }
        Expr::Call { name, args } => {
            if args.is_empty() {
                name.clone()
            } else {
                format!(
                    "{} with {}",
                    name,
                    args.iter().map(dump_expr).collect::<Vec<_>>().join(", ")
                )
            }
        }
        Expr::ListLit(items) => format!(
            "List contains {}",
            items.iter().map(dump_expr).collect::<Vec<_>>().join(", ")
        ),
        Expr::DictLit(pairs) => pairs
            .iter()
            .map(|(k, v)| format!("\"{}\" set to {}", k, dump_expr(v)))
            .collect::<Vec<_>>()
            .join(", "),
        Expr::Index(base, idx) => format!("{}[{}]", dump_expr(base), dump_expr(idx)),
        // Phrasal built-in expressions
        Expr::TotalOf(expr) => format!("total of {}", dump_expr(expr)),
        Expr::SmallestIn(expr) => format!("smallest in {}", dump_expr(expr)),
        Expr::LargestIn(expr) => format!("largest in {}", dump_expr(expr)),
        Expr::AbsoluteValueOf(expr) => format!("absolute value of {}", dump_expr(expr)),
        Expr::Round(expr) => format!("round {}", dump_expr(expr)),
        Expr::RoundDown(expr) => format!("round down {}", dump_expr(expr)),
        Expr::RoundUp(expr) => format!("round up {}", dump_expr(expr)),
        Expr::MakeUppercase(expr) => format!("make uppercase {}", dump_expr(expr)),
        Expr::MakeLowercase(expr) => format!("make lowercase {}", dump_expr(expr)),
        Expr::TrimSpaces(expr) => format!("trim spaces from {}", dump_expr(expr)),
        Expr::FirstIn(expr) => format!("first in {}", dump_expr(expr)),
        Expr::LastIn(expr) => format!("last in {}", dump_expr(expr)),
        Expr::ReverseOf(expr) => format!("reverse of {}", dump_expr(expr)),
        Expr::CountOf(expr) => format!("count of {}", dump_expr(expr)),
        Expr::JoinWith(a, b) => format!("join {} with {}", dump_expr(a), dump_expr(b)),
        Expr::SplitBy(a, b) => format!("split {} by {}", dump_expr(a), dump_expr(b)),
        Expr::Contains(item, collection) => {
            format!("contains {} in {}", dump_expr(item), dump_expr(collection))
        }
        Expr::Remove(item, list) => format!("remove {} from {}", dump_expr(item), dump_expr(list)),
        Expr::Append(item, list) => format!("append {} to {}", dump_expr(item), dump_expr(list)),
        Expr::InsertAt(item, index, list) => format!(
            "insert {} at {} in {}",
            dump_expr(item),
            dump_expr(index),
            dump_expr(list)
        ),
        // File I/O operations
        Expr::ReadFile(path) => format!("read file at {}", dump_expr(path)),
        Expr::WriteFile(content, path) => {
            format!(
                "write {} to file at {}",
                dump_expr(content),
                dump_expr(path)
            )
        }
        Expr::AppendFile(content, path) => {
            format!(
                "append {} to file at {}",
                dump_expr(content),
                dump_expr(path)
            )
        }
        Expr::FileExists(path) => format!("file exists at {}", dump_expr(path)),
        Expr::DeleteFile(path) => format!("delete file at {}", dump_expr(path)),
        Expr::CreateDir(path) => format!("create directory at {}", dump_expr(path)),
        Expr::ListDir(path) => format!("list files in {}", dump_expr(path)),
        Expr::ReadLines(path) => format!("read lines from {}", dump_expr(path)),
        Expr::CopyFile(source, dest) => {
            format!(
                "copy file from {} to {}",
                dump_expr(source),
                dump_expr(dest)
            )
        }
        Expr::MoveFile(source, dest) => {
            format!(
                "move file from {} to {}",
                dump_expr(source),
                dump_expr(dest)
            )
        }
        // JSON operations
        Expr::ParseJson(s) => format!("parse json from {}", dump_expr(s)),
        Expr::ToJson(v) => format!("convert to json {}", dump_expr(v)),
        Expr::ToJsonPretty(v) => format!("convert to pretty json {}", dump_expr(v)),
        Expr::GetPathParam(name) => format!("get path parameter {}", dump_expr(name)),
        Expr::JsonGet(json, key) => format!("get {} from json {}", dump_expr(key), dump_expr(json)),
        Expr::JsonSet(json, key, val) => {
            format!(
                "set {} in json {} to {}",
                dump_expr(key),
                dump_expr(json),
                dump_expr(val)
            )
        }
        Expr::NewJsonObject => "new json object".to_string(),
        Expr::NewJsonArray => "new json array".to_string(),
        Expr::JsonPush(json, item) => {
            format!("push {} to json {}", dump_expr(item), dump_expr(json))
        }
        Expr::JsonLength(json) => format!("json length of {}", dump_expr(json)),
        // Error operations
        Expr::ErrorMessage(e) => format!("error message of {}", dump_expr(e)),
        Expr::ErrorType(e) => format!("error type of {}", dump_expr(e)),
        Expr::NewError {
            error_type,
            message,
        } => {
            format!(
                "error of type {} with message {}",
                error_type,
                dump_expr(message)
            )
        }
        // Web server operations
        Expr::CreateWebServer(port) => format!("create web server on port {}", dump_expr(port)),
        Expr::HtmlResponse(content) => format!("html response with {}", dump_expr(content)),
        Expr::JsonResponse(data) => format!("json response with {}", dump_expr(data)),
        Expr::JsonResponseStatus(data, status) => {
            format!(
                "json response with {} and status {}",
                dump_expr(data),
                dump_expr(status)
            )
        }
        Expr::RenderTemplate(template, data) => {
            format!(
                "render template {} with {}",
                dump_expr(template),
                dump_expr(data)
            )
        }
        Expr::ErrorResponse(status, message) => {
            format!(
                "error response with status {} and message {}",
                dump_expr(status),
                dump_expr(message)
            )
        }
        Expr::RequestField(request, field) => {
            format!("{}[{}]", dump_expr(request), dump_expr(field))
        }
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
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", now.as_secs())
}

fn to_string(v: &Value) -> String {
    match v {
        Value::Str(s) => s.clone(),
        Value::Num(n) => format_number(*n),
        Value::Bool(b) => {
            if *b {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        Value::Null => "None".to_string(),
        Value::Func(f) => format!("<function {}>", f.name),
        Value::List(xs) => format!(
            "[{}]",
            xs.iter().map(to_string).collect::<Vec<_>>().join(", ")
        ),
        Value::Dict(m) => {
            let mut parts: Vec<String> = Vec::new();
            for (k, v) in m.iter() {
                parts.push(format!("\"{}\": {}", k, to_string(v)));
            }
            format!("{{{}}}", parts.join(", "))
        }
        Value::Error(e) => {
            // Natural format when printing error values
            if matches!(e.kind, ErrorKind::Custom(_)) {
                format!("{} occurred: {}", e.type_string(), e.message)
            } else {
                format!("Error occurred: {} - {}", e.type_description(), e.message)
            }
        }
        Value::WebServer(_) => "<WebServer>".to_string(),
        Value::HttpRequest(_) => "<HttpRequest>".to_string(),
        Value::HttpResponse(r) => format!("<HttpResponse status={}>", r.status),
        Value::LiveReloadTracker(_) => "<LiveReloadTracker>".to_string(),
    }
}

fn to_num(v: Value) -> Result<f64> {
    match v {
        Value::Num(n) => Ok(n),
        Value::Str(s) => s
            .parse::<f64>()
            .map_err(|_| anyhow!("Cannot convert string to number: {}", s)),
        Value::Bool(b) => Ok(if b { 1.0 } else { 0.0 }),
        Value::Null => Ok(0.0),
        Value::Error(e) => Err(anyhow!("Cannot convert error to number: {}", e.message)),
        _ => Err(anyhow!("Cannot convert value to number: {}", to_string(&v))),
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
            if let Value::Num(n) = args[0] {
                (0..n.max(0.0) as i64).map(|i| i as f64).collect()
            } else {
                vec![]
            }
        }
        2 => {
            if let (Value::Num(a), Value::Num(b)) = (&args[0], &args[1]) {
                let (a, b) = (*a as i64, *b as i64);
                (a..b).map(|i| i as f64).collect()
            } else {
                vec![]
            }
        }
        3 => {
            if let (Value::Num(a), Value::Num(b), Value::Num(s)) = (&args[0], &args[1], &args[2]) {
                let (mut i, end, step) = (*a as i64, *b as i64, *s as i64);
                if step == 0 {
                    return Ok(Value::List(Vec::new()));
                }
                let mut out = Vec::new();
                if step > 0 {
                    while i < end {
                        out.push(i as f64);
                        i += step;
                    }
                } else {
                    while i > end {
                        out.push(i as f64);
                        i += step;
                    }
                }
                out
            } else {
                vec![]
            }
        }
        _ => vec![],
    };
    Ok(Value::List(nums.into_iter().map(Value::Num).collect()))
}

fn builtin_join(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Str(String::new()));
    }
    let sep = if args.len() >= 2 {
        to_string(&args[1])
    } else {
        String::new()
    };
    match &args[0] {
        Value::List(xs) => Ok(Value::Str(
            xs.iter().map(to_string).collect::<Vec<_>>().join(&sep),
        )),
        other => Ok(Value::Str(to_string(other))),
    }
}

fn builtin_split(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Ok(Value::List(Vec::new()));
    }
    let s = to_string(&args[0]);
    let sep = to_string(&args[1]);
    let parts = if sep.is_empty() {
        s.chars().map(|c| Value::Str(c.to_string())).collect()
    } else {
        s.split(&sep).map(|p| Value::Str(p.to_string())).collect()
    };
    Ok(Value::List(parts))
}

fn builtin_length(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Num(0.0));
    }
    match &args[0] {
        Value::List(xs) => Ok(Value::Num(xs.len() as f64)),
        Value::Str(s) => Ok(Value::Num(s.chars().count() as f64)),
        Value::Dict(m) => Ok(Value::Num(m.len() as f64)),
        _ => Ok(Value::Num(0.0)),
    }
}

fn builtin_sum(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Num(0.0));
    }
    match &args[0] {
        Value::List(xs) => {
            if xs.is_empty() {
                return Ok(Value::Num(0.0));
            }
            let mut total = 0.0;
            for (i, v) in xs.iter().enumerate() {
                match v {
                    Value::Num(n) => total += n,
                    other => {
                        return Err(anyhow!(
                            "total of expects numbers; item {} was {}",
                            i,
                            to_string(other)
                        ))
                    }
                }
            }
            Ok(Value::Num(total))
        }
        Value::Num(n) => Ok(Value::Num(*n)),
        other => Err(anyhow!(
            "total of expects a list or a number, got {}",
            to_string(other)
        )),
    }
}

fn builtin_min(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("smallest in requires at least one argument"));
    }
    match &args[0] {
        Value::List(xs) => {
            if xs.is_empty() {
                return Err(anyhow!("smallest in an empty list is undefined"));
            }
            let mut min_val = None::<f64>;
            for (i, v) in xs.iter().enumerate() {
                match v {
                    Value::Num(n) => {
                        min_val = Some(match min_val {
                            Some(m) => m.min(*n),
                            None => *n,
                        });
                    }
                    other => {
                        return Err(anyhow!(
                            "smallest in expects numbers; item {} was {}",
                            i,
                            to_string(other)
                        ))
                    }
                }
            }
            Ok(Value::Num(min_val.unwrap()))
        }
        Value::Num(n) => Ok(Value::Num(*n)),
        other => Err(anyhow!(
            "smallest in expects a list or a number, got {}",
            to_string(other)
        )),
    }
}

fn builtin_max(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("largest in requires at least one argument"));
    }
    match &args[0] {
        Value::List(xs) => {
            if xs.is_empty() {
                return Err(anyhow!("largest in an empty list is undefined"));
            }
            let mut max_val = None::<f64>;
            for (i, v) in xs.iter().enumerate() {
                match v {
                    Value::Num(n) => {
                        max_val = Some(match max_val {
                            Some(m) => m.max(*n),
                            None => *n,
                        });
                    }
                    other => {
                        return Err(anyhow!(
                            "largest in expects numbers; item {} was {}",
                            i,
                            to_string(other)
                        ))
                    }
                }
            }
            Ok(Value::Num(max_val.unwrap()))
        }
        Value::Num(n) => Ok(Value::Num(*n)),
        other => Err(anyhow!(
            "largest in expects a list or a number, got {}",
            to_string(other)
        )),
    }
}

fn builtin_abs(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("abs requires one argument"));
    }
    match &args[0] {
        Value::Num(n) => Ok(Value::Num(n.abs())),
        _ => Err(anyhow!("abs requires a number")),
    }
}

fn builtin_round(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("round requires one argument"));
    }
    match &args[0] {
        Value::Num(n) => Ok(Value::Num(n.round())),
        _ => Err(anyhow!("round requires a number")),
    }
}

fn builtin_floor(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("floor requires one argument"));
    }
    match &args[0] {
        Value::Num(n) => Ok(Value::Num(n.floor())),
        _ => Err(anyhow!("floor requires a number")),
    }
}

fn builtin_ceil(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("ceil requires one argument"));
    }
    match &args[0] {
        Value::Num(n) => Ok(Value::Num(n.ceil())),
        _ => Err(anyhow!("ceil requires a number")),
    }
}

fn builtin_uppercase(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Str(String::new()));
    }
    Ok(Value::Str(to_string(&args[0]).to_uppercase()))
}

fn builtin_lowercase(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Str(String::new()));
    }
    Ok(Value::Str(to_string(&args[0]).to_lowercase()))
}

fn builtin_trim(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Ok(Value::Str(String::new()));
    }
    Ok(Value::Str(to_string(&args[0]).trim().to_string()))
}

fn builtin_first(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("first requires one argument"));
    }
    match &args[0] {
        Value::List(xs) => {
            if xs.is_empty() {
                Err(anyhow!("first called on empty list"))
            } else {
                Ok(xs[0].clone())
            }
        }
        Value::Str(s) => {
            if s.is_empty() {
                Err(anyhow!("first called on empty string"))
            } else {
                Ok(Value::Str(s.chars().next().unwrap().to_string()))
            }
        }
        _ => Err(anyhow!("first requires a list or string")),
    }
}

fn builtin_last(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("last requires one argument"));
    }
    match &args[0] {
        Value::List(xs) => {
            if xs.is_empty() {
                Err(anyhow!("last called on empty list"))
            } else {
                Ok(xs[xs.len() - 1].clone())
            }
        }
        Value::Str(s) => {
            if s.is_empty() {
                Err(anyhow!("last called on empty string"))
            } else {
                Ok(Value::Str(s.chars().last().unwrap().to_string()))
            }
        }
        _ => Err(anyhow!("last requires a list or string")),
    }
}

fn builtin_reverse(args: &[Value]) -> Result<Value> {
    if args.is_empty() {
        return Err(anyhow!("reverse requires one argument"));
    }
    match &args[0] {
        Value::List(xs) => {
            let mut rev = xs.clone();
            rev.reverse();
            Ok(Value::List(rev))
        }
        Value::Str(s) => Ok(Value::Str(s.chars().rev().collect())),
        _ => Err(anyhow!("reverse requires a list or string")),
    }
}

fn builtin_contains(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!(
            "contains requires two arguments: item and collection"
        ));
    }
    let item = &args[0];
    match &args[1] {
        Value::List(xs) => {
            // Check if item exists in list (using value equality)
            for v in xs {
                if values_equal(item, v) {
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        Value::Str(s) => {
            // Check if string contains substring
            let needle = to_string(item);
            Ok(Value::Bool(s.contains(&needle)))
        }
        Value::Dict(m) => {
            // Check if key exists in dictionary
            let key = to_string(item);
            Ok(Value::Bool(m.contains_key(&key)))
        }
        _ => Err(anyhow!(
            "contains expects a list, string, or dictionary as second argument"
        )),
    }
}

fn builtin_remove(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("remove requires two arguments: item and list"));
    }
    let item = &args[0];
    match &args[1] {
        Value::List(xs) => {
            // Remove first occurrence of item from list
            let mut result = Vec::new();
            let mut removed = false;
            for v in xs {
                if !removed && values_equal(item, v) {
                    removed = true;
                } else {
                    result.push(v.clone());
                }
            }
            Ok(Value::List(result))
        }
        _ => Err(anyhow!("remove expects a list as second argument")),
    }
}

fn builtin_append(args: &[Value]) -> Result<Value> {
    if args.len() < 2 {
        return Err(anyhow!("append requires two arguments: item and list"));
    }
    let item = &args[0];
    match &args[1] {
        Value::List(xs) => {
            // Append item to end of list
            let mut result = xs.clone();
            result.push(item.clone());
            Ok(Value::List(result))
        }
        _ => Err(anyhow!("append expects a list as second argument")),
    }
}

fn builtin_insert_at(args: &[Value]) -> Result<Value> {
    if args.len() < 3 {
        return Err(anyhow!(
            "insert at requires three arguments: item, index, and list"
        ));
    }
    let item = &args[0];
    let index = match &args[1] {
        Value::Num(n) => *n as isize,
        _ => return Err(anyhow!("insert at expects a number as index")),
    };
    match &args[2] {
        Value::List(xs) => {
            let mut result = xs.clone();
            let len = result.len() as isize;
            // Handle negative indices (from end)
            let idx = if index < 0 {
                (len + index).max(0) as usize
            } else {
                index.min(len) as usize
            };
            result.insert(idx, item.clone());
            Ok(Value::List(result))
        }
        _ => Err(anyhow!("insert at expects a list as third argument")),
    }
}

// Helper function for value equality comparison
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Num(x), Value::Num(y)) => (x - y).abs() < f64::EPSILON,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

// ------------------------ IMPORTS ------------------------

impl Vm {
    fn import_local(&mut self, rel: &str) -> Result<()> {
        let mut path = self.base_dir.join(rel);
        if path.extension().is_none() {
            path.set_extension("poh");
        }
        let canon = fs::canonicalize(&path).unwrap_or(path.clone());
        let key = canon.to_string_lossy().to_string();
        if self.loaded_modules.contains(&key) {
            return Ok(());
        }
        if self.loading_stack.contains(&key) {
            return Err(anyhow!("Circular import detected: {}", key));
        }
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

    fn import_system(
        &mut self,
        name: &str,
        alias: Option<&str>,
        exposing: &[String],
    ) -> Result<()> {
        self.ensure_system_module(name)?;

        self.module_aliases
            .entry(name.to_string())
            .or_insert_with(|| name.to_string());

        if let Some(alias_name) = alias {
            if let Some(existing) = self.module_aliases.get(alias_name) {
                if existing != name {
                    return Err(anyhow!(
                        "Alias '{}' already bound to module '{}'",
                        alias_name,
                        existing
                    ));
                }
            }
            self.module_aliases
                .insert(alias_name.to_string(), name.to_string());
        }

        if !exposing.is_empty() {
            let exports = self
                .system_exports
                .get(name)
                .ok_or_else(|| anyhow!(format!("Module '{}' is not available", name)))?;
            for sym in exposing {
                let value = exports.get(sym).ok_or_else(|| {
                    anyhow!(format!("Module '{}' does not export '{}'", name, sym))
                })?;
                if let Some(existing) = self.exposed_symbols.get(sym) {
                    if existing != name {
                        return Err(anyhow!(format!(
                            "Symbol '{}' already exposed from module '{}'",
                            sym, existing
                        )));
                    }
                }
                self.globals.insert(sym.clone(), value.clone());
                self.exposed_symbols.insert(sym.clone(), name.to_string());
            }
        }

        Ok(())
    }

    fn find_stdlib_module(&self, file_name: &str) -> Option<PathBuf> {
        // 1) Env override
        if let Ok(root) = std::env::var("POHLANG_STDLIB") {
            let p = PathBuf::from(root).join(file_name);
            if p.exists() {
                return Some(p);
            }
        }
        // 2) Search from base_dir upwards for Interpreter/stdlib/<file>
        let mut cur: Option<&Path> = Some(self.base_dir.as_path());
        while let Some(dir) = cur {
            let cand = dir.join("Interpreter").join("stdlib").join(file_name);
            if cand.exists() {
                return Some(cand);
            }
            cur = dir.parent();
        }
        // 3) Try CWD fallback
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let cand = cwd.join("Interpreter").join("stdlib").join(file_name);
        if cand.exists() {
            return Some(cand);
        }
        None
    }
}

impl Vm {
    fn ensure_system_module(&mut self, name: &str) -> Result<()> {
        if self.system_exports.contains_key(name) {
            return Ok(());
        }
        let exports = self.load_system_module(name)?;
        self.system_exports.insert(name.to_string(), exports);
        Ok(())
    }

    fn load_system_module(&mut self, name: &str) -> Result<HashMap<String, Value>> {
        if let Some(existing) = self.system_exports.get(name) {
            return Ok(existing.clone());
        }

        let file_name = format!("{}.poh", name);
        if let Some(path) = self.find_stdlib_module(&file_name) {
            let canon = fs::canonicalize(&path).unwrap_or(path.clone());
            let src = fs::read_to_string(&canon)?;
            let program = crate::parser::parse(&src)?;

            let prev_base = self.base_dir.clone();
            let prev_globals = self.globals.clone();
            self.base_dir = canon.parent().unwrap_or(Path::new(".")).to_path_buf();
            self.loading_stack.push(format!("<system:{}>", name));
            let exec_res = self.execute(&program);
            self.loading_stack.pop();
            self.base_dir = prev_base;

            match exec_res {
                Ok(()) => {
                    let mut exports = HashMap::new();
                    for (k, v) in self.globals.iter() {
                        if !prev_globals.contains_key(k) {
                            exports.insert(k.clone(), v.clone());
                        }
                    }
                    self.globals = prev_globals;
                    self.loaded_system.insert(name.to_string());
                    Ok(exports)
                }
                Err(e) => {
                    self.globals = prev_globals;
                    Err(e)
                }
            }
        } else {
            self.loaded_system.insert(name.to_string());
            Ok(HashMap::new())
        }
    }

    fn resolve_value(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.globals.get(name) {
            return Some(v.clone());
        }
        if let Some((alias, symbol)) = split_qualified(name) {
            if let Some(module_name) = self.module_aliases.get(alias) {
                if let Some(exports) = self.system_exports.get(module_name) {
                    if let Some(v) = exports.get(symbol) {
                        return Some(v.clone());
                    }
                }
            }
        }
        None
    }

    fn call_qualified_function(&self, name: &str, args: &[Value]) -> Result<Value> {
        let (alias, symbol) = split_qualified(name)
            .ok_or_else(|| anyhow!(format!("Invalid qualified name '{}'", name)))?;
        let module_name = self
            .module_aliases
            .get(alias)
            .ok_or_else(|| anyhow!(format!("Unknown module alias '{}'", alias)))?;
        let exports = self
            .system_exports
            .get(module_name)
            .ok_or_else(|| anyhow!(format!("Module '{}' is not loaded", module_name)))?;
        match exports.get(symbol) {
            Some(Value::Func(f)) => self.call_func_value(f, args),
            Some(_) => Err(anyhow!(format!("'{}::{}' is not callable", alias, symbol))),
            None => Err(anyhow!(format!(
                "Module '{}' does not export '{}'",
                module_name, symbol
            ))),
        }
    }
}

fn split_qualified(name: &str) -> Option<(&str, &str)> {
    let (alias, rest) = name.split_once("::")?;
    if alias.is_empty() || rest.is_empty() {
        return None;
    }
    Some((alias, rest))
}
