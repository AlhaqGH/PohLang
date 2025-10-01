# PohLang

A beginner-focused, fully phrasal (English-like) programming language designed to be a **real compiled language**—not a script on top of another runtime. PohLang is transitioning from Dart to a standalone Rust toolchain, aiming for native executables and full independence.

📘 Full documentation: see the comprehensive guide at [PohLang_Guide.md](./PohLang_Guide.md).  
🗺️ **Transition Plan**: See [TRANSITION.md](./TRANSITION.md) for the roadmap to becoming a fully independent compiled language.  
🚀 **Roadmap**: See [ROADMAP.md](./ROADMAP.md) for detailed milestones.

## Release Information
- **PohLang Core**: v0.1.0 (First Experimental Release)
- **Python Interpreter**: v0.5.0 (Stable—reference implementation)
- **Rust Runtime (pohlangc)**: v0.5.0 (Experimental—future primary toolchain)
- **Dart Code**: Legacy (being phased out)

## Goals
- Use plain-language statements: `Write "Hello"`, `Set count to 5`, `Repeat 3 ... End`.
- Keep one clear form per concept (no synonyms like Print/Show). Use `Write`, not `Say/Print`.
- Provide a gentle path to programming concepts (variables, loops, conditionals, functions) without punctuation noise.
- **Become a real compiled language**: native executables, no external runtime dependencies.
- **Eliminate Dart dependency**: transition to a pure Rust toolchain by Q4 2026.

## Example
```
Ask for name
Write "Hello " plus name
Set count to 3
Repeat 3
	 Write "Hi"
End
If count is greater than 1 Write "Many" Otherwise Write "Few"
Make greet with who Write "Hi " plus who
Use greet with "Poh"
```

## Running a Program

### Option A: Python Interpreter (Stable—Recommended for now)
1. Install: `pip install pohlang`
2. Run: `pohlang examples/poh/hello.poh`
3. Or from source: `python -m Interpreter.run_poh examples/poh/hello.poh`

### Option B: Rust Runtime/VM (Experimental—Future Primary Toolchain)

**Prerequisites (Windows)**: Install Visual Studio Build Tools with C++ workload. See `runtime-rs/README.md` for details.

1. Build Rust crate:
   ```bash
   cargo build --manifest-path runtime-rs/Cargo.toml
   ```
2. Run a program with the VM:
   ```bash
   cargo run --manifest-path runtime-rs/Cargo.toml -- --run examples/poh/hello.poh
   ```
3. Or use the compiled binary:
   ```bash
   target/debug/pohlangc --run examples/poh/hello.poh
   ```

**Status**: The Rust runtime is under active development. Not all features are implemented yet. See [ROADMAP.md](./ROADMAP.md) for progress.

## Current Features
- Write, Ask for, Set, Increase, Decrease
- If/Otherwise blocks and inline If
- Repeat and While loops
- Function definition (Make/End) and calls (Use or expression calls)
- Expression support: identifiers, numbers, strings, booleans; plus/minus/times; comparisons like `is greater than`, `is less than`, `is equal to`
- Desugaring for increase/decrease
 - File and process helpers (selected operations via runtime)

## Roadmap

### Short-term (Q4 2025)
- Rust runtime feature parity with Python interpreter
- `Ask for` input, `Increase`/`Decrease` desugaring
- Enhanced error messages

### Mid-term (Q1-Q2 2026)
- Standard library in Rust (collections, random, datetime)
- Bytecode compiler and VM
- Performance benchmarks

### Long-term (Q3-Q4 2026)
- AOT native compilation (`pohlangc --aot foo.poh -o foo.exe`)
- Remove Dart dependency completely
- Release standalone binaries for all platforms
- **v1.0.0**: PohLang as a real, independent compiled language

See [ROADMAP.md](./ROADMAP.md) for detailed plans.

## Directory Structure
```
doc/             Language syntax and vocabulary
examples/        Sample PohLang programs (.poh)
Interpreter/     Python reference interpreter and CLI (stable)
runtime-rs/      Rust runtime/VM (pohlangc) — future primary toolchain
tests_python/    Python test suite
.github/         CI workflows (builds Rust runtime on all platforms)
TRANSITION.md    Transition guide: moving from Dart to Rust
ROADMAP.md       Detailed roadmap and milestones
```

## Contributing

We're actively building the Rust toolchain! Contributions are welcome:

- **Test the Rust runtime**: Run your `.poh` programs with `pohlangc` and report issues.
- **Add features**: Implement missing statements, built-in functions, or stdlib modules.
- **Write tests**: Add test cases in `runtime-rs/tests/` or `tests_python/`.
- **Documentation**: Improve error messages, guides, tutorials.
- **Spread the word**: Share PohLang with educators and learners!

Open to experiments—keep syntax consistent and simple. Prefer the phrasal form (`plus`, `minus`, `times`, `is greater than`). Avoid adding synonyms for core statements unless backed by learning outcomes.

See [TRANSITION.md](./TRANSITION.md) for how to get involved.

## License
MIT
