# PohLang Roadmap: Becoming a Real Compiled Language

This roadmap outlines the path to make PohLang a fully independent, compiled language with its own toolchain—eliminating all Dart dependencies.

---

## Phase 1: Rust Runtime Feature Parity (Q4 2025)

**Goal**: The Rust runtime (`pohlangc`) can execute all core language features that the Python interpreter supports.

**Progress**: ~90% complete

**Tasks**:
- [x] Parser for all statement types (Write, Set, If, Repeat, While, Make, Use, Import)
- [x] VM execution for core statements and expressions
- [x] Inline and block functions with closures and default parameters
- [x] Local file imports
- [x] `Ask for <var>` input statement ✅ **COMPLETED** (2025-01-XX)
- [x] `Increase`/`Decrease` desugaring ✅ **COMPLETED** (2025-01-XX)
- [x] `minus` operator for subtraction ✅ **COMPLETED** (2025-01-XX)
- [x] `times` operator for multiplication ✅ **COMPLETED** (2025-01-XX)
- [x] `divided by` operator for division ✅ **COMPLETED** (2025-01-XX)
- [x] Modern collection literals: `[1, 2, 3]` lists, `{key: value}` dictionaries ✅ **COMPLETED** (2025-01-XX)
- [x] Collection indexing: `list[0]`, `dict["key"]`, negative indexing, nested indexing ✅ **COMPLETED** (2025-01-XX)
- [x] Enhanced error messages with helpful hints and suggestions ✅ **COMPLETED** (2025-01-XX)
- [ ] Run all Python test cases (`tests_python/`) via a test harness

**Deliverable**: `pohlangc --run` can execute any `.poh` program that the Python interpreter runs.

---

## Phase 2: Standard Library & System Modules (Q1 2026)

**Goal**: Implement standard library modules natively in Rust or as `.poh` libraries.

**Tasks**:
- [ ] `collections` module: list operations, dictionaries
- [ ] `random` module: random integers, floats, choices
- [ ] `datetime` module: current time, ISO dates, Hijri calendar helpers
- [ ] `math` module: sqrt, abs, floor, ceil, trigonometry
- [ ] `file` module: read/write text files
- [ ] `process` module: run shell commands (limited, safe API)
- [ ] Islamic utilities (optional): prayer times, Quran recitation metadata

**Deliverable**: Programs can `Import system "collections"` etc. and use rich built-in functions.

---

## Phase 3: Bytecode Compiler & VM (Q2 2026)

**Goal**: Compile `.poh` to a compact bytecode format for faster execution and portability.

**Tasks**:
- [ ] Define stack-based bytecode ISA (see DESIGN.md)
- [ ] Implement bytecode compiler in `src/compiler.rs`
- [ ] Implement bytecode VM in `src/bytecode_vm.rs`
- [ ] File format: magic header, const pool, code section
- [ ] CLI: `pohlangc --compile foo.poh -o foo.pbc`
- [ ] CLI: `pohlangc --run-bytecode foo.pbc`
- [ ] Benchmarks: compare AST-walking vs. bytecode VM performance

**Deliverable**: Users can compile and distribute `.pbc` files; faster execution than AST walking.

---

## Phase 4: AOT Native Compilation (Q3 2026)

**Goal**: Produce standalone native executables from `.poh` source.

**Tasks**:
- [ ] **Stage 1 (static bundle)**: Generate a Rust program embedding bytecode via `include_bytes!`, then compile with `rustc` or `cargo`.
  - `pohlangc --aot foo.poh -o foo.exe`
- [ ] **Stage 2 (Cranelift JIT/AOT)**: Lower bytecode to native code using Cranelift.
- [ ] Minimal runtime library linkable as a static archive
- [ ] Cross-compilation support (Linux, macOS, Windows, ARM)
- [ ] Strip debug info and optimize for size (release builds)

**Deliverable**: Single-file executables with no external dependencies; distributable binaries.

---

## Phase 5: Remove Dart & Publish Standalone Toolchain (Q4 2026)

**Goal**: Completely eliminate Dart dependencies; release PohLang as a standalone language.

**Tasks**:
- [ ] Audit codebase for any Dart references or dependencies
- [ ] Remove Dart code and documentation
- [ ] Update all examples to use `pohlangc`
- [ ] CI: build and test on all platforms; publish release artifacts (binaries for Windows, macOS, Linux)
- [ ] Package managers: `cargo install pohlangc`, `pip install pohlang` (Python wrapper deprecated or removed)
- [ ] Publish v1.0.0 release

**Deliverable**: PohLang is a real, independent compiled language with a native toolchain.

---

## Phase 6: Advanced Features & Ecosystem (2027+)

**Goal**: Grow the language with advanced features and community ecosystem.

**Possible features**:
- [ ] Package manager for PohLang libraries
- [ ] IDE support: LSP server, VS Code extension, syntax highlighting
- [ ] Debugger: breakpoints, step-through, variable inspection
- [ ] REPL: interactive shell
- [ ] WebAssembly target: compile to `.wasm` for browser/edge deployment
- [ ] Concurrency primitives: coroutines, async/await (if needed)
- [ ] Type hints or optional static typing (experiment)
- [ ] Community-contributed libraries and frameworks

---

## Success Criteria

1. **No Dart dependency**: Zero references to Dart in the codebase.
2. **Native executables**: Users can compile and distribute `.exe`, `.app`, or ELF binaries.
3. **Performance**: Bytecode VM is 10x+ faster than AST walking; AOT is comparable to Python or better.
4. **Usability**: Clear error messages, good docs, easy installation (one binary).
5. **Adoption**: Used in educational settings and by hobbyists; at least 10 community-contributed programs.

---

**Current Status** (October 2025):
- Phase 1: ~60% complete (core features work; need input, desugaring, collections, full test coverage)
- Phase 2: Not started
- Phase 3: Design done, implementation 0%
- Phase 4+: Planned

**Next Immediate Steps**:
1. Fix Windows MSVC toolchain so local builds work.
2. Run and pass all Rust smoke tests.
3. Implement `Ask for` and `Increase`/`Decrease`.
4. Add examples that work end-to-end with `pohlangc --run`.
