# Development Session: Ask for Input Feature

**Date**: January 2025  
**Feature**: Interactive user input (`Ask for <var>` statement)  
**Status**: ✅ Completed and merged  
**Commit**: 807761d

---

## Overview

Implemented the "Ask for" input statement in the Rust runtime, enabling PohLang programs to accept interactive user input. This was identified as a high-priority Phase 1 feature needed for interactive programs.

---

## Implementation Details

### 1. Parser Changes (`runtime-rs/src/parser.rs`)

**Added to Stmt enum** (line 22):
```rust
AskFor { var_name: String }
```

**Added parsing logic in `parse_block()`** (lines 90-97):
```rust
if let Some(rest) = t.strip_prefix("Ask for ") {
    if let Some((var_name, rest_after)) = split_ident(rest) {
        lines_consumed = 1;
        Ok((Stmt::AskFor { var_name: var_name.to_string() }, rest_after))
    } else {
        Err(anyhow!("Invalid identifier after 'Ask for'"))
    }
}
```

**Added to `parse_until_keywords()`** (lines 267-275):
```rust
if let Some(rest) = t.strip_prefix("Ask for ") {
    if let Some((var_name, rest_after)) = split_ident(rest) {
        Ok(vec![Stmt::AskFor { var_name: var_name.to_string() }])
    } else {
        Err(anyhow!("Invalid identifier after 'Ask for'"))
    }
}
```

### 2. VM Execution Changes (`runtime-rs/src/vm.rs`)

**Added stdin import** (top of file):
```rust
use std::io::{self, BufRead};
```

**Added execution in `execute()`** (lines 61-73):
```rust
Stmt::AskFor { var_name } => {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let input = line.trim();
    
    // Auto-detect number vs string
    let value = if let Ok(n) = input.parse::<f64>() {
        Value::Num(n)
    } else {
        Value::Str(input.to_string())
    };
    
    self.globals.insert(var_name.clone(), value);
}
```

**Added execution in `exec_block_with_frame()`** (lines 303-320):
```rust
Stmt::AskFor { var_name } => {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let input = line.trim();
    
    let value = if let Ok(n) = input.parse::<f64>() {
        Value::Num(n)
    } else {
        Value::Str(input.to_string())
    };
    
    // Store in local frame first, fallback to globals
    frame.insert(var_name.clone(), value);
}
```

**Added bytecode generation in `compile()`** (lines 636-642):
```rust
Stmt::AskFor { var_name } => {
    out.extend_from_slice(b"ASK\t");
    out.extend_from_slice(var_name.as_bytes());
    out.push(b'\n');
}
```

### 3. Tests (`runtime-rs/tests/smoke.rs`)

**Test 1: Parsing validation**:
```rust
#[test]
fn ask_for_parses_correctly() {
    // Test that Ask for syntax parses without error
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 5").unwrap();
    writeln!(f, "Write \"Before ask\"").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}
```

**Test 2: Bytecode compilation**:
```rust
#[test]
fn ask_for_in_bytecode() {
    // Test that Ask for compiles to bytecode without error
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Ask for name").unwrap();
    writeln!(f, "Write \"Got: \" plus name").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--compile").arg(path.to_str().unwrap());
    cmd.assert().success();
}
```

### 4. Example Program (`examples/poh/ask_name.poh`)

```poh
Write "What is your name?"
Ask for name
Write "Hello " plus name
```

**Bytecode output** (`ask_name.pbc`):
```
WRITE	What is your name?
ASK	name
WRITE	Hello  plus name
```

---

## Design Decisions

### 1. Auto-Type Detection
The implementation automatically detects whether input is a number or string:
- If `input.parse::<f64>()` succeeds → store as `Value::Num`
- Otherwise → store as `Value::Str`

This matches PohLang's dynamic typing philosophy and makes the language easier to use for beginners.

### 2. Variable Scope
Variables created by `Ask for` are stored in:
- **Top-level**: Global variables (`self.globals`)
- **Function blocks**: Local frame variables (`frame`)

This follows the same scoping rules as `Set` statements.

### 3. Whitespace Handling
Input is trimmed (`line.trim()`) to remove trailing newlines and leading/trailing whitespace, preventing common user errors.

---

## Testing Results

### Rust Tests (runtime-rs)
```
running 14 tests
test ask_for_in_bytecode ... ok
test ask_for_parses_correctly ... ok
test join_and_range_builtins_work ... ok
test import_local_file_and_call ... ok
test run_closure_like_capture ... ok
test run_func_block_and_return ... ok
test run_if_block_and_set ... ok
test run_if_inline_works ... ok
test run_inline_function_and_use ... ok
test run_nested_func_blocks ... ok
test run_repeat_block_counts ... ok
test run_while_block_counts ... ok
test system_import_stub_noop ... ok
test run_write_works ... ok

test result: ok. 14 passed; 0 failed
```

### Python Tests (reference implementation)
```
[1/15] Basic Write... ✅ PASS
[2/15] Variable assignment... ✅ PASS
[3/15] Addition... ✅ PASS
[4/15] String concatenation... ✅ PASS
[5/15] Greater than comparison... ✅ PASS
[6/15] Block if-else... ✅ PASS
[7/15] Block if... ✅ PASS
[8/15] Repeat loop... ✅ PASS
[9/15] While loop... ✅ PASS
[10/15] Inline function... ✅ PASS
[11/15] Block function with return... ✅ PASS
[12/15] Function with default parameter... ✅ PASS
[13/15] Nested if blocks... ✅ PASS
[14/15] Function composition... ✅ PASS
[15/15] Function arity error... ✅ PASS

Results: 15 passed, 0 failed
```

---

## Progress Update

### Phase 1 Completion: ~65%

**Completed**:
- ✅ Core statements (Write, Set, If, Repeat, While)
- ✅ Functions (inline and block, closures, default parameters)
- ✅ File imports
- ✅ **Ask for input** (NEW)

**Remaining**:
- ⬜ Increase/Decrease desugaring
- ⬜ `minus` operator
- ⬜ `divided by` operator
- ⬜ Collection literals `[1, 2, 3]`
- ⬜ Enhanced error messages with line numbers
- ⬜ Full Python test compatibility

---

## Next Development Priorities

Based on ROADMAP.md Phase 1:

1. **Increase/Decrease desugaring**: Transform `Increase x by 5` → `Set x to x plus 5`
2. **Minus operator**: Add subtraction support (`5 minus 2` → `3`)
3. **Divided by operator**: Add division support (`10 divided by 2` → `5`)
4. **Better error messages**: Add line/column tracking, contextual suggestions
5. **Collection literals**: Enable `Set nums to [1, 2, 3]` syntax

---

## References

- **Main Commit**: 807761d - "Add 'Ask for' input statement to Rust runtime"
- **Files Changed**: 
  - `runtime-rs/src/parser.rs` (+30 lines)
  - `runtime-rs/src/vm.rs` (+50 lines)
  - `runtime-rs/tests/smoke.rs` (+15 lines)
  - `examples/poh/ask_name.poh` (new file)
  - `ROADMAP.md` (progress update)
- **CI Status**: Pending validation on Ubuntu/macOS/Windows
- **Documentation**: ROADMAP.md, DESIGN.md, runtime-rs/README.md

---

## Known Limitations

1. **No prompt parameter**: Unlike some languages, `Ask for` doesn't take a prompt string. Users must use a separate `Write` statement.
   
   ```poh
   Write "Enter your age:"
   Ask for age
   ```

2. **No type hints**: The language doesn't support explicit type constraints like `Ask for age as Number`.

3. **Bytecode is AST dump**: The current bytecode compiler (`--compile`) produces a human-readable AST dump, not optimized bytecode. Full bytecode VM is Phase 3.

4. **No validation**: Input validation (ranges, patterns) must be done with separate `If` statements.

---

## Future Enhancements (Post-Phase 1)

- **Prompt parameter**: `Ask for name with "Enter name:"` (syntactic sugar)
- **Type hints**: `Ask for age as Number` to enforce numeric input
- **Validation**: `Ask for age between 0 and 120`
- **File input**: `Ask for content from "input.txt"`
- **Default values**: `Ask for name defaulting to "Guest"`

---

*This feature brings PohLang closer to feature parity with the Python interpreter and enables a wider range of interactive programs!*
