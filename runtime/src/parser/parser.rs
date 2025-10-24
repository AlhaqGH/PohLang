use crate::parser::ast::{CatchHandler, CmpOp, Expr, Param, Program, Stmt};
use crate::parser::phrases as P;
use crate::parser::phrases::strip_prefix_ci;
use anyhow::{anyhow, Result};

// Helper to check if a line starts with any of the given prefixes (case-insensitive)
fn line_starts_with_any(line: &str, prefixes: &[&str]) -> bool {
    for prefix in prefixes {
        if P::strip_prefix_ci(line, prefix).is_some() {
            return true;
        }
    }
    false
}

fn suggest_fix(error_msg: &str, context: &str) -> String {
    let suggestions = vec![
        (
            "Expected 'with'",
            "Hint: Function definitions use 'Define function name with parameter as expression'",
        ),
        (
            "Expected function name",
            "Hint: Function name must be a valid identifier (letters, numbers, underscore)",
        ),
        (
            "Expected 'as <expr>'",
            "Hint: Inline functions need 'as' followed by an expression",
        ),
        (
            "Expected variable name",
            "Hint: Variable names must start with a letter or underscore",
        ),
        (
            "Could not parse expression",
            "Hint: Check for unmatched brackets [], braces {}, or parentheses ()",
        ),
        (
            "Empty expression",
            "Hint: Expressions cannot be empty. Provide a value, variable, or operation",
        ),
        (
            "Unsupported statement",
            "Hint: Valid statements: Write, Set, Ask for, If, Repeat, While, Make, Use, Import",
        ),
        (
            "out of range",
            "Hint: Check array bounds. Use negative indexing (-1) for last element",
        ),
        (
            "not found",
            "Hint: Verify the key exists in the dictionary or check for typos",
        ),
        (
            "division by zero",
            "Hint: Ensure denominator is not zero before dividing",
        ),
    ];

    for (pattern, suggestion) in suggestions {
        if error_msg.contains(pattern) {
            return format!("{}.\n{}", error_msg, suggestion);
        }
    }

    // If context is provided and looks like incomplete syntax, add context-specific hint
    if !context.is_empty() {
        if context.contains("Set ") && !context.contains(" to ") {
            return format!(
                "{}.\nHint: Set statements require 'to': Set variable to value",
                error_msg
            );
        }
        if context.contains("If ") && !context.contains(" Write ") {
            return format!(
                "{}.\nHint: Inline If needs: If condition Write expression Otherwise expression",
                error_msg
            );
        }
    }

    error_msg.to_string()
}

// AST types now provided by crate::parser::ast

pub fn parse(src: &str) -> Result<Program> {
    let lines: Vec<&str> = src.lines().collect();
    let mut i = 0usize;

    // Skip leading blank lines
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }

    if i == lines.len() {
        return Err(anyhow!("[file: Line 1: Col 1] Expected 'Start Program'"));
    }

    if !lines[i].trim().eq_ignore_ascii_case("Start Program") {
        return Err(anyhow!(
            "[file: Line {}: Col 1] Expected 'Start Program'",
            i + 1
        ));
    }
    i += 1; // consume Start Program

    let prog = parse_until_keywords(&lines, &mut i, &["End Program"])?;

    if i >= lines.len() || !lines[i].trim().eq_ignore_ascii_case("End Program") {
        let line = if i < lines.len() {
            i + 1
        } else {
            lines.len() + 1
        };
        return Err(anyhow!(
            "[file: Line {}: Col 1] Expected 'End Program'",
            line
        ));
    }
    i += 1; // consume End Program

    while i < lines.len() {
        if !lines[i].trim().is_empty() {
            return Err(anyhow!(
                "[file: Line {}: Col 1] Unexpected content after 'End Program'",
                i + 1
            ));
        }
        i += 1;
    }

    Ok(prog)
}

fn parse_until_keywords(lines: &[&str], i: &mut usize, stops: &[&str]) -> Result<Program> {
    let mut out = Vec::new();
    while *i < lines.len() {
        let t = lines[*i].trim();
        // Check exact match or prefix match
        if stops.contains(&t) || line_starts_with_any(t, stops) {
            break;
        }

        // Skip blank lines
        if t.is_empty() {
            *i += 1;
            continue;
        }

        // Skip comments (lines starting with // or #)
        if t.starts_with("//") || t.starts_with("#") {
            *i += 1;
            continue;
        }

        // Define function (inline)
        if let Some(rest) = t.strip_prefix("Define function ") {
            let (name, after_name) =
                split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name
                .trim_start()
                .strip_prefix("with ")
                .ok_or_else(|| anyhow!("Expected 'with'"))?;
            let after_with = after_with.strip_prefix("parameters ").unwrap_or(after_with);
            let after_with = after_with.strip_prefix("parameter ").unwrap_or(after_with);
            if let Some((params_str, body_str)) = split_once_word(after_with, " as ") {
                let params = parse_params(params_str)?;
                let body = parse_expr(body_str.trim())?;
                out.push(Stmt::FuncInline { name, params, body });
                *i += 1;
                continue;
            } else {
                return Err(anyhow!("Expected 'as <expr>'"));
            }
        }
        // Write statement - check for "to file" pattern first
        if let Some(rest) = t.strip_prefix("Write ") {
            // Check if it's "Write <content> to file at <path>"
            if let Some((content_part, file_part)) = rest.split_once(" to file at ") {
                let content_expr = parse_expr(content_part.trim())?;
                let path_expr = parse_expr(file_part.trim())?;
                // Convert to a statement that writes to file
                let write_file_expr = Expr::WriteFile(Box::new(content_expr), Box::new(path_expr));
                out.push(Stmt::Write(write_file_expr));
                *i += 1;
                continue;
            }
            // Regular Write statement
            let expr = parse_expr(rest)?;
            out.push(Stmt::Write(expr));
            *i += 1;
            continue;
        }
        // Append statement - "Append <content> to file at <path>"
        if let Some(rest) = t.strip_prefix("Append ") {
            if let Some((content_part, file_part)) = rest.split_once(" to file at ") {
                let content_expr = parse_expr(content_part.trim())?;
                let path_expr = parse_expr(file_part.trim())?;
                // Convert to a statement that appends to file
                let append_file_expr =
                    Expr::AppendFile(Box::new(content_expr), Box::new(path_expr));
                out.push(Stmt::Write(append_file_expr));
                *i += 1;
                continue;
            }
            return Err(anyhow!("Expected 'Append <content> to file at <path>'"));
        }
        // Delete file statement - "Delete file at <path>"
        if let Some(rest) = t.strip_prefix("Delete file at ") {
            let path_expr = parse_expr(rest.trim())?;
            let delete_expr = Expr::DeleteFile(Box::new(path_expr));
            out.push(Stmt::Write(delete_expr));
            *i += 1;
            continue;
        }
        // Ask for
        if let Some(rest) = t.strip_prefix("Ask for ") {
            if let Some((var_name, rest_after)) = split_ident(rest) {
                if rest_after.trim().is_empty() {
                    out.push(Stmt::AskFor { var_name });
                    *i += 1;
                    continue;
                }
            }
            return Err(anyhow!("Expected variable name after 'Ask for'"));
        }
        // Call statement (alias of Use)
        if let Some(rest) = t.strip_prefix("Call ") {
            let (name, after_name) =
                split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name.trim_start().strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() {
                vec![]
            } else {
                parse_arg_list_multi(after_with, true)?
            };
            out.push(Stmt::Use { name, args });
            *i += 1;
            continue;
        }
        // Set
        if let Some(rest) = t.strip_prefix("Set ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let after = after.strip_prefix("to ").unwrap_or(after);
                let expr = parse_expr(after)?;
                out.push(Stmt::Set { name, value: expr });
                *i += 1;
                continue;
            }
        }
        // Increase name by expr -> desugar to Set name to name plus expr
        if let Some(rest) = t.strip_prefix("Increase ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str)?;
                let value = Expr::Plus(Box::new(Expr::Ident(name.clone())), Box::new(amount));
                out.push(Stmt::Set { name, value });
                *i += 1;
                continue;
            }
        }
        // Decrease name by expr -> desugar to Set name to name minus expr
        if let Some(rest) = t.strip_prefix("Decrease ") {
            if let Some((name, after)) = split_ident(rest) {
                let after = after.trim_start();
                let amount_str = after.strip_prefix("by ").unwrap_or(after);
                let amount = parse_expr(amount_str)?;
                let value = Expr::Minus(Box::new(Expr::Ident(name.clone())), Box::new(amount));
                out.push(Stmt::Set { name, value });
                *i += 1;
                continue;
            }
        }
        // Inline If
        if let Some(rest) = t.strip_prefix("If ") {
            if t.contains(" Write ") {
                let lc = rest;
                let (cond_str, after_cond) = split_once_word(lc, " Write ")
                    .ok_or_else(|| anyhow!("Expected 'Write' after condition"))?;
                let (then_str, otherwise_part) =
                    split_once_word(after_cond, " Otherwise ").unwrap_or((after_cond, ""));
                let then_expr = parse_expr(then_str.trim())?;
                let cond_expr = parse_expr(cond_str.trim())?;
                let otherwise_expr = if !otherwise_part.trim().is_empty() {
                    let esp = otherwise_part.trim();
                    let esp = if let Some(x) = esp.strip_prefix("Write ") {
                        x
                    } else {
                        esp
                    };
                    Some(parse_expr(esp.trim())?)
                } else {
                    None
                };
                out.push(Stmt::IfInline {
                    cond: cond_expr,
                    then_write: then_expr,
                    otherwise_write: otherwise_expr,
                });
                *i += 1;
                continue;
            }
        }
        // Block If
        if let Some(rest) = t.strip_prefix("If ") {
            // Strip trailing colon if present
            let rest = rest.trim().strip_suffix(':').unwrap_or(rest.trim());
            let cond_expr = parse_expr(rest)?;
            *i += 1;
            let then_body = parse_until_keywords(lines, i, &["Otherwise", "End If", "End"])?;
            let mut otherwise_body = None;
            if *i < lines.len() && lines[*i].trim() == "Otherwise" {
                *i += 1;
                otherwise_body = Some(parse_until_keywords(lines, i, &["End If", "End"])?);
            }
            if *i < lines.len() {
                let end_line = lines[*i].trim();
                if end_line == "End If" || end_line == "End" {
                    *i += 1;
                } else {
                    return Err(anyhow!("Expected 'End If' or 'End', found '{}'", end_line));
                }
            } else {
                return Err(anyhow!("Expected 'End If' or 'End'"));
            }
            out.push(Stmt::IfBlock {
                cond: cond_expr,
                then_body,
                otherwise_body,
            });
            continue;
        }
        // Import statements inside blocks
        if let Some(rest) = t.strip_prefix("Import system ") {
            let trimmed = rest.trim();
            let (module, mut remainder) = extract_quoted_and_rest(trimmed)
                .ok_or_else(|| anyhow!("Expected quoted module name"))?;
            let mut alias = None;
            let mut exposing = Vec::new();

            remainder = remainder.trim();
            if remainder.starts_with("as ") {
                remainder = remainder[3..].trim_start();
                if let Some((name, after)) = split_ident(remainder) {
                    if !after.trim().is_empty() && !after.trim_start().starts_with("exposing ") {
                        return Err(anyhow!("Unexpected content after alias in system import"));
                    }
                    alias = Some(name);
                    remainder = after.trim_start();
                } else {
                    return Err(anyhow!("Expected alias name after 'as'"));
                }
            }

            if let Some(rest_expose) = remainder.strip_prefix("exposing ") {
                exposing = parse_exposing_list(rest_expose)?;
                remainder = "";
            }

            if !remainder.trim().is_empty() {
                return Err(anyhow!("Unexpected trailing content in system import"));
            }

            out.push(Stmt::ImportSystem {
                name: module,
                alias,
                exposing,
            });
            *i += 1;
            continue;
        }
        if let Some(rest) = t.strip_prefix("Import ") {
            if let Some(p) = extract_quoted(rest.trim()) {
                out.push(Stmt::ImportLocal { path: p });
                *i += 1;
                continue;
            } else {
                return Err(anyhow!("Expected quoted path"));
            }
        }
        // While block
        if let Some(rest) = t.strip_prefix("While ") {
            let cond_expr = parse_expr(rest.trim())?;
            *i += 1;
            let body = parse_until_keywords(lines, i, &["End"])?;
            if *i < lines.len() && lines[*i].trim() == "End" {
                *i += 1;
            } else {
                return Err(anyhow!("Expected 'End' after While"));
            }
            out.push(Stmt::WhileBlock {
                cond: cond_expr,
                body,
            });
            continue;
        }
        // Repeat block
        if let Some(rest) = t.strip_prefix("Repeat ") {
            let mut r = rest.trim().to_string();
            if let Some(idx) = r.rfind(" times") {
                if idx == r.len() - " times".len() {
                    r.truncate(idx);
                }
            }
            let count_expr = parse_expr(r.trim())?;
            *i += 1;
            let body = parse_until_keywords(lines, i, &["End"])?;
            if *i < lines.len() && lines[*i].trim() == "End" {
                *i += 1;
            } else {
                return Err(anyhow!("Expected 'End' after Repeat"));
            }
            out.push(Stmt::RepeatBlock {
                count: count_expr,
                body,
            });
            continue;
        }
        // Make inline
        if let Some(rest) = t.strip_prefix("Make ") {
            let (name, after_name) =
                split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_name = after_name.trim_start();
            let after_with = after_name
                .strip_prefix("with ")
                .ok_or_else(|| anyhow!("Expected 'with'"))?;
            if let Some((params_str, after_params)) = split_once_word(after_with, " Write ") {
                let params = parse_params(params_str)?;
                let body = parse_expr(after_params.trim())?;
                out.push(Stmt::FuncInline { name, params, body });
                *i += 1;
                continue;
            } else {
                // Block func
                let params = parse_params(after_with)?;
                *i += 1;
                let body = parse_until_keywords(lines, i, &["End"])?;
                if *i < lines.len() && lines[*i].trim() == "End" {
                    *i += 1;
                } else {
                    return Err(anyhow!("Expected 'End' after function block"));
                }
                out.push(Stmt::FuncBlock { name, params, body });
                continue;
            }
        }
        // Use
        if let Some(rest) = t.strip_prefix("Use ") {
            let (name, after_name) =
                split_ident(rest).ok_or_else(|| anyhow!("Expected function name"))?;
            let after_with = after_name.trim_start().strip_prefix("with ").unwrap_or("");
            let args = if after_with.is_empty() {
                vec![]
            } else {
                parse_arg_list_multi(after_with, true)?
            };
            out.push(Stmt::Use { name, args });
            *i += 1;
            continue;
        }
        // Try-Catch
        if P::strip_prefix_ci(t, P::P_TRY).is_some() {
            *i += 1;
            // Parse try block
            let try_block = parse_until_keywords(
                lines,
                i,
                &[P::P_IF_ERROR, P::P_FINALLY, P::P_END_TRY, "End"],
            )?;

            let mut catch_handlers = Vec::new();
            let mut finally_block = None;

            // Parse catch handlers
            while *i < lines.len() {
                let line = lines[*i].trim();

                // Check for "if error" catch clause
                if let Some(rest) = P::strip_prefix_ci(line, P::P_IF_ERROR) {
                    *i += 1;

                    let rest = rest.trim();
                    let mut error_type = None;
                    let mut var_name = None;

                    // Check for "of type X"
                    if let Some(after_of_type) = strip_prefix_ci(rest, "of type ") {
                        // Split on " as " to get type and variable name
                        if let Some((type_part, var_part)) = split_once_word(after_of_type, " as ")
                        {
                            error_type = Some(type_part.trim().trim_matches('"').to_string());
                            var_name = Some(var_part.trim().to_string());
                        } else {
                            // Just type, no variable
                            error_type = Some(after_of_type.trim().trim_matches('"').to_string());
                        }
                    } else if let Some(rest_as) = strip_prefix_ci(rest, "as ") {
                        // Just "if error as var_name" - catch all errors
                        var_name = Some(rest_as.trim().to_string());
                    }
                    // else: just "if error" - catch all without binding

                    // Parse catch body
                    let block = parse_until_keywords(
                        lines,
                        i,
                        &[P::P_IF_ERROR, P::P_FINALLY, P::P_END_TRY, "End"],
                    )?;

                    catch_handlers.push(CatchHandler {
                        error_type,
                        var_name,
                        block,
                    });
                    continue;
                }

                // Check for "finally"
                if P::strip_prefix_ci(line, P::P_FINALLY).is_some() {
                    *i += 1;
                    finally_block = Some(parse_until_keywords(lines, i, &[P::P_END_TRY, "End"])?);
                    continue;
                }

                // Check for "end try"
                if P::strip_prefix_ci(line, P::P_END_TRY).is_some() || line == "End" {
                    *i += 1;
                    break;
                }

                return Err(anyhow!(
                    "Expected 'if error', 'finally', or 'end try', found '{}'",
                    line
                ));
            }

            out.push(Stmt::TryCatch {
                try_block,
                catch_handlers,
                finally_block,
            });
            continue;
        }
        // Return
        if let Some(rest) = t.strip_prefix("Return") {
            let r = rest.trim();
            let expr = if r.is_empty() {
                None
            } else {
                Some(parse_expr(r)?)
            };
            out.push(Stmt::Return(expr));
            *i += 1;
            continue;
        }
        // Throw
        if let Some(rest) = P::strip_prefix_ci(t, P::P_THROW) {
            let expr = parse_expr(rest.trim())?;
            out.push(Stmt::Throw(expr));
            *i += 1;
            continue;
        }

        // Web Framework Statements
        // Add route <path> with method <method> to server:
        if let Some(rest) = P::strip_prefix_ci(t, "add route ") {
            if let Some((path_and_method, _)) = rest.split_once(" to server:") {
                if let Some((path_part, method_part)) = path_and_method.split_once(" with method ")
                {
                    let path_expr = parse_expr(path_part.trim())?;
                    let method_expr = parse_expr(method_part.trim())?;

                    // Parse handler block (indented lines following)
                    *i += 1;
                    let handler_start = *i;

                    // Find indented handler lines
                    while *i < lines.len() {
                        let line = lines[*i];
                        if line.trim().is_empty() {
                            *i += 1;
                            continue;
                        }
                        // If line starts with whitespace, it's part of the handler
                        if line.starts_with(' ') || line.starts_with('\t') {
                            *i += 1;
                        } else {
                            break;
                        }
                    }

                    // Parse handler as a program
                    let handler_lines: Vec<&str> = lines[handler_start..*i]
                        .iter()
                        .map(|l| l.trim_start())
                        .collect();
                    let mut handler_i = 0;
                    let handler_prog = parse_until_keywords(&handler_lines, &mut handler_i, &[])?;

                    out.push(Stmt::AddRoute {
                        path: path_expr,
                        method: method_expr,
                        handler: handler_prog,
                    });
                    continue;
                }
            }
        }

        // Start server
        if P::strip_prefix_ci(t, "start server").is_some() {
            out.push(Stmt::StartServer);
            *i += 1;
            continue;
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
    } else {
        None
    }
}

fn split_ident(s: &str) -> Option<(String, &str)> {
    let mut chars = s.chars();
    let mut i = 0;
    for c in chars.by_ref() {
        if c.is_alphanumeric() || c == '_' {
            i += c.len_utf8();
        } else {
            break;
        }
    }
    if i == 0 {
        return None;
    }
    Some((s[..i].to_string(), &s[i..]))
}

fn parse_params(s: &str) -> Result<Vec<Param>> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut buf = String::new();
    for ch in s.chars() {
        match ch {
            '"' => {
                in_str = !in_str;
                buf.push(ch);
            }
            '(' if !in_str => {
                depth += 1;
                buf.push(ch);
            }
            ')' if !in_str => {
                depth -= 1;
                buf.push(ch);
            }
            ',' if !in_str && depth == 0 => {
                let item = buf.trim().to_string();
                if !item.is_empty() {
                    out.push(parse_param_item(&item)?);
                }
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }
    if !buf.trim().is_empty() {
        out.push(parse_param_item(buf.trim())?);
    }
    Ok(out)
}

fn parse_param_item(s: &str) -> Result<Param> {
    let s = s.trim();
    if let Some(idx) = s.find(" set to ") {
        let name = s[..idx].trim();
        let expr_str = &s[idx + " set to ".len()..];
        if let Some((n, rest)) = split_ident(name) {
            if rest.trim().is_empty() {
                let def = parse_expr(expr_str.trim())?;
                return Ok(Param {
                    name: n,
                    default: Some(def),
                });
            }
        }
        return Err(anyhow!("Invalid parameter default: {}", s));
    }
    if let Some(idx) = s.find(" defaulting to ") {
        let name = s[..idx].trim();
        let expr_str = &s[idx + " defaulting to ".len()..];
        if let Some((n, rest)) = split_ident(name) {
            if rest.trim().is_empty() {
                let def = parse_expr(expr_str.trim())?;
                return Ok(Param {
                    name: n,
                    default: Some(def),
                });
            }
        }
        return Err(anyhow!("Invalid parameter default: {}", s));
    }
    if let Some((n, rest)) = split_ident(s) {
        if rest.trim().is_empty() {
            return Ok(Param {
                name: n,
                default: None,
            });
        }
    }
    Err(anyhow!("Invalid parameter: {}", s))
}

fn parse_arg_list(s: &str) -> Result<Vec<Expr>> {
    parse_arg_list_multi(s, false)
}

fn parse_arg_list_multi(s: &str, allow_and: bool) -> Result<Vec<Expr>> {
    // Split by commas (always) and optionally by " and " at top level
    let mut parts: Vec<String> = split_top_level(s, ",");
    if allow_and {
        let mut expanded = Vec::new();
        for p in parts.into_iter() {
            let sub = split_top_level(&p, " and ");
            for item in sub {
                if !item.trim().is_empty() {
                    expanded.push(item);
                }
            }
        }
        parts = expanded;
    }
    let mut args = Vec::new();
    for p in parts {
        let t = p.trim();
        if t.is_empty() {
            continue;
        }
        args.push(parse_expr(t)?);
    }
    Ok(args)
}

pub fn parse_expr(s: &str) -> Result<Expr> {
    parse_or(s)
}

fn try_parse_phrasal_call(s: &str) -> Option<Expr> {
    let st = s.trim();
    if let Some((name, after)) = split_ident(st) {
        let after = after.trim_start();
        if let Some(rest) = strip_prefix_ci(after, "with ") {
            // Don't treat "error of type X with message Y" as a phrasal call
            // Check if the name is "error" and after starts with "of type"
            if name == "error" && after.trim_start().starts_with("of type") {
                return None;
            }
            if let Ok(args) = parse_arg_list_multi(rest, true) {
                return Some(Expr::Call { name, args });
            }
        }
    }
    None
}

fn parse_or(s: &str) -> Result<Expr> {
    if let Some(call) = try_parse_phrasal_call(s) {
        return Ok(call);
    }
    let parts = split_top_level_multi(s, &[" Or ", " or "]);
    if parts.len() > 1 {
        let mut it = parts.into_iter();
        let mut e = parse_and(it.next().unwrap().trim())?;
        for p in it {
            let r = parse_and(p.trim())?;
            e = Expr::Or(Box::new(e), Box::new(r));
        }
        return Ok(e);
    }
    parse_and(s)
}

fn parse_and(s: &str) -> Result<Expr> {
    if let Some(call) = try_parse_phrasal_call(s) {
        return Ok(call);
    }
    let parts = split_top_level_multi(s, &[" And ", " and "]);
    if parts.len() > 1 {
        let mut it = parts.into_iter();
        let mut e = parse_not(it.next().unwrap().trim())?;
        for p in it {
            let r = parse_not(p.trim())?;
            e = Expr::And(Box::new(e), Box::new(r));
        }
        return Ok(e);
    }
    parse_not(s)
}

fn parse_not(s: &str) -> Result<Expr> {
    let st = s.trim_start();
    if let Some(rest) = st.strip_prefix("Not ") {
        return Ok(Expr::Not(Box::new(parse_not(rest)?)));
    }
    if let Some(rest) = st.strip_prefix("not ") {
        return Ok(Expr::Not(Box::new(parse_not(rest)?)));
    }
    parse_cmp(st)
}

fn parse_cmp(s: &str) -> Result<Expr> {
    // Recognize comparisons at top-level, not inside strings or parens; prefer longest match
    // Order matters: check longer patterns first to avoid premature matching
    let cmps = [
        // Full phrasal forms with "is" (longest patterns first)
        (" is greater than or equal to ", CmpOp::Ge),
        (" is less than or equal to ", CmpOp::Le),
        (" Is Greater Than Or Equal To ", CmpOp::Ge),
        (" Is Less Than Or Equal To ", CmpOp::Le),
        (" is not equal to ", CmpOp::Ne),
        (" Is Not Equal To ", CmpOp::Ne),
        (" is equal to ", CmpOp::Eq),
        (" Is Equal To ", CmpOp::Eq),
        (" is greater than ", CmpOp::Gt),
        (" is less than ", CmpOp::Lt),
        (" Is Greater Than ", CmpOp::Gt),
        (" Is Less Than ", CmpOp::Lt),
        // Shorter forms without "is"
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
        // Basic "is" (check before symbolic to prefer phrasal)
        (" is not ", CmpOp::Ne),
        (" Is Not ", CmpOp::Ne),
        (" is ", CmpOp::Eq),
        (" Is ", CmpOp::Eq),
        // Symbolic operators (check after phrasal forms)
        (" >= ", CmpOp::Ge),
        (" <= ", CmpOp::Le),
        (" != ", CmpOp::Ne),
        (" == ", CmpOp::Eq),
        (" > ", CmpOp::Gt),
        (" < ", CmpOp::Lt),
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
    // Support both phrasal (plus/minus) and symbolic (+/-)
    let plus_parts = split_top_level(s, " plus ");
    let minus_parts = split_top_level(s, " minus ");
    let sym_plus_parts = split_top_level(s, " + ");
    let sym_minus_parts = split_top_level(s, " - ");

    // If we have any operators, process them in order
    if plus_parts.len() > 1
        || minus_parts.len() > 1
        || sym_plus_parts.len() > 1
        || sym_minus_parts.len() > 1
    {
        // Find which operator comes first
        let mut tokens: Vec<(usize, bool, String)> = Vec::new(); // (position, is_plus, text)
        let mut pos = 0;
        let bytes = s.as_bytes();
        let mut in_str = false;
        let mut depth = 0;
        let mut buf = String::new();

        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'"' {
                in_str = !in_str;
                buf.push('"');
                i += 1;
                continue;
            }
            if !in_str {
                if bytes[i] == b'(' || bytes[i] == b'[' || bytes[i] == b'{' {
                    depth += 1;
                    buf.push(bytes[i] as char);
                    i += 1;
                    continue;
                }
                if bytes[i] == b')' || bytes[i] == b']' || bytes[i] == b'}' {
                    depth -= 1;
                    buf.push(bytes[i] as char);
                    i += 1;
                    continue;
                }
                if depth == 0 {
                    // Check phrasal operators first (longer patterns)
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
                    // Check symbolic operators
                    if s[i..].starts_with(" + ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, true, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " + ".len();
                        pos = tokens.len();
                        continue;
                    }
                    if s[i..].starts_with(" - ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, false, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " - ".len();
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
                if tokens[i - 1].1 {
                    // Previous token marked as plus
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
    // Support both phrasal (times/divided by) and symbolic (*/)
    let times_parts = split_top_level(s, " times ");
    let div_parts = split_top_level(s, " divided by ");
    let sym_mult_parts = split_top_level(s, " * ");
    let sym_div_parts = split_top_level(s, " / ");

    if times_parts.len() > 1
        || div_parts.len() > 1
        || sym_mult_parts.len() > 1
        || sym_div_parts.len() > 1
    {
        // Find which operator comes first
        let mut tokens: Vec<(usize, bool, String)> = Vec::new(); // (position, is_times, text)
        let mut pos = 0;
        let bytes = s.as_bytes();
        let mut in_str = false;
        let mut depth = 0;
        let mut buf = String::new();

        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'"' {
                in_str = !in_str;
                buf.push('"');
                i += 1;
                continue;
            }
            if !in_str {
                if bytes[i] == b'(' || bytes[i] == b'[' || bytes[i] == b'{' {
                    depth += 1;
                    buf.push(bytes[i] as char);
                    i += 1;
                    continue;
                }
                if bytes[i] == b')' || bytes[i] == b']' || bytes[i] == b'}' {
                    depth -= 1;
                    buf.push(bytes[i] as char);
                    i += 1;
                    continue;
                }
                if depth == 0 {
                    // Check phrasal operators first (longer patterns)
                    if s[i..].starts_with(" divided by ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, false, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " divided by ".len();
                        pos = tokens.len();
                        continue;
                    }
                    if s[i..].starts_with(" times ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, true, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " times ".len();
                        pos = tokens.len();
                        continue;
                    }
                    // Check symbolic operators
                    if s[i..].starts_with(" * ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, true, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " * ".len();
                        pos = tokens.len();
                        continue;
                    }
                    if s[i..].starts_with(" / ") {
                        if !buf.trim().is_empty() {
                            tokens.push((pos, false, buf.trim().to_string()));
                        }
                        buf.clear();
                        i += " / ".len();
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
            let mut expr = parse_postfix(tokens[0].2.trim())?;
            for i in 1..tokens.len() {
                let rhs = parse_postfix(tokens[i].2.trim())?;
                if tokens[i - 1].1 {
                    // Previous token marked as times
                    expr = Expr::Times(Box::new(expr), Box::new(rhs));
                } else {
                    expr = Expr::DividedBy(Box::new(expr), Box::new(rhs));
                }
            }
            return Ok(expr);
        }
    }

    parse_postfix(s)
}

fn parse_postfix(s: &str) -> Result<Expr> {
    let s = s.trim();
    // Check for indexing: expr[index]
    // Find the rightmost '[' at depth 0 (not inside strings or nested brackets/parens)
    let mut in_str = false;
    let mut depth = 0i32;
    let mut last_bracket = None;

    for (i, ch) in s.char_indices() {
        if ch == '"' {
            in_str = !in_str;
            continue;
        }
        if !in_str {
            if ch == '(' || ch == '[' || ch == '{' {
                if depth == 0 && ch == '[' {
                    last_bracket = Some(i);
                }
                depth += 1;
            } else if ch == ')' || ch == ']' || ch == '}' {
                depth -= 1;
            }
        }
    }

    // If we found a '[' at top level and the expression ends with ']'
    if let Some(bracket_pos) = last_bracket {
        if s.ends_with(']') && bracket_pos > 0 {
            let base = &s[..bracket_pos];
            let index_expr = &s[bracket_pos + 1..s.len() - 1];

            // Recursively parse both parts
            let base_expr = parse_postfix(base.trim())?;
            let index = parse_expr(index_expr.trim())?;

            return Ok(Expr::Index(Box::new(base_expr), Box::new(index)));
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
        if bytes[i] == b'"' {
            in_str = !in_str;
            buf.push('"');
            i += 1;
            continue;
        }
        if !in_str {
            if bytes[i] == b'(' || bytes[i] == b'[' || bytes[i] == b'{' {
                depth += 1;
                buf.push(bytes[i] as char);
                i += 1;
                continue;
            }
            if bytes[i] == b')' || bytes[i] == b']' || bytes[i] == b'}' {
                depth -= 1;
                buf.push(bytes[i] as char);
                i += 1;
                continue;
            }
            if depth == 0 && s[i..].starts_with(delim) {
                out.push(buf.trim().to_string());
                buf.clear();
                i += delim.len();
                continue;
            }
        }
        buf.push(s[i..].chars().next().unwrap());
        i += s[i..].chars().next().unwrap().len_utf8();
    }
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
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
        if ch == '"' {
            in_str = !in_str;
            buf.push(ch);
            i += ch.len_utf8();
            continue;
        }
        if !in_str {
            if ch == '(' || ch == '[' || ch == '{' {
                depth += 1;
                buf.push(ch);
                i += 1;
                continue;
            }
            if ch == ')' || ch == ']' || ch == '}' {
                depth -= 1;
                buf.push(ch);
                i += 1;
                continue;
            }
            if depth == 0 {
                let mut matched = None;
                for d in delims {
                    if s[i..].starts_with(d) {
                        matched = Some(*d);
                        break;
                    }
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
    if !buf.trim().is_empty() {
        out.push(buf.trim().to_string());
    }
    out
}

fn split_once_top_level<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut in_str = false;
    let mut depth = 0i32;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + pat.len() <= bytes.len() {
        let ch = s[i..].chars().next().unwrap();
        if ch == '"' {
            in_str = !in_str;
            i += ch.len_utf8();
            continue;
        }
        if !in_str {
            if ch == '(' || ch == '[' || ch == '{' {
                depth += 1;
                i += 1;
                continue;
            }
            if ch == ')' || ch == ']' || ch == '}' {
                depth -= 1;
                i += 1;
                continue;
            }
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

// CmpOp provided by ast

// Helper: parse a comma/" and " separated list of expressions at top level.
fn parse_items_comma_or_and(s: &str) -> Result<Vec<Expr>> {
    let parts = split_top_level_multi(s, &[",", " and "]);
    let mut out = Vec::new();
    for p in parts {
        let t = p.trim();
        if t.is_empty() {
            continue;
        }
        out.push(parse_expr(t)?);
    }
    Ok(out)
}

fn parse_term(s: &str) -> Result<Expr> {
    let s = s.trim();
    if s.is_empty() {
        return Err(anyhow!("Empty expression"));
    }

    // Phrasal list literals (immutable/mutable): Make a (mutable) list of 1, 2 and 3
    if let Some(rest) = strip_prefix_ci(s, "Make a mutable list of ") {
        let items = if rest.trim().is_empty() {
            vec![]
        } else {
            parse_items_comma_or_and(rest)?
        };
        // TODO: track mutability; for now, same ListLit representation.
        return Ok(Expr::ListLit(items));
    }
    if let Some(rest) = strip_prefix_ci(s, "Make a list of ") {
        let items = if rest.trim().is_empty() {
            vec![]
        } else {
            parse_items_comma_or_and(rest)?
        };
        return Ok(Expr::ListLit(items));
    }

    // Phrasal dictionary literals (immutable/mutable): Make a (mutable) dictionary with "a" as 1 and "b" as 2
    if let Some(rest) = strip_prefix_ci(s, "Make a mutable dictionary with ") {
        let mut pairs = Vec::new();
        let mut r = rest.trim();
        while !r.is_empty() {
            // split key and remainder by ' as ' (or legacy ' set to ')
            let (kpart, after_key) = if let Some((k, a)) = split_once_top_level(r, " as ") {
                (k, a)
            } else if let Some((k, a)) = split_once_top_level(r, " set to ") {
                (k, a)
            } else {
                return Err(anyhow!("Expected 'as' in dictionary literal item"));
            };
            let kstr = extract_quoted(kpart.trim())
                .ok_or_else(|| anyhow!("Expected quoted key in dictionary literal"))?;
            // find next delimiter (either ' and ' or ',') at top level to terminate the value expression
            let (vpart, rest_after_val) =
                if let Some((v, after)) = split_once_top_level(after_key, " and ") {
                    (v.trim(), Some(after))
                } else if let Some((v, after)) = split_once_top_level(after_key, ",") {
                    (v.trim(), Some(after))
                } else {
                    (after_key.trim(), None)
                };
            let vexpr = parse_expr(vpart)?;
            pairs.push((kstr, vexpr));
            r = match rest_after_val {
                Some(a) => a.trim(),
                None => "",
            };
        }
        // TODO: track mutability; for now, same DictLit representation.
        return Ok(Expr::DictLit(pairs));
    }
    if let Some(rest) = strip_prefix_ci(s, "Make a dictionary with ") {
        let mut pairs = Vec::new();
        let mut r = rest.trim();
        while !r.is_empty() {
            let (kpart, after_key) = if let Some((k, a)) = split_once_top_level(r, " as ") {
                (k, a)
            } else if let Some((k, a)) = split_once_top_level(r, " set to ") {
                (k, a)
            } else {
                return Err(anyhow!("Expected 'as' in dictionary literal item"));
            };
            let kstr = extract_quoted(kpart.trim())
                .ok_or_else(|| anyhow!("Expected quoted key in dictionary literal"))?;
            let (vpart, rest_after_val) =
                if let Some((v, after)) = split_once_top_level(after_key, " and ") {
                    (v.trim(), Some(after))
                } else if let Some((v, after)) = split_once_top_level(after_key, ",") {
                    (v.trim(), Some(after))
                } else {
                    (after_key.trim(), None)
                };
            let vexpr = parse_expr(vpart)?;
            pairs.push((kstr, vexpr));
            r = match rest_after_val {
                Some(a) => a.trim(),
                None => "",
            };
        }
        return Ok(Expr::DictLit(pairs));
    }

    // Bracket list literals are NOT supported - use phrasal syntax
    if s.starts_with('[') && s.ends_with(']') {
        return Err(anyhow!("Bracket list literals '[]' are not supported. Use: Make a list of ..."));
    }

    // Brace dict literals are NOT supported - use phrasal syntax
    if s.starts_with('{') && s.ends_with('}') {
        return Err(anyhow!("Brace dictionary literals '{{}}' are not supported. Use: Make a dictionary with ..."));
    }

    // Phrasal built-in expressions (case-insensitive)
    if let Some(rest) = P::strip_prefix_ci(s, P::P_COUNT_OF) {
        return Ok(Expr::CountOf(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_TOTAL_OF) {
        return Ok(Expr::TotalOf(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_SMALLEST_IN) {
        return Ok(Expr::SmallestIn(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_LARGEST_IN) {
        return Ok(Expr::LargestIn(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ABS_OF) {
        return Ok(Expr::AbsoluteValueOf(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ROUND_DOWN) {
        return Ok(Expr::RoundDown(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ROUND_UP) {
        return Ok(Expr::RoundUp(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ROUND) {
        return Ok(Expr::Round(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_MAKE_UPPER) {
        return Ok(Expr::MakeUppercase(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_MAKE_LOWER) {
        return Ok(Expr::MakeLowercase(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_TRIM_FROM) {
        return Ok(Expr::TrimSpaces(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_FIRST_IN) {
        return Ok(Expr::FirstIn(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_LAST_IN) {
        return Ok(Expr::LastIn(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_REVERSE_OF) {
        return Ok(Expr::ReverseOf(Box::new(parse_expr(rest)?)));
    }
    // Aliases for friendliness
    if let Some(rest) = P::strip_prefix_ci(s, P::P_REVERSE_ALIAS) {
        // alias of "reverse of"
        return Ok(Expr::ReverseOf(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_CLEAN_ALIAS) {
        // alias of "trim spaces from"
        return Ok(Expr::TrimSpaces(Box::new(parse_expr(rest)?)));
    }
    // New aliases
    if let Some(rest) = P::strip_prefix_ci(s, P::P_SIZE_OF) {
        return Ok(Expr::CountOf(Box::new(parse_expr(rest)?)));
    }

    // Phrasal binary built-ins: join <list> with <sep>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JOIN) {
        if let Some((lhs, rhs)) = split_once_top_level(rest, P::P_JOIN_WITH) {
            let a = parse_expr(lhs.trim())?;
            let b = parse_expr(rhs.trim())?;
            return Ok(Expr::JoinWith(Box::new(a), Box::new(b)));
        }
    }
    // split <text> by <sep>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_SPLIT) {
        if let Some((lhs, rhs)) = split_once_top_level(rest, P::P_SPLIT_BY) {
            let a = parse_expr(lhs.trim())?;
            let b = parse_expr(rhs.trim())?;
            return Ok(Expr::SplitBy(Box::new(a), Box::new(b)));
        }
    }
    // separate <text> by <sep> (alias of split)
    if let Some(rest) = P::strip_prefix_ci(s, P::P_SEPARATE) {
        if let Some((lhs, rhs)) = split_once_top_level(rest, P::P_SPLIT_BY) {
            let a = parse_expr(lhs.trim())?;
            let b = parse_expr(rhs.trim())?;
            return Ok(Expr::SplitBy(Box::new(a), Box::new(b)));
        }
    }

    // Collection operations: contains <item> in <collection>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_CONTAINS) {
        if let Some((item, collection)) = split_once_top_level(rest, P::P_CONTAINS_IN) {
            let item_expr = parse_expr(item.trim())?;
            let coll_expr = parse_expr(collection.trim())?;
            return Ok(Expr::Contains(Box::new(item_expr), Box::new(coll_expr)));
        }
    }
    // remove <item> from <list>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_REMOVE) {
        if let Some((item, list)) = split_once_top_level(rest, P::P_REMOVE_FROM) {
            let item_expr = parse_expr(item.trim())?;
            let list_expr = parse_expr(list.trim())?;
            return Ok(Expr::Remove(Box::new(item_expr), Box::new(list_expr)));
        }
    }
    // append <item> to <list>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_APPEND) {
        if let Some((item, list)) = split_once_top_level(rest, P::P_APPEND_TO) {
            let item_expr = parse_expr(item.trim())?;
            let list_expr = parse_expr(list.trim())?;
            return Ok(Expr::Append(Box::new(item_expr), Box::new(list_expr)));
        }
    }
    // insert <item> at <index> in <list>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_INSERT) {
        // First split by " at " to get item and "index in list"
        if let Some((item, rest2)) = split_once_top_level(rest, P::P_INSERT_AT) {
            // Then split by " in " to get index and list
            if let Some((index, list)) = split_once_top_level(rest2, P::P_INSERT_IN) {
                let item_expr = parse_expr(item.trim())?;
                let index_expr = parse_expr(index.trim())?;
                let list_expr = parse_expr(list.trim())?;
                return Ok(Expr::InsertAt(
                    Box::new(item_expr),
                    Box::new(index_expr),
                    Box::new(list_expr),
                ));
            }
        }
    }

    // File I/O operations
    // read file at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_READ_FILE) {
        return Ok(Expr::ReadFile(Box::new(parse_expr(rest)?)));
    }
    // read lines from file at <path> or read lines from <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_READ_LINES) {
        return Ok(Expr::ReadLines(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_READ_LINES_ALT) {
        return Ok(Expr::ReadLines(Box::new(parse_expr(rest)?)));
    }
    // file exists at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_FILE_EXISTS) {
        return Ok(Expr::FileExists(Box::new(parse_expr(rest)?)));
    }
    // delete file at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_DELETE_FILE) {
        return Ok(Expr::DeleteFile(Box::new(parse_expr(rest)?)));
    }
    // create directory at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_CREATE_DIR) {
        return Ok(Expr::CreateDir(Box::new(parse_expr(rest)?)));
    }
    // list files in directory at <path> or list files in <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_LIST_DIR_ALT) {
        return Ok(Expr::ListDir(Box::new(parse_expr(rest)?)));
    }
    if let Some(rest) = P::strip_prefix_ci(s, P::P_LIST_DIR) {
        return Ok(Expr::ListDir(Box::new(parse_expr(rest)?)));
    }
    // write <content> to file at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_WRITE_FILE) {
        if let Some((content, path)) = split_once_top_level(rest, P::P_WRITE_TO_FILE) {
            let content_expr = parse_expr(content.trim())?;
            let path_expr = parse_expr(path.trim())?;
            return Ok(Expr::WriteFile(Box::new(content_expr), Box::new(path_expr)));
        }
    }
    // append <content> to file at <path>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_APPEND_FILE) {
        if let Some((content, path)) = split_once_top_level(rest, P::P_APPEND_TO_FILE) {
            let content_expr = parse_expr(content.trim())?;
            let path_expr = parse_expr(path.trim())?;
            return Ok(Expr::AppendFile(
                Box::new(content_expr),
                Box::new(path_expr),
            ));
        }
    }
    // copy file from <source> to <dest>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_COPY_FILE) {
        if let Some((source, dest)) = split_once_top_level(rest, P::P_COPY_TO) {
            let source_expr = parse_expr(source.trim())?;
            let dest_expr = parse_expr(dest.trim())?;
            return Ok(Expr::CopyFile(Box::new(source_expr), Box::new(dest_expr)));
        }
    }
    // move file from <source> to <dest>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_MOVE_FILE) {
        if let Some((source, dest)) = split_once_top_level(rest, P::P_COPY_TO) {
            let source_expr = parse_expr(source.trim())?;
            let dest_expr = parse_expr(dest.trim())?;
            return Ok(Expr::MoveFile(Box::new(source_expr), Box::new(dest_expr)));
        }
    }

    // JSON operations
    // parse json from <string>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_PARSE_JSON) {
        return Ok(Expr::ParseJson(Box::new(parse_expr(rest)?)));
    }
    // convert to json <value>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_TO_JSON) {
        return Ok(Expr::ToJson(Box::new(parse_expr(rest)?)));
    }
    // convert to pretty json <value>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JSON_PRETTY) {
        return Ok(Expr::ToJsonPretty(Box::new(parse_expr(rest)?)));
    }
    // json length of <value>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JSON_LENGTH) {
        return Ok(Expr::JsonLength(Box::new(parse_expr(rest)?)));
    }
    // new json object
    if s.eq_ignore_ascii_case(P::P_NEW_JSON_OBJECT) {
        return Ok(Expr::NewJsonObject);
    }
    // new json array
    if s.eq_ignore_ascii_case(P::P_NEW_JSON_ARRAY) {
        return Ok(Expr::NewJsonArray);
    }
    // get <key> from json <object>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JSON_GET) {
        if let Some((key, json)) = split_once_top_level(rest, P::P_JSON_FROM) {
            let key_expr = parse_expr(key.trim())?;
            let json_expr = parse_expr(json.trim())?;
            return Ok(Expr::JsonGet(Box::new(json_expr), Box::new(key_expr)));
        }
    }
    // set <key> in json <object> to <value>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JSON_SET) {
        if let Some((key_part, rest2)) = split_once_top_level(rest, P::P_JSON_IN) {
            if let Some((json_part, value_part)) = split_once_top_level(rest2, P::P_JSON_TO) {
                let key_expr = parse_expr(key_part.trim())?;
                let json_expr = parse_expr(json_part.trim())?;
                let value_expr = parse_expr(value_part.trim())?;
                return Ok(Expr::JsonSet(
                    Box::new(json_expr),
                    Box::new(key_expr),
                    Box::new(value_expr),
                ));
            }
        }
    }
    // push <item> to json <array>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_JSON_PUSH) {
        if let Some((item, json)) = split_once_top_level(rest, P::P_JSON_PUSH_TO) {
            let item_expr = parse_expr(item.trim())?;
            let json_expr = parse_expr(json.trim())?;
            return Ok(Expr::JsonPush(Box::new(json_expr), Box::new(item_expr)));
        }
    }

    // Error operations
    // error message of <error>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ERROR_MESSAGE) {
        return Ok(Expr::ErrorMessage(Box::new(parse_expr(rest)?)));
    }
    // error type of <error>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_ERROR_TYPE) {
        return Ok(Expr::ErrorType(Box::new(parse_expr(rest)?)));
    }
    // error of type <type> with message <message>
    if let Some(rest) = P::strip_prefix_ci(s, "error of type ") {
        if let Some((type_part, message_part)) = split_once_top_level(rest, P::P_WITH_MESSAGE) {
            let error_type = type_part.trim();
            // Remove quotes from type if present
            let error_type_clean = if (error_type.starts_with('"') && error_type.ends_with('"'))
                || (error_type.starts_with('\'') && error_type.ends_with('\''))
            {
                &error_type[1..error_type.len() - 1]
            } else {
                error_type
            };
            let message_expr = parse_expr(message_part.trim())?;
            return Ok(Expr::NewError {
                error_type: error_type_clean.to_string(),
                message: Box::new(message_expr),
            });
        }
    }

    // Web Framework Expressions
    // create web server on port <port>
    if let Some(rest) = P::strip_prefix_ci(s, "create web server on port ") {
        return Ok(Expr::CreateWebServer(Box::new(parse_expr(rest)?)));
    }
    // html response with <content>
    if let Some(rest) = P::strip_prefix_ci(s, "html response with ") {
        return Ok(Expr::HtmlResponse(Box::new(parse_expr(rest)?)));
    }
    // json response with <data>
    if let Some(rest) = P::strip_prefix_ci(s, "json response with ") {
        // Check if it has "and status"
        if let Some((data_part, status_part)) = split_once_top_level(rest, " and status ") {
            let data_expr = parse_expr(data_part.trim())?;
            let status_expr = parse_expr(status_part.trim())?;
            return Ok(Expr::JsonResponseStatus(
                Box::new(data_expr),
                Box::new(status_expr),
            ));
        }
        return Ok(Expr::JsonResponse(Box::new(parse_expr(rest)?)));
    }
    // render template <template> with <data>
    if let Some(rest) = P::strip_prefix_ci(s, "render template ") {
        if let Some((template_part, data_part)) = split_once_top_level(rest, " with ") {
            let template_expr = parse_expr(template_part.trim())?;
            let data_expr = parse_expr(data_part.trim())?;
            return Ok(Expr::RenderTemplate(
                Box::new(template_expr),
                Box::new(data_expr),
            ));
        }
    }
    // get path parameter <name>
    if let Some(rest) = P::strip_prefix_ci(s, P::P_GET_PATH_PARAM) {
        let param_expr = parse_expr(rest.trim())?;
        return Ok(Expr::GetPathParam(Box::new(param_expr)));
    }
    // error response with status <status> and message <message>
    if let Some(rest) = P::strip_prefix_ci(s, "error response with status ") {
        if let Some((status_part, message_part)) = split_once_top_level(rest, " and message ") {
            let status_expr = parse_expr(status_part.trim())?;
            let message_expr = parse_expr(message_part.trim())?;
            return Ok(Expr::ErrorResponse(
                Box::new(status_expr),
                Box::new(message_expr),
            ));
        }
    }

    // String literal
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
    {
        return Ok(Expr::Str(s[1..s.len() - 1].to_string()));
    }
    // Booleans
    if s.eq_ignore_ascii_case("True") {
        return Ok(Expr::Bool(true));
    }
    if s.eq_ignore_ascii_case("False") {
        return Ok(Expr::Bool(false));
    }
    // Null/None
    if s.eq_ignore_ascii_case("Null") || s.eq_ignore_ascii_case("Nothing") || s == "None" {
        return Ok(Expr::Null);
    }
    // Legacy list syntax is NOT supported
    if let Some(_rest) = strip_prefix_ci(s, "List contains ") {
        return Err(anyhow!("Legacy 'List contains' is not supported. Use: Make a list of ..."));
    }
    // Legacy dictionary syntax is NOT supported
    if let Some(_rest) = strip_prefix_ci(s, "Dictionary contains ") {
        return Err(anyhow!("Legacy 'Dictionary contains' is not supported. Use: Make a dictionary with ..."));
    }
    // Number
    if let Ok(n) = s.parse::<f64>() {
        return Ok(Expr::Num(n));
    }
    // Phrasal call: name with args
    if let Some((name, after)) = split_ident(s) {
        let after = after.trim_start();
        if let Some(rest) = after.strip_prefix("with ") {
            let args = if rest.trim().is_empty() {
                vec![]
            } else {
                parse_arg_list_multi(rest, true)?
            };
            return Ok(Expr::Call { name, args });
        }
    }
    // Call form: name(args) OR grouping: (expr)
    if let Some(idx) = s.find('(') {
        if s.ends_with(')') {
            let name = s[..idx].trim();
            let args_str = &s[idx + 1..s.len() - 1];
            
            // If name is empty, this is a grouping expression: (expr)
            if name.is_empty() {
                let inner = args_str.trim();
                if inner.is_empty() {
                    return Err(anyhow!("Empty parentheses () are not allowed"));
                }
                // Parse the grouped expression recursively
                return parse_expr(inner);
            }
            
            // Otherwise, it's a function call: name(args)
            let args = if args_str.trim().is_empty() {
                vec![]
            } else {
                parse_arg_list(args_str)?
            };
            return Ok(Expr::Call {
                name: name.to_string(),
                args,
            });
        }
    }
    // Identifier
    if let Some((id, rest)) = split_ident(s) {
        if rest.trim().is_empty() {
            return Ok(Expr::Ident(id));
        }
    }
    let error_msg = format!("Could not parse expression: {}", s);
    Err(anyhow!("{}", suggest_fix(&error_msg, s)))
}

fn extract_quoted(s: &str) -> Option<String> {
    let st = s.trim();
    if let Some(stripped) = st.strip_prefix('"') {
        if let Some(pos) = stripped.find('"') {
            return Some(stripped[..pos].to_string());
        }
    }
    None
}

fn extract_quoted_and_rest(s: &str) -> Option<(String, &str)> {
    let st = s.trim_start();
    if !st.starts_with('"') {
        return None;
    }
    if let Some(pos) = st[1..].find('"') {
        let end = 1 + pos;
        let value = st[1..end].to_string();
        let rest = &st[end + 1..];
        Some((value, rest))
    } else {
        None
    }
}

fn parse_exposing_list(s: &str) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("Expected at least one symbol after 'exposing'"));
    }

    // Allow comma separated list with optional 'and'
    let parts = split_top_level_multi(trimmed, &[",", " and "]);
    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((name, rest)) = split_ident(trimmed) {
            if !rest.trim().is_empty() {
                return Err(anyhow!("Invalid symbol name in exposing list"));
            }
            if !names.contains(&name) {
                names.push(name);
            }
        } else {
            return Err(anyhow!("Invalid symbol name in exposing list"));
        }
    }
    if names.is_empty() {
        return Err(anyhow!("Expected at least one symbol after 'exposing'"));
    }
    Ok(names)
}
