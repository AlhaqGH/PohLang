use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Expr {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Ident(String),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    DividedBy(Box<Expr>, Box<Expr>),
    Call { name: String, args: Vec<Expr> },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Cmp(CmpOp, Box<Expr>, Box<Expr>),
    ListLit(Vec<Expr>),
    DictLit(Vec<(String, Expr)>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Write(Expr),
    AskFor { var_name: String },
    IfInline { cond: Expr, then_write: Expr, otherwise_write: Option<Expr> },
    IfBlock { cond: Expr, then_body: Program, otherwise_body: Option<Program> },
    FuncInline { name: String, params: Vec<Param>, body: Expr },
    FuncBlock { name: String, params: Vec<Param>, body: Program },
    WhileBlock { cond: Expr, body: Program },
    RepeatBlock { count: Expr, body: Program },
    ImportLocal { path: String },
    ImportSystem { name: String },
    Use { name: String, args: Vec<Expr> },
    Set { name: String, value: Expr },
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct Param { pub name: String, pub default: Option<Expr> }

pub type Program = Vec<Stmt>;

pub fn parse(src: &str) -> Result<Program> {
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0usize;
    parse_block(&lines, &mut i)
}

fn parse_block(lines: &[&str], i: &mut usize) -> Result<Program> {
    let mut prog = Vec::new();
    while *i < lines.len() {
        let raw = lines[*i];
        let t = raw.trim();
        // Block terminator
        if t == "End" { *i += 1; break; }
        if t.is_empty() { *i += 1; continue; }

        // Define function (inline): "Define function name with parameter(s) ... as <expr>"
        if let Some(rest) = t.strip_prefix("Define function ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected function name", *i+1))?;
            let after_name = after_name.trim_start();
            let after_with = after_name.strip_prefix("with ").ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected 'with'", *i+1))?;
            let after_with = after_with.strip_prefix("parameters ").unwrap_or(after_with);
            let after_with = after_with.strip_prefix("parameter ").unwrap_or(after_with);
            if let Some((params_str, body_str)) = split_once_word(after_with, " as ") {
                let params = parse_params(params_str)?;
                let body = parse_expr(body_str.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                prog.push(Stmt::FuncInline { name, params, body });
                *i += 1; continue;
            } else {
                return Err(anyhow!("[file: Line {}: Col 1] Expected 'as <expr>'", *i+1));
            }
        }

        // Call statement: "Call name with a and b" (alias of Use)
        if let Some(rest) = t.strip_prefix("Call ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected function name", *i+1))?;
            let after_with = after_name.trim_start().strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() { vec![] } else { parse_arg_list_multi(after_with, true)? };
            prog.push(Stmt::Use { name, args });
            *i += 1; continue;
        }

        // Write
        if let Some(rest) = t.strip_prefix("Write ") {
            let expr = parse_expr(rest).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
            prog.push(Stmt::Write(expr));
            *i += 1; continue;
        }

        // Ask for
        if let Some(rest) = t.strip_prefix("Ask for ") {
            if let Some((var_name, rest_after)) = split_ident(rest) {
                if rest_after.trim().is_empty() {
                    prog.push(Stmt::AskFor { var_name });
                    *i += 1; continue;
                }
            }
            return Err(anyhow!("[file: Line {}: Col 1] Expected variable name after 'Ask for'", *i+1));
        }

        // Set name to expr
        if let Some(rest) = t.strip_prefix("Set ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let after = after.strip_prefix("to ").unwrap_or(after);
                let expr = parse_expr(after).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                prog.push(Stmt::Set { name, value: expr });
                *i += 1; continue;
            }
        }

        // Increase name by expr -> desugar to Set name to name plus expr
        if let Some(rest) = t.strip_prefix("Increase ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                // Desugar: Increase x by 5 -> Set x to x plus 5
                let value = Expr::Plus(
                    Box::new(Expr::Ident(name.clone())),
                    Box::new(amount)
                );
                prog.push(Stmt::Set { name, value });
                *i += 1; continue;
            }
        }

        // Decrease name by expr -> desugar to Set name to name minus expr
        if let Some(rest) = t.strip_prefix("Decrease ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                // Desugar: Decrease x by 5 -> Set x to x minus 5
                let value = Expr::Minus(
                    Box::new(Expr::Ident(name.clone())),
                    Box::new(amount)
                );
                prog.push(Stmt::Set { name, value });
                *i += 1; continue;
            }
        }

        // If inline vs block
        if let Some(rest) = t.strip_prefix("If ") {
            if t.contains(" Write ") {
                // Inline
                let lc = rest;
                let (cond_str, after_cond) = split_once_word(lc, " Write ")
                    .ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected 'Write' after condition", *i+1))?;
                let (then_str, otherwise_part) = split_once_word(after_cond, " Otherwise ").unwrap_or((after_cond, ""));
                let then_expr = parse_expr(then_str.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                let cond_expr = parse_expr(cond_str.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                let otherwise_expr = if !otherwise_part.trim().is_empty() {
                    let esp = otherwise_part.trim();
                    let esp = if let Some(x) = esp.strip_prefix("Write ") { x } else { esp };
                    Some(parse_expr(esp.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?)
                } else { None };
                prog.push(Stmt::IfInline { cond: cond_expr, then_write: then_expr, otherwise_write: otherwise_expr });
                *i += 1; continue;
            } else {
                // Block If
                let cond_expr = parse_expr(rest.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                *i += 1; // consume If line
                let then_body = parse_until_keywords(lines, i, &["Otherwise", "End"])?;
                let mut otherwise_body = None;
                if *i < lines.len() {
                    let t2 = lines[*i].trim();
                    if t2 == "Otherwise" {
                        *i += 1; // consume Otherwise
                        let eb = parse_until_keywords(lines, i, &["End"]) ?;
                        otherwise_body = Some(eb);
                    }
                }
                // expect End
                if *i < lines.len() {
                    let t3 = lines[*i].trim();
                    if t3 == "End" { *i += 1; } else {
                        return Err(anyhow!("[file: Line {}: Col 1] Expected 'End'", *i+1));
                    }
                } else {
                    return Err(anyhow!("[file: Line {}: Col 1] Expected 'End' before EOF", *i));
                }
                prog.push(Stmt::IfBlock { cond: cond_expr, then_body, otherwise_body });
                continue;
            }
        }

        // Import statements
        if let Some(rest) = t.strip_prefix("Import system ") {
            // Expect quoted name
            if let Some(n) = extract_quoted(rest.trim()) {
                prog.push(Stmt::ImportSystem { name: n });
                *i += 1; continue;
            } else { return Err(anyhow!("[file: Line {}: Col 1] Expected quoted module name", *i+1)); }
        }
        if let Some(rest) = t.strip_prefix("Import ") {
            if let Some(p) = extract_quoted(rest.trim()) {
                prog.push(Stmt::ImportLocal { path: p });
                *i += 1; continue;
            } else { return Err(anyhow!("[file: Line {}: Col 1] Expected quoted path", *i+1)); }
        }

        // While block
        if let Some(rest) = t.strip_prefix("While ") {
            let cond_expr = parse_expr(rest.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
            *i += 1; // consume header
            let body = parse_until_keywords(lines, i, &["End"])?;
            if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("[file: Line {}: Col 1] Expected 'End' after While", *i+1)); }
            prog.push(Stmt::WhileBlock { cond: cond_expr, body });
            continue;
        }

        // Repeat block (counted): "Repeat <expr> times" or just "Repeat <expr>"
        if let Some(rest) = t.strip_prefix("Repeat ") {
            let mut r = rest.trim().to_string();
            if let Some(idx) = r.rfind(" times") { if idx == r.len() - " times".len() { r.truncate(idx); } }
            let count_expr = parse_expr(r.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
            *i += 1; // consume header
            let body = parse_until_keywords(lines, i, &["End"])?;
            if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("[file: Line {}: Col 1] Expected 'End' after Repeat", *i+1)); }
            prog.push(Stmt::RepeatBlock { count: count_expr, body });
            continue;
        }

        // Make inline or block
        if let Some(rest) = t.strip_prefix("Make ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected function name", *i+1))?;
            let after_name = after_name.trim_start();
            let after_with = after_name.strip_prefix("with ").ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected 'with'", *i+1))?;
            if let Some((params_str, after_params)) = split_once_word(after_with, " Write ") {
                // Inline form
                let params = parse_params(params_str)?;
                let body = parse_expr(after_params.trim()).map_err(|e| anyhow!("[file: Line {}: Col 1] {}", *i+1, e))?;
                prog.push(Stmt::FuncInline { name, params, body });
                *i += 1; continue;
            } else {
                // Block form: rest of line has only params
                let params = parse_params(after_with)?;
                *i += 1; // consume header line
                let body = parse_until_keywords(lines, i, &["End"]) ?;
                // Expect End
                if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; }
                else { return Err(anyhow!("[file: Line {}: Col 1] Expected 'End' after function block", *i+1)); }
                prog.push(Stmt::FuncBlock { name, params, body });
                continue;
            }
        }

        // Use name with args
        if let Some(rest) = t.strip_prefix("Use ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("[file: Line {}: Col 1] Expected function name", *i+1))?;
            let after_name = after_name.trim_start();
            let after_with = after_name.strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() { vec![] } else { parse_arg_list_multi(after_with, true)? };
            prog.push(Stmt::Use { name, args });
            *i += 1; continue;
        }

        // Return [expr]
        if let Some(rest) = t.strip_prefix("Return") {
            let expr = {
                let r = rest.trim();
                if r.is_empty() { None } else { Some(parse_expr(r)?) }
            };
            prog.push(Stmt::Return(expr));
            *i += 1; continue;
        }

        return Err(anyhow!("[file: Line {}: Col 1] Unsupported statement", *i+1));
    }
    Ok(prog)
}

fn parse_until_keywords(lines: &[&str], i: &mut usize, stops: &[&str]) -> Result<Program> {
    let mut out = Vec::new();
    while *i < lines.len() {
        let t = lines[*i].trim();
        if stops.iter().any(|s| t == *s) { break; }

        // Delegate one statement parse similar to parse_block
        // Define function (inline)
        if let Some(rest) = t.strip_prefix("Define function ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name.trim_start().strip_prefix("with ").ok_or_else(|| anyhow!("Expected 'with'"))?;
            let after_with = after_with.strip_prefix("parameters ").unwrap_or(after_with);
            let after_with = after_with.strip_prefix("parameter ").unwrap_or(after_with);
            if let Some((params_str, body_str)) = split_once_word(after_with, " as ") {
                let params = parse_params(params_str)?;
                let body = parse_expr(body_str.trim())?;
                out.push(Stmt::FuncInline { name, params, body });
                *i += 1; continue;
            } else { return Err(anyhow!("Expected 'as <expr>'")); }
        }
        // Write
        if let Some(rest) = t.strip_prefix("Write ") {
            let expr = parse_expr(rest)?;
            out.push(Stmt::Write(expr));
            *i += 1; continue;
        }
        // Ask for
        if let Some(rest) = t.strip_prefix("Ask for ") {
            if let Some((var_name, rest_after)) = split_ident(rest) {
                if rest_after.trim().is_empty() {
                    out.push(Stmt::AskFor { var_name });
                    *i += 1; continue;
                }
            }
            return Err(anyhow!("Expected variable name after 'Ask for'"));
        }
        // Call statement (alias of Use)
        if let Some(rest) = t.strip_prefix("Call ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name.trim_start().strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() { vec![] } else { parse_arg_list_multi(after_with, true)? };
            out.push(Stmt::Use { name, args });
            *i += 1; continue;
        }
        // Set
        if let Some(rest) = t.strip_prefix("Set ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let after = after.strip_prefix("to ").unwrap_or(after);
                let expr = parse_expr(after)?;
                out.push(Stmt::Set { name, value: expr });
                *i += 1; continue;
            }
        }
        // Increase name by expr -> desugar to Set name to name plus expr
        if let Some(rest) = t.strip_prefix("Increase ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str)?;
                let value = Expr::Plus(
                    Box::new(Expr::Ident(name.clone())),
                    Box::new(amount)
                );
                out.push(Stmt::Set { name, value });
                *i += 1; continue;
            }
        }
        // Decrease name by expr -> desugar to Set name to name minus expr
        if let Some(rest) = t.strip_prefix("Decrease ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str)?;
                let value = Expr::Minus(
                    Box::new(Expr::Ident(name.clone())),
                    Box::new(amount)
                );
                out.push(Stmt::Set { name, value });
                *i += 1; continue;
            }
        }
        // Inline If
        if let Some(rest) = t.strip_prefix("If ") { if t.contains(" Write ") {
            let lc = rest;
            let (cond_str, after_cond) = split_once_word(lc, " Write ").ok_or_else(|| anyhow!("Expected 'Write' after condition"))?;
            let (then_str, otherwise_part) = split_once_word(after_cond, " Otherwise ").unwrap_or((after_cond, ""));
            let then_expr = parse_expr(then_str.trim())?;
            let cond_expr = parse_expr(cond_str.trim())?;
            let otherwise_expr = if !otherwise_part.trim().is_empty() {
                let esp = otherwise_part.trim();
                let esp = if let Some(x) = esp.strip_prefix("Write ") { x } else { esp };
                Some(parse_expr(esp.trim())?)
            } else { None };
            out.push(Stmt::IfInline { cond: cond_expr, then_write: then_expr, otherwise_write: otherwise_expr });
            *i += 1; continue; }
        }
        // Block If
        if let Some(rest) = t.strip_prefix("If ") {
            let cond_expr = parse_expr(rest.trim())?;
            *i += 1;
            let then_body = parse_until_keywords(lines, i, &["Otherwise", "End"]) ?;
            let mut otherwise_body = None;
            if *i < lines.len() && lines[*i].trim() == "Otherwise" {
                *i += 1;
                otherwise_body = Some(parse_until_keywords(lines, i, &["End"]) ?);
            }
            if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("Expected 'End'")); }
            out.push(Stmt::IfBlock { cond: cond_expr, then_body: then_body, otherwise_body });
            continue;
        }
        // Import statements inside blocks
        if let Some(rest) = t.strip_prefix("Import system ") {
            if let Some(n) = extract_quoted(rest.trim()) { out.push(Stmt::ImportSystem { name: n }); *i += 1; continue; }
            else { return Err(anyhow!("Expected quoted module name")); }
        }
        if let Some(rest) = t.strip_prefix("Import ") {
            if let Some(p) = extract_quoted(rest.trim()) { out.push(Stmt::ImportLocal { path: p }); *i += 1; continue; }
            else { return Err(anyhow!("Expected quoted path")); }
        }
        // While block
        if let Some(rest) = t.strip_prefix("While ") {
            let cond_expr = parse_expr(rest.trim())?;
            *i += 1;
            let body = parse_until_keywords(lines, i, &["End"]) ?;
            if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("Expected 'End' after While")); }
            out.push(Stmt::WhileBlock { cond: cond_expr, body });
            continue;
        }
        // Repeat block
        if let Some(rest) = t.strip_prefix("Repeat ") {
            let mut r = rest.trim().to_string();
            if let Some(idx) = r.rfind(" times") { if idx == r.len() - " times".len() { r.truncate(idx); } }
            let count_expr = parse_expr(r.trim())?;
            *i += 1;
            let body = parse_until_keywords(lines, i, &["End"]) ?;
            if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("Expected 'End' after Repeat")); }
            out.push(Stmt::RepeatBlock { count: count_expr, body });
            continue;
        }
        // Make inline
        if let Some(rest) = t.strip_prefix("Make ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_name = after_name.trim_start();
            let after_with = after_name.strip_prefix("with ").ok_or_else(|| anyhow!("Expected 'with'"))?;
            if let Some((params_str, after_params)) = split_once_word(after_with, " Write ") {
                let params = parse_params(params_str)?;
                let body = parse_expr(after_params.trim())?;
                out.push(Stmt::FuncInline { name, params, body });
                *i += 1; continue;
            } else {
                // Block func
                let params = parse_params(after_with)?;
                *i += 1;
                let body = parse_until_keywords(lines, i, &["End"]) ?;
                if *i < lines.len() && lines[*i].trim() == "End" { *i += 1; } else { return Err(anyhow!("Expected 'End' after function block")); }
                out.push(Stmt::FuncBlock { name, params, body });
                continue;
            }
        }
        // Use
        if let Some(rest) = t.strip_prefix("Use ") {
            let (name, after_name) = split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name.trim_start().strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() { vec![] } else { parse_arg_list_multi(after_with, true)? };
            out.push(Stmt::Use { name, args });
            *i += 1; continue;
        }
        // Return
        if let Some(rest) = t.strip_prefix("Return") {
            let r = rest.trim();
            let expr = if r.is_empty() { None } else { Some(parse_expr(r)?) };
            out.push(Stmt::Return(expr));
            *i += 1; continue;
        }
        return Err(anyhow!("Unsupported statement: {}", t));
    }
    Ok(out)
}

fn split_once_word<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    if let Some(idx) = s.find(pat) {
        let (a, b) = s.split_at(idx);
        let b = &b[pat.len()..];
        Some((a, b))
    } else { None }
}

fn split_ident(s: &str) -> Option<(String, &str)> {
    let mut chars = s.chars();
    let mut i = 0;
    for c in chars.by_ref() {
        if c.is_alphanumeric() || c == '_' { i += c.len_utf8(); } else { break; }
    }
    if i == 0 { return None; }
    Some((s[..i].to_string(), &s[i..]))
}

fn parse_params(s: &str) -> Result<Vec<Param>> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut buf = String::new();
    for ch in s.chars() {
        match ch {
            '"' => { in_str = !in_str; buf.push(ch); }
            '(' if !in_str => { depth += 1; buf.push(ch); }
            ')' if !in_str => { depth -= 1; buf.push(ch); }
            ',' if !in_str && depth == 0 => {
                let item = buf.trim().to_string();
                if !item.is_empty() { out.push(parse_param_item(&item)?); }
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }
    if !buf.trim().is_empty() { out.push(parse_param_item(buf.trim())?); }
    Ok(out)
}

fn parse_param_item(s: &str) -> Result<Param> {
    let s = s.trim();
    if let Some(idx) = s.find(" set to ") {
        let name = s[..idx].trim();
        let expr_str = &s[idx + " set to ".len()..];
        if let Some((n, rest)) = split_ident(name) { if rest.trim().is_empty() {
            let def = parse_expr(expr_str.trim())?;
            return Ok(Param { name: n, default: Some(def) });
        }}
        return Err(anyhow!("Invalid parameter default: {}", s));
    }
    if let Some(idx) = s.find(" defaulting to ") {
        let name = s[..idx].trim();
        let expr_str = &s[idx + " defaulting to ".len()..];
        if let Some((n, rest)) = split_ident(name) { if rest.trim().is_empty() {
            let def = parse_expr(expr_str.trim())?;
            return Ok(Param { name: n, default: Some(def) });
        }}
        return Err(anyhow!("Invalid parameter default: {}", s));
    }
    if let Some((n, rest)) = split_ident(s) { if rest.trim().is_empty() {
        return Ok(Param { name: n, default: None });
    }}
    Err(anyhow!("Invalid parameter: {}", s))
}

fn parse_arg_list(s: &str) -> Result<Vec<Expr>> { parse_arg_list_multi(s, false) }

fn parse_arg_list_multi(s: &str, allow_and: bool) -> Result<Vec<Expr>> {
    // Split by commas (always) and optionally by " and " at top level
    let mut parts: Vec<String> = split_top_level(s, ",");
    if allow_and {
        let mut expanded = Vec::new();
        for p in parts.into_iter() {
            let sub = split_top_level(&p, " and ");
            for item in sub { if !item.trim().is_empty() { expanded.push(item); } }
        }
        parts = expanded;
    }
    let mut args = Vec::new();
    for p in parts {
        let t = p.trim();
        if t.is_empty() { continue; }
        args.push(parse_expr(t)?);
    }
    Ok(args)
}

pub fn parse_expr(s: &str) -> Result<Expr> { parse_or(s) }

fn parse_or(s: &str) -> Result<Expr> {
    let parts = split_top_level_multi(s, &[" Or ", " or "]);
    if parts.len() > 1 {
        let mut it = parts.into_iter();
        let mut e = parse_and(it.next().unwrap().trim())?;
        for p in it { let r = parse_and(p.trim())?; e = Expr::Or(Box::new(e), Box::new(r)); }
        return Ok(e);
    }
    parse_and(s)
}

fn parse_and(s: &str) -> Result<Expr> {
    let parts = split_top_level_multi(s, &[" And ", " and "]);
    if parts.len() > 1 {
        let mut it = parts.into_iter();
        let mut e = parse_not(it.next().unwrap().trim())?;
        for p in it { let r = parse_not(p.trim())?; e = Expr::And(Box::new(e), Box::new(r)); }
        return Ok(e);
    }
    parse_not(s)
}

fn parse_not(s: &str) -> Result<Expr> {
    let st = s.trim_start();
    if let Some(rest) = st.strip_prefix("Not ") { return Ok(Expr::Not(Box::new(parse_not(rest)?))); }
    if let Some(rest) = st.strip_prefix("not ") { return Ok(Expr::Not(Box::new(parse_not(rest)?))); }
    parse_cmp(st)
}

fn parse_cmp(s: &str) -> Result<Expr> {
    // Recognize comparisons at top-level, not inside strings or parens; prefer longest match
    let cmps = [
        (" is not ", CmpOp::Ne),
        (" Is Not ", CmpOp::Ne),
        (" is ", CmpOp::Eq),
        (" Is ", CmpOp::Eq),
        (" Greater Or Equal ", CmpOp::Ge),
        (" Less Or Equal ", CmpOp::Le),
        (" greater or equal ", CmpOp::Ge),
        (" less or equal ", CmpOp::Le),
        (" Greater Than ", CmpOp::Gt),
        (" Less Than ", CmpOp::Lt),
        (" greater than ", CmpOp::Gt),
        (" less than ", CmpOp::Lt),
        (" Not Equals ", CmpOp::Ne),
        (" not equals ", CmpOp::Ne),
        (" Equals ", CmpOp::Eq),
        (" equals ", CmpOp::Eq),
        (" = ", CmpOp::Eq),
    ];
    for (pat, op) in cmps.iter() {
        if let Some((l, r)) = split_once_top_level(s, pat) {
            let le = parse_add(l.trim())?;
            let re = parse_add(r.trim())?;
            return Ok(Expr::Cmp(op.clone(), Box::new(le), Box::new(re)));
        }
    }
    parse_add(s)
}

fn parse_add(s: &str) -> Result<Expr> {
    // Handle addition and subtraction (left-to-right)
    let plus_parts = split_top_level(s, " plus ");
    let minus_parts = split_top_level(s, " minus ");
    
    // If we have both operators, we need to handle them in order
    // For simplicity, we'll process left-to-right by finding the first occurrence
    if plus_parts.len() > 1 || minus_parts.len() > 1 {
        // Find which operator comes first
        let mut tokens: Vec<(usize, bool, String)> = Vec::new(); // (position, is_plus, text)
        let mut pos = 0;
        let bytes = s.as_bytes();
        let mut in_str = false;
        let mut depth = 0;
        let mut buf = String::new();
        
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'"' { in_str = !in_str; buf.push('"'); i += 1; continue; }
            if !in_str {
                if bytes[i] == b'(' { depth += 1; buf.push('('); i += 1; continue; }
                if bytes[i] == b')' { depth -= 1; buf.push(')'); i += 1; continue; }
                if depth == 0 {
                    if s[i..].starts_with(" plus ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, true, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " plus ".len();
                        pos = tokens.len();
                        continue;
                    }
                    if s[i..].starts_with(" minus ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, false, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " minus ".len();
                        pos = tokens.len();
                        continue;
                    }
                }
            }
            buf.push(s[i..].chars().next().unwrap());
            i += s[i..].chars().next().unwrap().len_utf8();
        }
        if !buf.trim().is_empty() {
            tokens.push((pos, true, buf.trim().to_string())); // Last token, operator doesn't matter
        }
        
        if tokens.len() > 1 {
            let mut expr = parse_mult(tokens[0].2.trim())?;
            for i in 1..tokens.len() {
                let rhs = parse_mult(tokens[i].2.trim())?;
                if tokens[i-1].1 { // Previous token marked as plus
                    expr = Expr::Plus(Box::new(expr), Box::new(rhs));
                } else {
                    expr = Expr::Minus(Box::new(expr), Box::new(rhs));
                }
            }
            return Ok(expr);
        }
    }
    
    parse_mult(s)
}

fn parse_mult(s: &str) -> Result<Expr> {
    // Handle multiplication and division (left-to-right, higher precedence than +/-)
    let times_parts = split_top_level(s, " times ");
    let div_parts = split_top_level(s, " divided by ");
    
    if times_parts.len() > 1 || div_parts.len() > 1 {
        // Find which operator comes first
        let mut tokens: Vec<(usize, bool, String)> = Vec::new(); // (position, is_times, text)
        let mut pos = 0;
        let bytes = s.as_bytes();
        let mut in_str = false;
        let mut depth = 0;
        let mut buf = String::new();
        
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'"' { in_str = !in_str; buf.push('"'); i += 1; continue; }
            if !in_str {
                if bytes[i] == b'(' { depth += 1; buf.push('('); i += 1; continue; }
                if bytes[i] == b')' { depth -= 1; buf.push(')'); i += 1; continue; }
                if depth == 0 {
                    if s[i..].starts_with(" times ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, true, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " times ".len();
                        pos = tokens.len();
                        continue;
                    }
                    if s[i..].starts_with(" divided by ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, false, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " divided by ".len();
                        pos = tokens.len();
                        continue;
                    }
                }
            }
            buf.push(s[i..].chars().next().unwrap());
            i += s[i..].chars().next().unwrap().len_utf8();
        }
        if !buf.trim().is_empty() {
            tokens.push((pos, true, buf.trim().to_string())); // Last token, operator doesn't matter
        }
        
        if tokens.len() > 1 {
            let mut expr = parse_term(tokens[0].2.trim())?;
            for i in 1..tokens.len() {
                let rhs = parse_term(tokens[i].2.trim())?;
                if tokens[i-1].1 { // Previous token marked as times
                    expr = Expr::Times(Box::new(expr), Box::new(rhs));
                } else {
                    expr = Expr::DividedBy(Box::new(expr), Box::new(rhs));
                }
            }
            return Ok(expr);
        }
    }
    
    parse_term(s)
}

fn split_top_level(s: &str, delim: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_str = false;
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' { in_str = !in_str; buf.push('"'); i += 1; continue; }
        if !in_str {
            if bytes[i] == b'(' || bytes[i] == b'[' || bytes[i] == b'{' { depth += 1; buf.push(bytes[i] as char); i += 1; continue; }
            if bytes[i] == b')' || bytes[i] == b']' || bytes[i] == b'}' { depth -= 1; buf.push(bytes[i] as char); i += 1; continue; }
            if depth == 0 {
                if s[i..].starts_with(delim) {
                    out.push(buf.trim().to_string());
                    buf.clear();
                    i += delim.len();
                    continue;
                }
            }
        }
        buf.push(s[i..].chars().next().unwrap());
        i += s[i..].chars().next().unwrap().len_utf8();
    }
    if !buf.trim().is_empty() { out.push(buf.trim().to_string()); }
    out
}

fn split_top_level_multi(s: &str, delims: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_str = false;
    let mut depth = 0i32;
    let mut i = 0;
    while i < s.len() {
        let ch = s[i..].chars().next().unwrap();
        if ch == '"' { in_str = !in_str; buf.push(ch); i += ch.len_utf8(); continue; }
        if !in_str {
            if ch == '(' || ch == '[' || ch == '{' { depth += 1; buf.push(ch); i += 1; continue; }
            if ch == ')' || ch == ']' || ch == '}' { depth -= 1; buf.push(ch); i += 1; continue; }
            if depth == 0 {
                let mut matched = None;
                for d in delims {
                    if s[i..].starts_with(d) { matched = Some(*d); break; }
                }
                if let Some(d) = matched {
                    out.push(buf.trim().to_string());
                    buf.clear();
                    i += d.len();
                    continue;
                }
            }
        }
        buf.push(ch);
        i += ch.len_utf8();
    }
    if !buf.trim().is_empty() { out.push(buf.trim().to_string()); }
    out
}

fn split_once_top_level<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut in_str = false;
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + pat.len() <= bytes.len() {
        let ch = s[i..].chars().next().unwrap();
        if ch == '"' { in_str = !in_str; i += ch.len_utf8(); continue; }
        if !in_str {
            if ch == '(' || ch == '[' || ch == '{' { depth += 1; i += 1; continue; }
            if ch == ')' || ch == ']' || ch == '}' { depth -= 1; i += 1; continue; }
            if depth == 0 && s[i..].starts_with(pat) {
                let (l, r) = s.split_at(i);
                let r = &r[pat.len()..];
                return Some((l, r));
            }
        }
        i += ch.len_utf8();
    }
    None
}

#[derive(Debug, Clone)]
pub enum CmpOp { Lt, Le, Gt, Ge, Eq, Ne }

fn parse_term(s: &str) -> Result<Expr> {
    let s = s.trim();
    if s.is_empty() { return Err(anyhow!("Empty expression")); }
    
    // Modern list literal: [1, 2, 3]
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s[1..s.len()-1].trim();
        let items = if inner.is_empty() { vec![] } else { parse_arg_list(inner)? };
        return Ok(Expr::ListLit(items));
    }
    
    // Modern dictionary literal: {key: value, key2: value2}
    if s.starts_with('{') && s.ends_with('}') {
        let inner = s[1..s.len()-1].trim();
        let mut pairs = Vec::new();
        if !inner.is_empty() {
            let parts = split_top_level(inner, ",");
            for part in parts {
                let p = part.trim();
                // split by colon at top level
                if let Some(idx) = p.find(':') {
                    // Check if this colon is at top level (not inside parens/strings)
                    let mut in_str = false;
                    let mut depth = 0i32;
                    let mut colon_idx = None;
                    for (i, ch) in p.char_indices() {
                        if ch == '"' { in_str = !in_str; continue; }
                        if !in_str {
                            if ch == '(' || ch == '[' || ch == '{' { depth += 1; }
                            else if ch == ')' || ch == ']' || ch == '}' { depth -= 1; }
                            else if ch == ':' && depth == 0 { colon_idx = Some(i); break; }
                        }
                    }
                    if let Some(cidx) = colon_idx {
                        let kpart = p[..cidx].trim();
                        let vpart = p[cidx+1..].trim();
                        let vexpr = parse_expr(vpart)?;
                        // Key can be a quoted string or an identifier
                        if let Some(k) = extract_quoted(kpart) {
                            pairs.push((k, vexpr));
                        } else if kpart.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            pairs.push((kpart.to_string(), vexpr));
                        } else {
                            return Err(anyhow!("Dictionary key must be string literal or identifier"));
                        }
                    } else {
                        return Err(anyhow!("Expected ':' in dictionary literal item"));
                    }
                } else {
                    return Err(anyhow!("Expected ':' in dictionary literal item"));
                }
            }
        }
        return Ok(Expr::DictLit(pairs));
    }
    
    // String literal
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2) || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2) {
        let quote = s.chars().next().unwrap();
        return Ok(Expr::Str(s[1..s.len()-1].to_string()));
    }
    // Booleans
    if s.eq_ignore_ascii_case("True") { return Ok(Expr::Bool(true)); }
    if s.eq_ignore_ascii_case("False") { return Ok(Expr::Bool(false)); }
    // Null/None
    if s.eq_ignore_ascii_case("Null") || s.eq_ignore_ascii_case("Nothing") || s == "None" { return Ok(Expr::Null); }
    // List literal: legacy form "List contains <exprs>"
    if let Some(rest) = s.strip_prefix("List contains ") {
        let items = if rest.trim().is_empty() { vec![] } else { parse_arg_list(rest)? };
        return Ok(Expr::ListLit(items));
    }
    // Dictionary literal: legacy form "Dictionary contains \"k\" set to v, ..."
    if let Some(rest) = s.strip_prefix("Dictionary contains ") {
        let mut pairs = Vec::new();
        // split rest by commas at top-level
        let parts = split_top_level(rest, ",");
        for part in parts {
            let p = part.trim();
            // expect "key" set to value
            if let Some(idx) = p.find(" set to ") {
                let (kpart, vpart) = p.split_at(idx);
                let vexpr = parse_expr(vpart[" set to ".len()..].trim())?;
                if let Some(k) = extract_quoted(kpart.trim()) {
                    pairs.push((k, vexpr));
                } else {
                    return Err(anyhow!("Expected quoted key in dictionary literal"));
                }
            } else {
                return Err(anyhow!("Expected 'set to' in dictionary literal item"));
            }
        }
        return Ok(Expr::DictLit(pairs));
    }
    // Number
    if let Ok(n) = s.parse::<f64>() { return Ok(Expr::Num(n)); }
    // Phrasal call: name with args
    if let Some((name, after)) = split_ident(s) {
        let after = after.trim_start();
        if let Some(rest) = after.strip_prefix("with ") {
            let args = if rest.trim().is_empty() { vec![] } else { parse_arg_list(rest)? };
            return Ok(Expr::Call { name, args });
        }
    }
    // Call form: name(args)
    if let Some(idx) = s.find('(') {
        if s.ends_with(')') {
            let name = s[..idx].trim();
            let args_str = &s[idx+1..s.len()-1];
            let args = if args_str.trim().is_empty() { vec![] } else { parse_arg_list(args_str)? };
            return Ok(Expr::Call { name: name.to_string(), args });
        }
    }
    // Identifier
    if let Some((id, rest)) = split_ident(s) {
        if rest.trim().is_empty() { return Ok(Expr::Ident(id)); }
    }
    Err(anyhow!("Could not parse expression: {}", s))
}

fn extract_quoted(s: &str) -> Option<String> {
    let st = s.trim();
    if st.starts_with('"') {
        if let Some(pos) = st[1..].find('"') {
            return Some(st[1..1+pos].to_string());
        }
    }
    None
}
