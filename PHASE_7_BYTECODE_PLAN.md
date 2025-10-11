# Phase 7: Bytecode Compilation & VM

**Start Date:** October 11, 2025  
**Target Completion:** Q4 2025 (6-8 weeks)  
**Status:** Planning Phase 🚀

---

## Overview

Phase 7 implements a bytecode compiler and virtual machine to achieve **10x+ performance improvement** over AST walking and prepare the foundation for AOT (Ahead-Of-Time) compilation.

### Why Bytecode Now?

After completing the web framework in Phase 6, bytecode compilation is the logical next step:

1. **Performance Critical**: Web apps need fast execution
2. **Clean Architecture**: Better to optimize before adding more features (stdlib)
3. **Production Ready**: `.pbc` files are more professional than interpreted scripts
4. **Foundation for AOT**: Phase 9 (native compilation) depends on this
5. **Distribution**: Compiled bytecode protects source code

---

## Goals

### Primary Goals
- [ ] 10x+ faster execution than AST walking
- [ ] `.pbc` bytecode file format
- [ ] CLI commands: `--compile`, `--run-bytecode`
- [ ] 100% feature parity with interpreter
- [ ] Cross-platform bytecode (write once, run anywhere)

### Secondary Goals
- [ ] Bytecode optimization passes
- [ ] Debug symbols in `.pbc` files
- [ ] Bytecode disassembler tool
- [ ] Benchmark suite comparing AST vs bytecode

---

## Architecture

### Stack-Based Bytecode ISA

```rust
pub enum Instruction {
    // === Literals ===
    LoadConst(u32),      // Push constant from pool by index
    LoadTrue,            // Push true
    LoadFalse,           // Push false
    LoadNull,            // Push null
    
    // === Variables ===
    LoadLocal(u32),      // Load local variable by index
    StoreLocal(u32),     // Store to local variable
    LoadGlobal(String),  // Load global variable by name
    StoreGlobal(String), // Store to global variable
    
    // === Arithmetic ===
    Add,                 // Pop two values, push sum
    Subtract,            // Pop two values, push difference
    Multiply,            // Pop two values, push product
    Divide,              // Pop two values, push quotient
    Negate,              // Pop value, push negation
    
    // === Comparisons ===
    Equal,               // Pop two, push true if equal
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    // === Logical ===
    And,                 // Pop two, push logical AND
    Or,                  // Pop two, push logical OR
    Not,                 // Pop one, push logical NOT
    
    // === Control Flow ===
    Jump(u32),           // Unconditional jump to offset
    JumpIfFalse(u32),    // Jump if top of stack is false
    JumpIfTrue(u32),     // Jump if top of stack is true
    Loop(u32),           // Jump backward (for loops)
    
    // === Functions ===
    Call(u8),            // Call function with N arguments
    Return,              // Return from function
    
    // === Collections ===
    BuildList(u32),      // Build list from N stack items
    BuildDict(u32),      // Build dict from N*2 stack items
    Index,               // Pop index and collection, push element
    
    // === I/O ===
    Print,               // Pop and print value
    Input,               // Read input, push as string
    
    // === Other ===
    Pop,                 // Discard top of stack
    Duplicate,           // Duplicate top of stack
    Halt,                // Stop execution
}
```

### Bytecode File Format (`.pbc`)

```
┌─────────────────────────────────────┐
│ Magic Number: "POHC" (4 bytes)      │
├─────────────────────────────────────┤
│ Version: u32 (currently 1)          │
├─────────────────────────────────────┤
│ Constant Pool Size: u32             │
├─────────────────────────────────────┤
│ Constant Pool:                      │
│   - Type tag (1 byte)               │
│   - Data (variable length)          │
│   [Repeated N times]                │
├─────────────────────────────────────┤
│ Code Size: u32 (number of bytes)    │
├─────────────────────────────────────┤
│ Code:                               │
│   - Instruction bytecode            │
│   [Code section]                    │
├─────────────────────────────────────┤
│ Debug Info (optional):              │
│   - Source file name                │
│   - Line number mappings            │
│   - Variable names                  │
└─────────────────────────────────────┘
```

### Constant Pool Types

```rust
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
```

---

## Implementation Plan

### Stage 1: Bytecode ISA Design (Week 1)
**Goal:** Define the instruction set and bytecode format

**Tasks:**
- [x] Document instruction set (see above)
- [ ] Design file format specification
- [ ] Create `src/bytecode/mod.rs` with instruction enum
- [ ] Implement instruction encoding/decoding
- [ ] Write unit tests for instructions

**Deliverable:** `bytecode::Instruction` enum with full documentation

---

### Stage 2: Bytecode Compiler (Weeks 2-3)
**Goal:** Convert AST to bytecode instructions

**File:** `runtime/src/bytecode/compiler.rs`

**Architecture:**
```rust
pub struct Compiler {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    locals: HashMap<String, u32>,
    scope_depth: usize,
}

impl Compiler {
    pub fn compile(program: &Program) -> Result<BytecodeChunk>;
    fn compile_statement(&mut self, stmt: &Statement);
    fn compile_expression(&mut self, expr: &Expression);
    fn emit(&mut self, instruction: Instruction);
    fn add_constant(&mut self, constant: Constant) -> u32;
}
```

**Example Compilation:**

Source:
```poh
Set x to 5
Set y to x plus 10
Write y
```

Bytecode:
```
LoadConst 0      # Push 5
StoreLocal 0     # Store to x (local #0)
LoadLocal 0      # Load x
LoadConst 1      # Push 10
Add              # x + 10
StoreLocal 1     # Store to y (local #1)
LoadLocal 1      # Load y
Print            # Write y
Halt
```

**Tasks:**
- [ ] Implement `Compiler` struct
- [ ] Compile all statement types
- [ ] Compile all expression types
- [ ] Handle control flow (If, While, Repeat)
- [ ] Handle function definitions and calls
- [ ] Manage constant pool
- [ ] Track variable scopes and indices
- [ ] Generate jump labels and resolve addresses

**Deliverable:** Working compiler that converts AST to bytecode

---

### Stage 3: Bytecode VM (Weeks 4-5)
**Goal:** Execute bytecode instructions

**File:** `runtime/src/bytecode/vm.rs`

**Architecture:**
```rust
pub struct BytecodeVM {
    stack: Vec<Value>,
    ip: usize,                    // Instruction pointer
    code: Vec<Instruction>,
    constants: Vec<Constant>,
    globals: HashMap<String, Value>,
    call_stack: Vec<CallFrame>,
}

struct CallFrame {
    ip: usize,
    locals: Vec<Value>,
}

impl BytecodeVM {
    pub fn new(chunk: BytecodeChunk) -> Self;
    pub fn run(&mut self) -> Result<()>;
    fn execute_instruction(&mut self, instr: &Instruction) -> Result<()>;
    fn push(&mut self, value: Value);
    fn pop(&mut self) -> Result<Value>;
}
```

**Tasks:**
- [ ] Implement `BytecodeVM` struct
- [ ] Execute all instruction types
- [ ] Manage value stack
- [ ] Handle function calls and returns
- [ ] Implement error handling (try/catch)
- [ ] Track call stack for error messages
- [ ] Support hot reload for web apps

**Deliverable:** Fully functional bytecode VM

---

### Stage 4: File Format & Serialization (Week 6)
**Goal:** Read/write `.pbc` files

**File:** `runtime/src/bytecode/format.rs`

**Architecture:**
```rust
pub struct BytecodeChunk {
    pub version: u32,
    pub constants: Vec<Constant>,
    pub code: Vec<Instruction>,
    pub debug_info: Option<DebugInfo>,
}

impl BytecodeChunk {
    pub fn save_to_file(&self, path: &Path) -> Result<()>;
    pub fn load_from_file(path: &Path) -> Result<Self>;
    pub fn to_bytes(&self) -> Vec<u8>;
    pub fn from_bytes(bytes: &[u8]) -> Result<Self>;
}
```

**Tasks:**
- [ ] Implement bytecode serialization
- [ ] Implement bytecode deserialization
- [ ] Add magic number validation
- [ ] Add version compatibility checks
- [ ] Implement debug info sections
- [ ] Add file compression (optional)

**Deliverable:** Working `.pbc` file format

---

### Stage 5: CLI Integration (Week 7)
**Goal:** Add bytecode commands to CLI

**File:** `runtime/src/main.rs`

**New CLI Arguments:**
```bash
# Compile to bytecode
pohlang --compile program.poh -o program.pbc

# Run bytecode
pohlang --run-bytecode program.pbc

# Compile and run (auto)
pohlang --bytecode program.poh

# Show bytecode (disassembler)
pohlang --disassemble program.pbc
```

**Tasks:**
- [ ] Add `--compile` flag
- [ ] Add `--run-bytecode` flag  
- [ ] Add `--bytecode` flag (compile + run)
- [ ] Add `--disassemble` flag
- [ ] Update help text
- [ ] Add bytecode caching

**Deliverable:** Complete CLI interface for bytecode

---

### Stage 6: Testing & Benchmarks (Week 8)
**Goal:** Ensure correctness and measure performance

**Tasks:**
- [ ] Run all existing tests with bytecode VM
- [ ] Verify output matches AST interpreter
- [ ] Test error handling in bytecode
- [ ] Benchmark: Fibonacci (recursive)
- [ ] Benchmark: List operations
- [ ] Benchmark: String manipulation
- [ ] Benchmark: Web server performance
- [ ] Compare AST vs Bytecode performance
- [ ] Optimize hot paths

**Success Criteria:**
- All tests pass ✅
- 10x+ faster than AST walking ✅
- Error messages preserved ✅
- Web framework works with bytecode ✅

**Deliverable:** Performance report showing 10x+ improvement

---

## Performance Targets

| Benchmark | AST Walking | Bytecode | Target Speedup |
|-----------|-------------|----------|----------------|
| Fibonacci(30) | ~2.5s | <250ms | 10x+ |
| List sum (100k) | ~500ms | <50ms | 10x+ |
| String concat (10k) | ~300ms | <30ms | 10x+ |
| Web request handling | ~50ms | <5ms | 10x+ |

---

## File Structure

```
runtime/
├── src/
│   ├── bytecode/
│   │   ├── mod.rs          # Module exports
│   │   ├── instruction.rs  # Instruction enum
│   │   ├── compiler.rs     # AST → Bytecode
│   │   ├── vm.rs          # Bytecode execution
│   │   ├── format.rs      # .pbc file I/O
│   │   └── optimizer.rs   # Optimization passes
│   ├── main.rs            # CLI with bytecode flags
│   └── ...
├── tests/
│   ├── bytecode/          # Bytecode-specific tests
│   │   ├── compiler_test.rs
│   │   ├── vm_test.rs
│   │   └── format_test.rs
│   └── ...
└── benches/               # Performance benchmarks
    ├── ast_vs_bytecode.rs
    └── ...
```

---

## Example Usage

### Compile a Program
```bash
$ pohlang --compile examples/poh/fact.poh -o fact.pbc
✅ Compiled to bytecode: fact.pbc (2.1 KB)
```

### Run Bytecode
```bash
$ pohlang --run-bytecode fact.pbc
Enter a number: 10
Factorial of 10 is 3628800
```

### Compile and Run
```bash
$ pohlang --bytecode examples/poh/web_hello.poh
✅ Compiled to bytecode cache
🚀 Server listening on http://localhost:3000
```

### Disassemble Bytecode
```bash
$ pohlang --disassemble fact.pbc
=== fact.pbc ===
Version: 1
Constants: 3
  [0] Number(1)
  [1] Number(1)
  [2] String("Factorial of ")

Code (25 instructions):
  0000  LoadLocal 0         # n
  0001  LoadConst 0         # 1
  0002  LessEqual
  0003  JumpIfFalse 8
  0004  LoadConst 0         # 1
  0005  Return
  ...
```

---

## Dependencies

**New Crates Needed:**
```toml
[dependencies]
# Existing dependencies...

# For bytecode serialization
bincode = "1.3"  # Binary encoding
```

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Bytecode bugs hard to debug | High | Build comprehensive disassembler, add debug symbols |
| Performance not 10x | Medium | Profile VM, optimize hot paths, use benchmarks early |
| Breaking changes to AST | Low | Maintain both AST and bytecode paths in parallel |
| File format changes | Medium | Add version checking, support migration |

---

## Success Criteria

### Must Have ✅
- [ ] 10x+ faster than AST interpreter
- [ ] All existing tests pass
- [ ] `.pbc` file format working
- [ ] CLI commands implemented
- [ ] Error messages preserved

### Should Have 🎯
- [ ] Bytecode optimizer
- [ ] Debug symbols
- [ ] Disassembler tool
- [ ] Benchmark suite

### Nice to Have 🌟
- [ ] JIT compilation hints
- [ ] Bytecode caching
- [ ] Cross-compilation support
- [ ] Bytecode signing

---

## Timeline

**Week 1:** ISA design & documentation  
**Week 2-3:** Compiler implementation  
**Week 4-5:** VM implementation  
**Week 6:** File format & serialization  
**Week 7:** CLI integration  
**Week 8:** Testing & benchmarks  

**Total:** 8 weeks (6-8 weeks with buffer)

---

## Next Steps

1. **Review this plan** - Feedback and approval
2. **Create `src/bytecode/` directory**
3. **Implement `Instruction` enum**
4. **Write first compiler test**
5. **Start Stage 1: ISA Design**

---

## Questions to Resolve

1. **Optimization level**: Should we have `-O0`, `-O1`, `-O2` flags?
2. **Debug symbols**: Include source line numbers by default?
3. **Caching**: Auto-compile `.poh` to `.pbc` on first run?
4. **Versioning**: How to handle bytecode version mismatches?
5. **Distribution**: Should we bundle bytecode VM in releases?

---

**Ready to start Phase 7?** 🚀

This will make PohLang **production-ready** with enterprise-level performance!
