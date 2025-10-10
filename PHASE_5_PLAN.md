# Phase 5: Error Handling System - Design Document

**Status**: Planning Phase  
**Created**: October 10, 2025  
**Goal**: Add comprehensive error handling to PohLang with try/catch blocks, custom error types, stack traces, and error recovery mechanisms.

## Current State Analysis

### Existing Error Handling (v0.5.3)
PohLang currently uses Rust's `anyhow` crate for error handling:
- **Error propagation**: `Result<T>` types throughout VM
- **Error creation**: `anyhow!()` and `bail!()` macros
- **Error enhancement**: `enhance_error()` function adds contextual hints
- **No user-facing error handling**: Errors bubble up and crash the program

### Limitations of Current System
1. ❌ **No try/catch**: Users cannot handle errors gracefully
2. ❌ **No custom errors**: Cannot create domain-specific errors
3. ❌ **No stack traces**: Hard to debug when errors occur deep in call chains
4. ❌ **No error types**: Cannot distinguish between different error categories
5. ❌ **No recovery**: Programs crash on any error
6. ❌ **Poor user experience**: Rust error messages exposed to end users

## Design Goals

### 1. Natural Language Syntax
Error handling should feel natural and readable:
```pohlang
Try this:
    Set content to read file at "config.json"
    Set config to parse json from content
If error:
    Write line "Using default configuration"
    Set config to new json object
End try
```

### 2. Typed Errors
Support error types for fine-grained handling:
```pohlang
Try this:
    Set result to divide 10 by 0
If error of type "MathError":
    Write line "Cannot divide by zero"
If error of type "FileError":
    Write line "File not found"
End try
```

### 3. Error Information
Errors should carry useful information:
```pohlang
Try this:
    Set data to read file at "missing.txt"
If error as err:
    Write line "Error: " with error message of err
    Write line "Type: " with error type of err
End try
```

### 4. Finally Blocks
Cleanup code that always runs:
```pohlang
Try this:
    Set file to open file at "data.txt"
    Process file
If error:
    Write line "Processing failed"
Finally:
    Close file
End try
```

### 5. Custom Errors
Users can throw their own errors:
```pohlang
Define function validate age with param age:
    If age is less than 0:
        Throw error of type "ValidationError" with message "Age cannot be negative"
    End if
    Return age
End function
```

### 6. Stack Traces
Detailed error context for debugging:
```
RuntimeError: Division by zero
  at calculate_average (math.poh:15)
  at process_scores (student.poh:42)
  at main (main.poh:8)
```

## Technical Architecture

### 1. Error Type System

**New file: `runtime/src/stdlib/errors.rs`**
```rust
#[derive(Clone, Debug)]
pub struct PohError {
    pub kind: ErrorKind,
    pub message: String,
    pub stack_trace: Vec<StackFrame>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    RuntimeError,
    TypeError,
    MathError,
    FileError,
    JsonError,
    NetworkError,
    ValidationError,
    Custom(String),
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub function: String,
    pub file: String,
    pub line: usize,
}
```

### 2. AST Extensions

**Add to `runtime/src/parser/ast.rs`**:
```rust
pub enum Stmt {
    // ... existing variants
    TryCatch {
        try_block: Vec<Stmt>,
        catch_handlers: Vec<CatchHandler>,
        finally_block: Option<Vec<Stmt>>,
    },
    Throw(Expr), // throw error expression
}

pub struct CatchHandler {
    pub error_type: Option<String>, // None = catch all
    pub var_name: Option<String>,   // variable to bind error to
    pub block: Vec<Stmt>,
}

pub enum Expr {
    // ... existing variants
    ErrorMessage(Box<Expr>),  // "error message of X"
    ErrorType(Box<Expr>),      // "error type of X"
    NewError {                 // "error of type X with message Y"
        error_type: String,
        message: Box<Expr>,
    },
}
```

### 3. Phrasal Expressions

**Add to `runtime/src/parser/phrases.rs`**:
```rust
// Error handling control flow
pub const P_TRY: &str = "try this:";
pub const P_IF_ERROR: &str = "if error";
pub const P_OF_TYPE: &str = " of type ";
pub const P_AS: &str = " as ";
pub const P_FINALLY: &str = "finally:";
pub const P_END_TRY: &str = "end try";
pub const P_THROW: &str = "throw ";

// Error operations
pub const P_ERROR: &str = "error";
pub const P_ERROR_MESSAGE: &str = "error message of ";
pub const P_ERROR_TYPE: &str = "error type of ";
pub const P_WITH_MESSAGE: &str = " with message ";
```

### 4. VM Execution Changes

**Modify `runtime/src/vm/vm.rs`**:

#### Add Error Context to VM
```rust
pub struct VM {
    // ... existing fields
    call_stack: Vec<CallFrame>,
    current_file: String,
}

struct CallFrame {
    function_name: String,
    file: String,
    line: usize,
}
```

#### Error Value Type
```rust
enum Value {
    // ... existing variants
    Error(PohError),
}
```

#### Try/Catch Execution
```rust
fn execute_try_catch(&mut self, try_block: &[Stmt], handlers: &[CatchHandler], finally: &Option<Vec<Stmt>>) -> Result<()> {
    // Execute try block
    let try_result = self.execute_block(try_block);
    
    // If error occurred, find matching handler
    if let Err(e) = try_result {
        let error = convert_to_poh_error(e);
        
        for handler in handlers {
            if handler_matches(&handler, &error) {
                if let Some(var) = &handler.var_name {
                    self.scope.insert(var.clone(), Value::Error(error.clone()));
                }
                self.execute_block(&handler.block)?;
                break;
            }
        }
    }
    
    // Always execute finally block
    if let Some(finally_stmts) = finally {
        self.execute_block(finally_stmts)?;
    }
    
    Ok(())
}
```

### 5. Stack Trace Implementation

**Add to VM execution**:
```rust
impl VM {
    fn push_call_frame(&mut self, func_name: &str) {
        self.call_stack.push(CallFrame {
            function_name: func_name.to_string(),
            file: self.current_file.clone(),
            line: self.current_line,
        });
    }
    
    fn pop_call_frame(&mut self) {
        self.call_stack.pop();
    }
    
    fn build_stack_trace(&self) -> Vec<StackFrame> {
        self.call_stack.iter()
            .map(|frame| StackFrame {
                function: frame.function_name.clone(),
                file: frame.file.clone(),
                line: frame.line,
            })
            .collect()
    }
}
```

## Syntax Examples

### Example 1: Basic Try/Catch
```pohlang
Start Program

Try this:
    Set result to divide 10 by 0
If error:
    Write line "An error occurred!"
End try

End Program
```

### Example 2: Typed Error Handling
```pohlang
Start Program

Try this:
    Set content to read file at "data.txt"
    Set data to parse json from content
If error of type "FileError":
    Write line "File not found - using defaults"
    Set data to new json object
If error of type "JsonError":
    Write line "Invalid JSON - skipping"
End try

End Program
```

### Example 3: Error Information
```pohlang
Start Program

Try this:
    Set result to get "missing_key" from json data
If error as err:
    Write line "Error occurred:"
    Write line "  Message: " with error message of err
    Write line "  Type: " with error type of err
End try

End Program
```

### Example 4: Finally Block
```pohlang
Start Program

Set connection to null

Try this:
    Set connection to connect to database
    Process data from connection
If error:
    Write line "Processing failed"
Finally:
    If connection is not null:
        Close connection
    End if
End try

End Program
```

### Example 5: Custom Errors
```pohlang
Start Program

Define function validate email with param email:
    If email does not contain "@":
        Throw error of type "ValidationError" with message "Invalid email format"
    End if
    Return email
End function

Try this:
    Set user_email to validate email with "invalid-email"
If error of type "ValidationError" as err:
    Write line "Validation failed: " with error message of err
End try

End Program
```

### Example 6: Nested Try/Catch
```pohlang
Start Program

Try this:
    Write line "Outer try block"
    
    Try this:
        Set result to divide 5 by 0
    If error:
        Write line "Inner error handled"
        Throw error of type "RuntimeError" with message "Re-throwing error"
    End try
    
If error as err:
    Write line "Outer caught: " with error message of err
End try

End Program
```

### Example 7: Error Recovery with Retry
```pohlang
Start Program

Set max_retries to 3
Set attempt to 0
Set success to false

Repeat while attempt is less than max_retries and success is false:
    Set attempt to increase attempt by 1
    
    Try this:
        Write line "Attempt " with attempt
        Set data to fetch from url "https://api.example.com/data"
        Set success to true
    If error as err:
        Write line "Attempt failed: " with error message of err
        If attempt is less than max_retries:
            Write line "Retrying..."
        End if
    End try
End repeat

If success is false:
    Write line "All attempts failed"
End if

End Program
```

## Implementation Phases

### Phase 5.1: Core Error Infrastructure ✅
- [ ] Create `stdlib/errors.rs` with PohError, ErrorKind, StackFrame
- [ ] Add Error variant to Value enum
- [ ] Add call stack tracking to VM
- [ ] Implement stack trace building

### Phase 5.2: Basic Try/Catch 🎯
- [ ] Add TryCatch to AST
- [ ] Add try/catch parsing to parser
- [ ] Implement try/catch execution in VM
- [ ] Test basic error catching

### Phase 5.3: Error Types & Filtering
- [ ] Add error type matching in catch handlers
- [ ] Implement typed error catching
- [ ] Add error binding to variables
- [ ] Test error type filtering

### Phase 5.4: Error Operations
- [ ] Add ErrorMessage and ErrorType expressions
- [ ] Add NewError expression for custom errors
- [ ] Implement Throw statement
- [ ] Test custom error creation

### Phase 5.5: Finally Blocks
- [ ] Add finally block support to TryCatch
- [ ] Implement finally execution (always runs)
- [ ] Test finally with various scenarios

### Phase 5.6: Retrofit Existing Code
- [ ] Update file I/O errors to use PohError
- [ ] Update JSON errors to use PohError
- [ ] Update math errors to use PohError
- [ ] Ensure backward compatibility

### Phase 5.7: Testing & Examples
- [ ] Create comprehensive integration tests
- [ ] Create example programs demonstrating patterns
- [ ] Test edge cases (nested, multiple handlers, etc.)
- [ ] Performance testing

## Error Type Mapping

Map existing Rust errors to PohError types:

| Current Error | New Error Type | Example |
|--------------|----------------|---------|
| Division by zero | MathError | `divide 10 by 0` |
| Type mismatch | TypeError | `add "text" to 5` |
| File not found | FileError | `read file at "missing.txt"` |
| JSON parse error | JsonError | `parse json from "invalid"` |
| Index out of bounds | RuntimeError | `get item at 99 from list` |
| Function not defined | RuntimeError | `call undefined_function` |

## Testing Strategy

### Unit Tests (stdlib/errors.rs)
- Error creation and formatting
- Stack frame construction
- Error type matching
- Error message extraction

### Integration Tests (tests/errors.rs)
1. **test_basic_try_catch** - Simple try/catch with error
2. **test_try_catch_no_error** - Try block succeeds
3. **test_typed_error_handling** - Catch specific error types
4. **test_error_binding** - Access error variable
5. **test_finally_block** - Finally always executes
6. **test_finally_with_error** - Finally runs even on error
7. **test_custom_throw** - User-defined errors
8. **test_nested_try_catch** - Nested error handling
9. **test_multiple_catch_handlers** - Multiple error types
10. **test_stack_trace** - Stack trace generation
11. **test_error_propagation** - Uncaught errors bubble up
12. **test_error_recovery** - Retry patterns

### Example Programs
1. **error_basic.poh** - Basic try/catch demonstration
2. **error_types.poh** - Typed error handling
3. **error_custom.poh** - Custom error creation
4. **error_finally.poh** - Finally block usage
5. **error_recovery.poh** - Retry and fallback patterns
6. **error_file_handling.poh** - Real-world file error handling
7. **error_validation.poh** - Input validation with errors

## Success Criteria

✅ **Functionality**:
- Try/catch/finally blocks work correctly
- Error types can be distinguished
- Custom errors can be created and thrown
- Stack traces are generated accurately
- Finally blocks always execute

✅ **Usability**:
- Natural language syntax feels intuitive
- Error messages are clear and helpful
- Examples cover common patterns

✅ **Reliability**:
- All tests pass
- No regressions in existing functionality
- Edge cases handled properly

✅ **Performance**:
- Minimal overhead when no errors occur
- Stack trace building is efficient

## Open Questions

1. **Should we support error chaining?** (error caused by another error)
2. **Should we add error codes in addition to types?**
3. **How should we handle errors in finally blocks?**
4. **Should we add a global error handler?**
5. **Do we want error filtering by message pattern?**

## Timeline Estimate

- **Phase 5.1** (Infrastructure): 1 hour
- **Phase 5.2** (Try/Catch): 2 hours
- **Phase 5.3** (Error Types): 1 hour
- **Phase 5.4** (Error Ops): 1 hour
- **Phase 5.5** (Finally): 30 minutes
- **Phase 5.6** (Retrofit): 1 hour
- **Phase 5.7** (Testing): 2 hours

**Total Estimated Time**: ~8.5 hours

## Next Steps

1. Create `stdlib/errors.rs` with error infrastructure
2. Add Error variant to Value enum
3. Add call stack tracking to VM
4. Implement basic try/catch parsing and execution
5. Test with simple examples
6. Iterate and expand functionality

---

**Let's build robust error handling for PohLang! 🚀**
