# PohLang Rust Runtime Design

This document captures the execution model, data model, and compilation plan for the Rust backend (`pohlangc`).

## Execution model

- **Frontend**: a hand-rolled parser builds a lightweight AST from phrasal syntax.
- **Two backends**:
  - **Direct interpreter** (current): the VM walks the AST and executes statements and expressions.
  - **Bytecode VM** (planned): the compiler lowers AST to a compact bytecode; the VM executes opcodes.
- **AOT compilation** (planned): compile bytecode to a native executable via code generation or a JIT backend (e.g., Cranelift), then link a minimal runtime.

## Data model

- **Scalars**: number (f64), string (UTF-8), boolean (as numeric truthiness: 0/1), nil (empty string currently). 
- **Collections**: list of values (vector). Dictionaries/maps: not yet.
- **Functions**: closures with default parameters; captured environments are snapshotted maps.

## Language surface (MVP)

**Statements:**
- `Write <expr>`
- `Set <name> to <expr>`
- `If <cond> Write <expr> [Otherwise Write <expr>]`
- `If <cond>` ... `Otherwise` ... `End`
- `Repeat <count> [times]` ... `End`
- `While <cond>` ... `End`
- `Make <name> with <params> Write <expr>`
- `Make <name> with <params>` ... `Return <expr>` ... `End`
- `Use <name> [with <args>]`
- `Import "path"` (relative file)
- `Import system "name"` (stubbed)

**Expressions:**
- string, number, identifier, call `name(arg, ...)`, logical `And/Or/Not`, comparisons (`Equals`, `Not Equals`, `Greater Than`, `Less Than`, `Greater Or Equal`, `Less Or Equal`), concatenation/addition with `plus`.

## Bytecode plan

Define a simple stack-based ISA:
- **Constants**: `PUSH_STR s`, `PUSH_NUM n`
- **Vars**: `LOAD name`, `STORE name`
- **Ops**: `ADD`, `CMP_EQ`, `CMP_NE`, `CMP_LT`, `CMP_LE`, `CMP_GT`, `CMP_GE`, `NOT`, `AND`, `OR`
- **Control**: `JUMP addr`, `JUMP_IF_FALSE addr`, `RET`, `POP`
- **Calls**: `CALL name argc`
- **Print**: `WRITE`

**File format (v0):**
- Header: magic `POHB` + version `0x0001`
- Const pool: strings array, names array
- Code section: sequence of opcodes with u32 operands

## AOT plan

**Stage 1 (static runner):**
- `pohlangc --aot foo.poh -o foo.exe` emits a small Rust program embedding compiled bytecode via `include_bytes!` and a `main()` that runs the VM; then spawns `cargo`/`rustc` to build it.

**Stage 2 (JIT/AOT):**
- Use Cranelift to lower the bytecode to native code and emit an exe.

## Error handling

- Keep phrasal diagnostics `[file: Line X: Col 1] message` (already used in parser).
- For runtime errors, print function name and best-effort source context.

## Compatibility

- Aim to run a growing subset of the Python interpreter tests (`tests_python/`).
- Add golden-output tests in `runtime-rs/tests` for examples in `examples/poh`.

## Removing Dart dependency

Currently, PohLang has legacy Dart dependencies. The Rust runtime is the replacement path:

1. **Phase 1**: Rust runtime reaches feature parity with Python interpreter for core language features.
2. **Phase 2**: Port or reimplement standard library modules (collections, random, date/time utilities).
3. **Phase 3**: Remove all Dart code and references; publish standalone binaries.
4. **Phase 4**: Add native compilation via bytecode or AOT for deployment as real executables.

## Standard library architecture

- **Built-in functions** (in `vm.rs`): `now`, `range`, `join`, `split`, `length`
- **System modules** (future): `collections.poh`, `random.poh`, `datetime.poh`, etc. loaded via `Import system "name"`
- **FFI bridge** (optional): allow calling Rust functions from PohLang for performance-critical operations.
