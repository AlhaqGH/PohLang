# Phase 6 Quick Start Guide

**Date**: October 10, 2025  
**Goal**: Implement Standard Library modules starting with File I/O

---

## 🚀 Starting with Module 1: File I/O

### Why File I/O First?
- Most practical and immediately useful
- Users can build real applications
- Foundation for other modules (JSON reads files, etc.)
- Demonstrates module system architecture

---

## Step 1: Create File Structure

```bash
cd c:\Users\habib\POHLANG\PohLang\runtime

# Create stdlib directory if it doesn't exist
mkdir src\stdlib -Force

# Create file module
New-Item src\stdlib\file.rs -Force
```

---

## Step 2: Module Registration

Update `src/stdlib/mod.rs`:

```rust
// Standard library modules
pub mod errors;  // Already exists
pub mod file;    // NEW

use crate::vm::value::Value;
use std::collections::HashMap;

pub fn register_stdlib_modules() -> HashMap<String, HashMap<String, Value>> {
    let mut modules = HashMap::new();
    
    // Register file module
    modules.insert("file".to_string(), file::create_file_module());
    
    modules
}
```

---

## Step 3: Implement File Module

File: `src/stdlib/file.rs`

```rust
use crate::vm::value::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn create_file_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    
    // file read "path"
    module.insert("read".to_string(), Value::NativeFunction("file_read".to_string()));
    
    // file write "path" with "content"
    module.insert("write".to_string(), Value::NativeFunction("file_write".to_string()));
    
    // file append "path" with "content"
    module.insert("append".to_string(), Value::NativeFunction("file_append".to_string()));
    
    // file exists "path"
    module.insert("exists".to_string(), Value::NativeFunction("file_exists".to_string()));
    
    // file delete "path"
    module.insert("delete".to_string(), Value::NativeFunction("file_delete".to_string()));
    
    // file list "directory"
    module.insert("list".to_string(), Value::NativeFunction("file_list".to_string()));
    
    module
}

// Native function implementations
pub fn file_read(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error occurred: a file error - Failed to read '{}': {}", path, e))
}

pub fn file_write(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content)
        .map_err(|e| format!("Error occurred: a file error - Failed to write to '{}': {}", path, e))
}

pub fn file_append(path: &str, content: &str) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| format!("Error occurred: a file error - Failed to append to '{}': {}", path, e))
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn file_delete(path: &str) -> Result<(), String> {
    fs::remove_file(path)
        .map_err(|e| format!("Error occurred: a file error - Failed to delete '{}': {}", path, e))
}

pub fn file_list(dir: &str) -> Result<Vec<String>, String> {
    fs::read_dir(dir)
        .map_err(|e| format!("Error occurred: a file error - Failed to list '{}': {}", dir, e))?
        .map(|entry| {
            entry
                .map(|e| e.file_name().to_string_lossy().to_string())
                .map_err(|e| format!("Error reading directory entry: {}", e))
        })
        .collect()
}
```

---

## Step 4: Integrate with VM

Update `src/vm/vm.rs` to call native functions:

```rust
// In execute_phrasal_call or similar
match function_name.as_str() {
    "file_read" => {
        let path = self.evaluate_expression(&args[0])?;
        let path_str = path.as_string()?;
        let content = crate::stdlib::file::file_read(&path_str)?;
        Ok(Value::String(content))
    }
    "file_write" => {
        let path = self.evaluate_expression(&args[0])?;
        let content = self.evaluate_expression(&args[1])?;
        crate::stdlib::file::file_write(&path.as_string()?, &content.as_string()?)?;
        Ok(Value::Null)
    }
    // ... more functions
    _ => Err(format!("Unknown native function: {}", function_name))
}
```

---

## Step 5: Write Tests

Create `tests/stdlib/file_test.poh`:

```pohlang
# Test file operations
Import system "file"

# Test write and read
file write "test.txt" with "Hello, World!"
Set content to file read "test.txt"
Write "Read: " + content

# Test append
file append "test.txt" with "\nSecond line"
Set content2 to file read "test.txt"
Write "After append: " + content2

# Test exists
If file exists "test.txt":
    Write "File exists!"
end if

If not file exists "missing.txt":
    Write "Missing file doesn't exist (correct)"
end if

# Test delete
file delete "test.txt"
If not file exists "test.txt":
    Write "File deleted successfully"
end if

Write "All file tests passed!"
```

---

## Step 6: Test It!

```bash
cd c:\Users\habib\POHLANG\PohLang\runtime

# Build
cargo build --release

# Run test
..\target\release\pohlang.exe tests\stdlib\file_test.poh
```

---

## Step 7: Add Error Handling Tests

Create `tests/stdlib/file_error_test.poh`:

```pohlang
Import system "file"

# Test error handling
try this:
    Set content to file read "nonexistent.txt"
if error of type "FileError" as e:
    Write "Caught file error: " + error message of e
finally:
    Write "Test complete"
end try
```

---

## Expected Output

```
Read: Hello, World!
After append: Hello, World!
Second line
File exists!
Missing file doesn't exist (correct)
File deleted successfully
All file tests passed!
```

---

## Next Steps After File Module

1. ✅ File I/O complete
2. 🔥 Math module (2-3 days)
3. 🔥 Collections module (4-5 days)
4. Continue with other modules...

---

## Troubleshooting

### "Module not found"
- Check `stdlib/mod.rs` has `pub mod file;`
- Check module is registered in `register_stdlib_modules()`

### "Native function not found"
- Check VM has handler for native function calls
- Check function name matches exactly

### "File operation failed"
- Check file paths are correct
- Check permissions
- Use absolute paths if relative paths don't work

---

## Tips

1. **Start small**: Implement one function at a time
2. **Test as you go**: Write test for each function
3. **Natural errors**: Use "a file error" format for consistency
4. **Document**: Add comments explaining phrasal syntax

---

**Ready to implement File I/O module?** 

Run these commands to get started:

```powershell
cd c:\Users\habib\POHLANG\PohLang\runtime
mkdir src\stdlib -Force
code src\stdlib\file.rs
```

Then copy the implementation above! 🚀
