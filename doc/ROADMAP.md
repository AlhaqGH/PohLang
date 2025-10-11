# PohLang Roadmap: Becoming a Real Compiled Language

This roadmap outlines the path to make PohLang a fully independent, compiled language with its own Rust-based toolchain.

---

## Phase 1: Rust Runtime Feature Parity (Q4 2025)

**Goal**: The Rust runtime can execute all core language features.

**Progress**: ~90% complete ✅

**Completed Tasks**:
- [x] Parser for all statement types (Write, Set, If, Repeat, While, Make, Use, Import)
- [x] VM execution for core statements and expressions
- [x] Inline and block functions with closures and default parameters
- [x] Local file imports
- [x] `Ask for <var>` input statement ✅
- [x] `Increase`/`Decrease` desugaring ✅
- [x] `minus` operator for subtraction ✅
- [x] `times` operator for multiplication ✅
- [x] `divided by` operator for division ✅
- [x] Modern collection literals: `[1, 2, 3]` lists, `{key: value}` dictionaries ✅
- [x] Collection indexing: `list[0]`, `dict["key"]`, negative indexing, nested indexing ✅
- [x] Enhanced error messages with helpful hints and suggestions ✅

**Remaining Tasks**:
- [ ] Complete test coverage (run all test cases via test harness)
- [ ] Comprehensive documentation for all features
- [ ] Performance benchmarks and optimization

**Deliverable**: The runtime can execute any core `.poh` program with excellent error messages.

---

## Phase 6: Standard Library & System Modules (Q1 2025)

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

## Phase 6.5: Bytecode Compiler & VM (Q2 2025)

**Goal**: Compile `.poh` to a compact bytecode format for faster execution and portability.

**Tasks**:
- [ ] Define stack-based bytecode ISA (see DESIGN.md)
- [ ] Implement bytecode compiler in `src/compiler.rs`
- [ ] Implement bytecode VM in `src/bytecode_vm.rs`
- [ ] File format: magic header, const pool, code section
- [ ] CLI: `pohlang --compile foo.poh -o foo.pbc`
- [ ] CLI: `pohlang --run-bytecode foo.pbc`
- [ ] Benchmarks: compare AST-walking vs. bytecode VM performance

**Deliverable**: Users can compile and distribute `.pbc` files; faster execution than AST walking.

---

## Phase 7: AOT Native Compilation (Q3-Q4 2026)

**Goal**: Produce standalone native executables from `.poh` source.

**Tasks**:
- [ ] **Stage 1 (static bundle)**: Generate a Rust program embedding bytecode via `include_bytes!`, then compile with `rustc` or `cargo`.
  - `pohlang --aot foo.poh -o foo.exe`
- [ ] **Stage 2 (Cranelift JIT/AOT)**: Lower bytecode to native code using Cranelift.
- [ ] Minimal runtime library linkable as a static archive
- [ ] Cross-compilation support (Linux, macOS, Windows, ARM)
- [ ] Strip debug info and optimize for size (release builds)

**Deliverable**: Single-file executables with no external dependencies; distributable binaries.

---

## Phase 8: Ecosystem & Tooling (Q4 2026 - 2027)

**Goal**: Publish PohLang as a standalone language with rich ecosystem.

**Tasks**:
- [ ] Publish v1.0.0 release with stable API
- [ ] CI: build and test on all platforms; publish release artifacts
- [ ] Binary distributions for Windows, macOS, Linux
- [ ] Integration with PLHub for complete development experience
- [ ] Package manager integration
- [ ] VS Code extension and LSP support

---

## Phase 9: Advanced Features & Ecosystem (2027+)

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

1. **Pure Rust implementation**: No external language dependencies.
2. **Native executables**: Users can compile and distribute `.exe`, `.app`, or ELF binaries.
3. **Performance**: Bytecode VM is 10x+ faster than AST walking; AOT is comparable to compiled languages.
4. **Usability**: Clear error messages, good docs, easy installation (one binary).
5. **Adoption**: Used in educational settings and by hobbyists; active community.

---

**Current Status** (October 2025):
- ✅ **Phase 1-4**: COMPLETE (Core language features, collections, operators, control flow)
- ✅ **Phase 5**: COMPLETE (Error handling with try/catch/finally, 7 error types, v0.5.4 released)
- ✅ **VS Code Extensions**: Published v0.2.5 (PohLang Language Support + PLHub)
- 🚀 **Phase 6**: IN PROGRESS (Standard Library modules)
- 🔥 **Phase 7 (NEW)**: IN PROGRESS (Web Framework - 80% complete!)

**Actual Implementation Order** (Prioritizing practical applications):
1. ✅ Phase 1-4: Core Language (2024-2025)
2. ✅ Phase 5: Error Handling (October 2025)
3. ✅ VS Code Extensions v0.2.5 with IntelliSense (October 2025)
4. 🔥 **Phase 7: Web Framework (October 2025) - CURRENT PRIORITY**
   - ✅ HTTP server implementation (http.rs) - 325 lines
   - ✅ Template engine (template.rs) - 275 lines
   - ✅ JSON support (network.rs) - Pre-existing
   - 🚧 Parser integration (in progress)
   - ⏳ VM integration
   - ⏳ Task Master web app demo
5. 🔥 Phase 6: Standard Library modules (Q4 2025 - Q1 2026)
6. ⏳ Phase 6.5: Bytecode Compiler & VM (Q1-Q2 2026)
7. ⏳ Phase 7: AOT Native Compilation (Q2-Q3 2026)
8. ⏳ Phase 8: Advanced Features & Ecosystem (2026+)

**Next Immediate Steps**:
1. ✅ Windows MSVC toolchain compatibility - COMPLETE
2. ✅ Core features fully implemented - COMPLETE  
3. ✅ Error handling system - COMPLETE
4. ✅ VS Code extensions published - COMPLETE
5. ✅ HTTP server & template engine - COMPLETE
6. 🔥 **Web framework parser integration (TODAY)**
7. 🔥 **First web app in PohLang (THIS WEEK)**
8. ⏳ Complete standard library modules
9. ⏳ Bytecode compiler and VM

**See [PHASE_6_PLAN.md](../PHASE_6_PLAN.md) for detailed implementation plan.**
