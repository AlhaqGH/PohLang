# Error Location Reporting Status

## Current State (v0.5.2)

### ✅ Implemented Features

1. **Filename Reporting**
   - VM tracks current executing file via `current_file` field
   - `set_current_file()` method allows setting the filename
   - `error_with_location()` helper adds "\n  in file: {filename}" to errors
   - `main.rs` passes filename to VM when executing with `--run`

2. **Updated Error Sites**
   - Division by zero (4 locations: bytecode + 3 eval methods)
   - Throw statements
   - All now show "in file: filename.poh"

3. **Example Output**
   ```
   Error: Division by zero
     in file: test_error_location.poh
   ```
   
   ```
   Error: [RuntimeError] Error occurred: a runtime error - Something went wrong!
     in file: test_throw_location.poh
   ```

### ❌ Not Implemented

1. **Line Numbers**
   - AST nodes don't store source positions (line, column)
   - Would require fundamental refactor:
     - Create `Span { file, line, column }` struct
     - Wrap every AST node: `Expr::Plus(Span, Box<Expr>, Box<Expr>)`
     - Update entire parser to track positions
     - Update all VM pattern matches
   - **Estimated effort**: 20+ hours, 2000+ lines across multiple files

2. **Column Numbers**
   - Same issue as line numbers - requires AST refactor

3. **Comprehensive Coverage**
   - ~50 `bail!()` call sites not yet updated:
     - File I/O operations (lines 744, 748, 758, 762, 766, 776, 780, 784, 791, 799, 803, 810, 814, 821, 825, 832, 836, 844, 848, 852)
     - JSON operations
     - String operations
     - Type errors
   - These could be updated incrementally to use `error_with_location()`

## Technical Details

### VM Infrastructure

```rust
pub struct Vm {
    // ... other fields ...
    pub current_file: String,
}

impl Vm {
    pub fn set_current_file(&mut self, file: String) {
        self.current_file = file;
    }
    
    pub fn current_file(&self) -> &str {
        &self.current_file
    }
    
    fn error_with_location(&self, message: impl Into<String>) -> anyhow::Error {
        let msg = message.into();
        if self.current_file.is_empty() {
            anyhow!(msg)
        } else {
            anyhow!("{}\n  in file: {}", msg, self.current_file)
        }
    }
}
```

### Usage Pattern

```rust
// Before:
return Err(anyhow!("Division by zero"));

// After:
return Err(self.error_with_location("Division by zero"));
```

### Entry Point Integration

```rust
// main.rs line 41
vm.set_current_file(args.input.display().to_string());
let result = vm.execute(&program)?;
```

## Future Work

### Short-term (Can be done incrementally)

1. **Update remaining bail!() sites**
   - File I/O operations
   - JSON operations  
   - String/collection operations
   - Type validation errors
   - Estimated: 2-3 hours

2. **Add context to caught errors**
   - When catching errors in try/catch, preserve location info
   - Test with nested try/catch blocks

### Long-term (Major refactor required)

1. **Add source position tracking to AST**
   - Define `Span` or `SourceLocation` struct
   - Add to all Expr and Stmt variants
   - Update parser to track positions during parsing
   - Update VM to include line/column in errors
   - Estimated: 20-30 hours

2. **Enhanced stack traces**
   - Show call chain with line numbers
   - Include file locations for each frame
   - Format like: "at function_name (file.poh:line:column)"

## Recommendations

For **Phase 5 completion**, the current filename-only approach is acceptable:
- ✅ Errors are actionable (users know which file has the issue)
- ✅ Consistent with natural language error messages
- ✅ No breaking changes to existing functionality

For **future releases**, consider:
1. Incremental: Update remaining `bail!()` sites (low effort, high consistency)
2. Major: Add full source position tracking (high effort, significant improvement)

The decision depends on:
- User feedback (are line numbers critical?)
- Development priorities (other features vs. error reporting)
- Breaking change tolerance (AST refactor affects everything)
