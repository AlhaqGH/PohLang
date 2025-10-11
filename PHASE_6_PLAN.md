# Phase 6 Implementation Plan - Standard Library & Bytecode Compiler

**Start Date**: October 10, 2025  
**Target Completion**: Q1 2025  
**Status**: Planning Phase

---

## Overview

Phase 6 combines two critical improvements:
1. **Standard Library** - Make PohLang practical for real-world use
2. **Bytecode Compiler** - Make PohLang performant and production-ready

We'll implement these **in parallel** with two tracks:

---

## Track 1: Standard Library (Priority 1)

### Goal
Provide essential built-in modules for file operations, mathematics, collections, and more.

### Implementation Approach
All modules will be **native Rust implementations** in `runtime/src/stdlib/`.

---

### Module 1: File I/O (`file` module) 🔥 **HIGH PRIORITY**

**Syntax:**
```pohlang
Import system "file"

# Read entire file
Set content to file read "data.txt"

# Write to file
file write "output.txt" with "Hello, World!"

# Append to file
file append "log.txt" with "New entry\n"

# Check if file exists
If file exists "config.txt":
    Write "Config found"
end if

# Delete file
file delete "temp.txt"

# List files in directory
Set files to file list "."
```

**Implementation:**
- `runtime/src/stdlib/file.rs`
- Error handling with `FileError` type (already exists!)
- Natural language error messages
- Path validation and safety checks

**Estimated Time**: 3-4 days

---

### Module 2: Math (`math` module) 🔥 **HIGH PRIORITY**

**Syntax:**
```pohlang
Import system "math"

# Basic functions
Set root to math sqrt of 16          # 4.0
Set rounded to math round 3.7        # 4
Set absolute to math abs of -5       # 5
Set power to math pow 2 to 8         # 256

# Trigonometry
Set sine to math sin of 1.5708       # ~1.0
Set cosine to math cos of 0          # 1.0
Set tangent to math tan of 0.785     # ~1.0

# Constants
Set pi to math pi                     # 3.14159...
Set e to math e                       # 2.71828...

# Advanced
Set minimum to math min of [5, 2, 9, 1]    # 1
Set maximum to math max of [5, 2, 9, 1]    # 9
Set sum to math sum of [1, 2, 3, 4]        # 10
Set average to math average of [1, 2, 3, 4] # 2.5
```

**Implementation:**
- `runtime/src/stdlib/math.rs`
- Use Rust's `f64` math functions
- Handle domain errors (sqrt of negative, etc.)
- Return `MathError` for invalid operations

**Estimated Time**: 2-3 days

---

### Module 3: Collections (`collections` module) 🔥 **HIGH PRIORITY**

**Syntax:**
```pohlang
Import system "collections"

# List operations
Set numbers to [5, 2, 9, 1, 7]
Set sorted to collections sort numbers        # [1, 2, 5, 7, 9]
Set reversed to collections reverse numbers   # [7, 1, 9, 2, 5]

# Filtering
Set evens to collections filter numbers with is_even
Set odds to collections filter numbers where value > 3

# Mapping
Set doubled to collections map numbers with double_fn

# Reducing
Set sum to collections reduce numbers with add_fn from 0

# Searching
Set index to collections find numbers with value equals 9  # 2
If collections contains numbers value 7:
    Write "Found 7"
end if

# Dictionary operations
Set data to {"name": "Ali", "age": 25}
Set keys to collections keys of data         # ["name", "age"]
Set values to collections values of data     # ["Ali", 25]
```

**Implementation:**
- `runtime/src/stdlib/collections.rs`
- Sort, reverse, filter, map, reduce
- Search operations
- Dictionary utilities
- Natural iteration support

**Estimated Time**: 4-5 days

---

### Module 4: String Utilities (`string` module)

**Syntax:**
```pohlang
Import system "string"

# Manipulation
Set upper to string uppercase "hello"        # "HELLO"
Set lower to string lowercase "WORLD"        # "world"
Set trimmed to string trim "  text  "        # "text"

# Splitting and joining
Set words to string split "a,b,c" by ","     # ["a", "b", "c"]
Set joined to string join ["a", "b"] with "," # "a,b,c"

# Searching
If string contains "hello world" substring "world":
    Write "Found it"
end if

Set index to string find "hello" in "world"  # -1 (not found)

# Replacing
Set replaced to string replace "hello" with "hi" in "hello world"

# Formatting
Set padded to string pad "5" to 3 with "0"   # "005"
```

**Implementation:**
- `runtime/src/stdlib/string.rs`
- Common string operations
- Unicode-aware where needed
- Integration with existing string type

**Estimated Time**: 2-3 days

---

### Module 5: DateTime (`datetime` module)

**Syntax:**
```pohlang
Import system "datetime"

# Current time
Set now to datetime now
Write now

# Formatting
Set formatted to datetime format now as "YYYY-MM-DD HH:mm:ss"
Write formatted  # "2025-10-10 14:30:00"

# Parsing
Set date to datetime parse "2025-10-10" with format "YYYY-MM-DD"

# Components
Set year to datetime year of now
Set month to datetime month of now
Set day to datetime day of now
Set hour to datetime hour of now
Set minute to datetime minute of now
Set second to datetime second of now

# Comparisons
If datetime before date1 than date2:
    Write "date1 is earlier"
end if

# Arithmetic (optional)
Set tomorrow to datetime add now days 1
Set yesterday to datetime subtract now days 1
```

**Implementation:**
- `runtime/src/stdlib/datetime.rs`
- Use `chrono` crate for date/time handling
- Support common date formats
- Timezone handling (UTC by default)

**Estimated Time**: 3-4 days

---

### Module 6: JSON (`json` module)

**Syntax:**
```pohlang
Import system "json"

# Parse JSON string to PohLang value
Set data to json parse "{\"name\": \"Ali\", \"age\": 25}"
Write data["name"]  # "Ali"

# Convert PohLang value to JSON string
Set person to {"name": "Sara", "age": 30}
Set json_string to json stringify person
Write json_string  # {"name":"Sara","age":30}

# Pretty print
Set pretty to json stringify person with indent 2

# Read from file
Set config to json read "config.json"

# Write to file
json write "output.json" with data
```

**Implementation:**
- `runtime/src/stdlib/json.rs`
- Use `serde_json` for parsing/serialization
- Convert between JSON and PohLang values
- Handle `JsonError` type (already exists!)

**Estimated Time**: 2-3 days

---

### Module 7: Random (`random` module)

**Syntax:**
```pohlang
Import system "random"

# Random integer in range
Set dice to random int from 1 to 6

# Random float between 0 and 1
Set chance to random float

# Random float in range
Set temperature to random float from 20.0 to 30.0

# Random choice from list
Set colors to ["red", "green", "blue"]
Set color to random choice from colors

# Shuffle list
Set shuffled to random shuffle colors

# Random boolean
Set coin to random boolean
```

**Implementation:**
- `runtime/src/stdlib/random.rs`
- Use `rand` crate
- Seed management (optional)
- Cryptographically secure option (optional)

**Estimated Time**: 2 days

---

### Module 8: Process (`process` module) ⚠️ **OPTIONAL**

**Syntax:**
```pohlang
Import system "process"

# Run command and get output
Set result to process run "ls -la"
Write result

# Run with error handling
try this:
    Set output to process run "invalid_command"
if error of type "ProcessError" as e:
    Write "Command failed: " + error message of e
end try

# Environment variables
Set path to process env "PATH"
process set env "MY_VAR" to "value"
```

**Implementation:**
- `runtime/src/stdlib/process.rs`
- Use `std::process::Command`
- Security considerations (restrict dangerous commands?)
- Cross-platform compatibility

**Estimated Time**: 3-4 days

---

## Track 2: Bytecode Compiler (Priority 2)

### Goal
Compile PohLang to bytecode for 10x+ faster execution and prepare for AOT compilation.

---

### Stage 1: Design Bytecode ISA

**Define Stack-Based Instructions:**
```rust
pub enum Instruction {
    // Literals
    LoadConst(u32),      // Push constant from pool
    LoadTrue,
    LoadFalse,
    LoadNull,
    
    // Variables
    LoadLocal(u32),      // Load local variable
    StoreLocal(u32),     // Store to local variable
    LoadGlobal(String),  // Load global variable
    StoreGlobal(String), // Store to global variable
    
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    
    // Comparison
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // Logical
    And,
    Or,
    Not,
    
    // Control flow
    Jump(u32),           // Unconditional jump
    JumpIfFalse(u32),    // Jump if top of stack is false
    JumpIfTrue(u32),     // Jump if top of stack is true
    
    // Functions
    Call(u8),            // Call function with N args
    Return,
    
    // Collections
    BuildList(u32),      // Build list from N stack items
    BuildDict(u32),      // Build dict from N*2 stack items
    Index,               // Index into collection
    
    // Error handling
    PushTryHandler(u32), // Register error handler
    PopTryHandler,
    Throw,
    
    // I/O
    Print,
    Input,
    
    // Other
    Pop,                 // Discard top of stack
    Duplicate,           // Duplicate top of stack
    Halt,                // Stop execution
}
```

**Estimated Time**: 2-3 days (design + documentation)

---

### Stage 2: Implement Bytecode Compiler

**File:** `runtime/src/compiler/bytecode.rs`

**Responsibilities:**
- Walk AST and emit bytecode instructions
- Manage constant pool (strings, numbers)
- Track variable scopes and indices
- Generate jump labels and resolve addresses
- Optimize common patterns

**Example Compilation:**
```pohlang
# Source
Set x to 5
Set y to x plus 10
Write y
```

```
# Bytecode
LoadConst 0      # Push 5
StoreLocal 0     # Store to x
LoadLocal 0      # Load x
LoadConst 1      # Push 10
Add              # x + 10
StoreLocal 1     # Store to y
LoadLocal 1      # Load y
Print            # Write y
Halt
```

**Estimated Time**: 1-2 weeks

---

### Stage 3: Implement Bytecode VM

**File:** `runtime/src/vm/bytecode_vm.rs`

**Responsibilities:**
- Execute bytecode instructions
- Manage value stack
- Handle function calls and returns
- Implement error handling (try/catch)
- Track call stack for errors

**Components:**
- `BytecodeVM` struct
- Value stack (`Vec<Value>`)
- Call frames stack
- Instruction pointer
- Constant pool
- Global variables map

**Estimated Time**: 1-2 weeks

---

### Stage 4: File Format & CLI

**`.pbc` File Format:**
```
Magic: "POHC" (4 bytes)
Version: 1 (u32)
Const Pool Size: N (u32)
Const Pool: [const1, const2, ...]
Code Size: M (u32)
Code: [instruction bytes...]
```

**CLI Commands:**
```bash
# Compile to bytecode
pohlang --compile program.poh -o program.pbc

# Run bytecode
pohlang --run-bytecode program.pbc

# Run with bytecode (automatic compilation)
pohlang --bytecode program.poh
```

**Estimated Time**: 3-4 days

---

### Stage 5: Testing & Benchmarking

**Test Suite:**
- Compile and run all existing examples
- Verify output matches AST interpreter
- Test error handling in bytecode
- Stress test with large programs

**Benchmarks:**
- Fibonacci (recursive)
- List operations
- String manipulation
- Error handling overhead
- Target: 10x+ faster than AST walking

**Estimated Time**: 1 week

---

## Timeline

### Month 1 (October-November 2025)
- Week 1: File I/O module
- Week 2: Math + Collections modules
- Week 3: String + DateTime modules
- Week 4: JSON + Random modules

### Month 2 (November-December 2025)
- Week 1: Bytecode ISA design
- Week 2-3: Bytecode compiler implementation
- Week 4: Start bytecode VM

### Month 3 (December 2025-January 2026)
- Week 1-2: Complete bytecode VM
- Week 3: File format + CLI
- Week 4: Testing + benchmarking

**Target Completion**: End of January 2026

---

## Success Criteria

### Standard Library:
- ✅ 7-8 modules implemented
- ✅ All modules have comprehensive tests
- ✅ Documentation with examples
- ✅ Natural language error messages
- ✅ Cross-platform compatibility

### Bytecode Compiler:
- ✅ Compile all PohLang features to bytecode
- ✅ 10x+ performance improvement
- ✅ `.pbc` file format working
- ✅ 100% test coverage (all examples work)
- ✅ Error handling preserved

---

## File Structure

```
runtime/
├── src/
│   ├── stdlib/
│   │   ├── mod.rs          # Module registration
│   │   ├── file.rs         # File I/O
│   │   ├── math.rs         # Mathematics
│   │   ├── collections.rs  # Collections
│   │   ├── string.rs       # String utilities
│   │   ├── datetime.rs     # Date/Time
│   │   ├── json.rs         # JSON
│   │   ├── random.rs       # Random
│   │   └── process.rs      # Process (optional)
│   ├── compiler/
│   │   ├── mod.rs          # Compiler entry point
│   │   ├── bytecode.rs     # Bytecode compiler
│   │   └── optimizer.rs    # Optimization passes
│   ├── vm/
│   │   ├── mod.rs          # VM module
│   │   ├── vm.rs           # Current AST VM
│   │   └── bytecode_vm.rs  # New bytecode VM
│   └── ...
├── tests/
│   ├── stdlib/             # Standard library tests
│   │   ├── file_test.poh
│   │   ├── math_test.poh
│   │   └── ...
│   └── bytecode/           # Bytecode tests
│       └── ...
└── ...
```

---

## Dependencies to Add

```toml
[dependencies]
# Existing dependencies...

# For datetime module
chrono = "0.4"

# For random module
rand = "0.8"

# For JSON module (already have serde_json)
# serde_json = "1.0"

# For bytecode serialization
bincode = "1.3"  # Optional: for .pbc file format
```

---

## Next Immediate Steps

1. **Update ROADMAP.md** with correct phase information
2. **Create Phase 6 directory structure**
3. **Start with File I/O module** (highest priority)
4. **Write comprehensive tests for each module**
5. **Document each module with examples**

---

**Ready to start Phase 6?** 🚀

Let me know which module you want to implement first!
