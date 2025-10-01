# PohLang Transition Guide: Moving from Dart to Rust

**Date**: October 2025  
**Status**: In progress  
**Target Completion**: Q4 2026

---

## Overview

PohLang is transitioning from a Dart-dependent implementation to a **fully independent compiled language** with a native Rust toolchain. This document explains why, what's changing, and how to participate.

---

## Why the Change?

**Goals:**
1. **Independence**: Remove dependency on Dart runtime and VM.
2. **Performance**: Native compilation for faster execution.
3. **Portability**: Single-file executables for all platforms (Windows, macOS, Linux).
4. **Simplicity**: One toolchain (`pohlangc`) instead of multiple interpreters.
5. **Real Language**: Become a true compiled language like Rust, Go, or C—not just a script on top of another runtime.

**What this means:**
- PohLang will have its own compiler and virtual machine written in Rust.
- Programs will compile to bytecode or native executables.
- No more Dart runtime required.
- Python interpreter remains as a reference implementation during transition.

---

## Current State (October 2025)

### What Works Today

**Python Interpreter (`pohlang` package):**
- ✅ Full language support: variables, loops, conditionals, functions, imports
- ✅ Standard library: collections, random, datetime helpers
- ✅ Complete test suite
- ✅ Stable and recommended for production use

**Rust Runtime (`pohlangc` binary):**
- ✅ Core language features: Write, Set, If, Repeat, While, Make, Use, Import
- ✅ Expressions: numbers, strings, identifiers, plus, comparisons, logic operators
- ✅ Functions with closures and default parameters
- ✅ Local file imports
- ⚠️ **Missing**: Ask for input, Increase/Decrease, collections literals, standard library
- ⚠️ **Status**: Experimental—not ready for production

**Dart Code:**
- 🔶 Legacy code still present in codebase
- 🔶 Will be removed in Phase 5 (Q4 2026)

---

## Migration Path

### For Users

**Today (Q4 2025):**
- Use the Python interpreter: `pip install pohlang && pohlang yourfile.poh`
- Try the Rust runtime experimentally: `cargo run --manifest-path runtime-rs/Cargo.toml -- --run yourfile.poh`

**Q1-Q2 2026:**
- Rust runtime reaches feature parity
- Start testing your programs with `pohlangc --run`
- Report any bugs or missing features

**Q3 2026:**
- Use `pohlangc --compile` for bytecode or `pohlangc --aot` for native executables
- Distribute standalone binaries

**Q4 2026:**
- Python interpreter deprecated (but still available)
- `pohlangc` becomes the official toolchain
- Install via `cargo install pohlangc` or download binaries

### For Contributors

**How to Help:**

1. **Test the Rust runtime**: Run your `.poh` programs with `pohlangc --run` and report issues.
2. **Add features**: Implement missing statements (Ask for, Increase, Decrease) in `runtime-rs/src/parser.rs` and `vm.rs`.
3. **Port standard library**: Rewrite Python stdlib modules in Rust or as `.poh` libraries.
4. **Write tests**: Add test cases in `runtime-rs/tests/` for edge cases and examples.
5. **Documentation**: Improve error messages, write tutorials, update guides.

**Development Setup:**
- Install Rust (1.70+): `rustup default stable`
- Windows: Install Visual Studio Build Tools with C++ workload
- Build: `cargo build --manifest-path runtime-rs/Cargo.toml`
- Test: `cargo test --manifest-path runtime-rs/Cargo.toml`
- Run: `cargo run --manifest-path runtime-rs/Cargo.toml -- --run examples/poh/hello.poh`

---

## Breaking Changes

### Timeline

**Phase 1-2 (Q4 2025 - Q1 2026):** No breaking changes. Both interpreters coexist.

**Phase 3 (Q2 2026):** Introduce bytecode format (`.pbc` files). Source syntax unchanged.

**Phase 4 (Q3 2026):** Native compilation. May require minor syntax adjustments for AOT optimization.

**Phase 5 (Q4 2026):** Dart code removed. Python interpreter deprecated.

### Compatibility

- **Syntax**: PohLang syntax will remain stable. All `.poh` programs written today will work with `pohlangc`.
- **Standard library**: API-compatible reimplementation in Rust.
- **File imports**: `Import "file.poh"` works the same.
- **System imports**: `Import system "module"` will load native Rust modules instead of Python.

---

## FAQ

**Q: Will my existing `.poh` programs stop working?**  
A: No. The syntax is unchanged. Programs work with both Python and Rust runtimes.

**Q: Why not just keep using Python?**  
A: Python interpreter is great for development, but it's slow and requires Python runtime. Rust allows native executables and better performance.

**Q: Can I still use the Python interpreter?**  
A: Yes, it will remain available as a legacy option. But the Rust toolchain will be the official, maintained version.

**Q: What about Dart? Is it gone?**  
A: Dart code is being phased out. By Q4 2026, it will be completely removed.

**Q: How fast will compiled PohLang be?**  
A: Bytecode VM will be ~10x faster than AST walking. AOT compilation will approach Python's performance or better, depending on optimizations.

**Q: Can I contribute to the Rust runtime?**  
A: Absolutely! Check `runtime-rs/README.md` and `DESIGN.md`. Start with small tasks like adding tests or fixing parser edge cases.

**Q: Will there be a package manager?**  
A: Yes, planned for Phase 6 (2027+). Community can start designing it.

**Q: What about IDE support?**  
A: VS Code extension with syntax highlighting exists. LSP server planned for Phase 6.

---

## Resources

- **Roadmap**: See `ROADMAP.md` for detailed timeline and milestones.
- **Design**: See `runtime-rs/DESIGN.md` for technical architecture.
- **Build Instructions**: See `runtime-rs/README.md` for setup (especially Windows).
- **Language Guide**: See `PohLang_Guide.md` for syntax and semantics.
- **CI Status**: Check GitHub Actions for build/test status on all platforms.

---

## Get Involved

- **GitHub**: [github.com/AlhaqGH/PohLang](https://github.com/AlhaqGH/PohLang)
- **Issues**: Report bugs or request features
- **Discussions**: Ask questions, share ideas
- **Pull Requests**: Contribute code, tests, docs

---

**Let's build PohLang into a real, independent language together! 🚀**
