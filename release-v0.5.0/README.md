# PohLang

A beginner-focused, fully phrasal (English-like) programming language designed to be a **real compiled language**—not a script on top of another runtime. PohLang is built with a standalone Rust toolchain, aiming for native executables and full independence.

📘 Full documentation: see the comprehensive guide at [PohLang_Guide.md](./PohLang_Guide.md).  
🗺️ **Roadmap**: See [ROADMAP.md](./ROADMAP.md) for detailed milestones.

> **Note**: This repository contains the **core Rust runtime only**. For the complete development environment including CLI tools, package management, and editor integration, see [PLHub](https://github.com/AlhaqGH/PLHub).

## Repository Overview

**PohLang** = The core language runtime (this repository)  
**PLHub** = The complete development environment (separate repository)

Think of it like:
- **PohLang** is to **Rust** (the language core)
- **PLHub** is to **Cargo** (the development tooling)

## Release Information
- **PohLang Rust Runtime**: v0.5.0 (**Phase 1 Complete** ✅)
- **Core Language**: Phase 1 Production-Ready
- **Test Coverage**: 50 passing tests (100% core features)

## Goals
- Use plain-language statements: `Write "Hello"`, `Set count to 5`, `Repeat 3 ... End`.
- Keep one clear form per concept (no synonyms like Print/Show). Use `Write`, not `Say/Print`.
- Provide a gentle path to programming concepts (variables, loops, conditionals, functions) without punctuation noise.
- **Become a real compiled language**: native executables, no external runtime dependencies.
- **Pure Rust implementation**: Fast, safe, and portable runtime.

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

### Prerequisites (Windows)
Install Visual Studio Build Tools with C++ workload. See `runtime/README.md` for details.

### Building the Runtime
```bash
# Build the Rust runtime
cargo build --manifest-path runtime/Cargo.toml

# For optimized release build
cargo build --release --manifest-path runtime/Cargo.toml
```

### Running Programs
```bash
# Run a program directly (development)
cargo run --manifest-path runtime/Cargo.toml -- --run examples/poh/hello.poh

# Or use the compiled binary
./target/debug/pohlang --run examples/poh/hello.poh

# With release build (faster)
./target/release/pohlang --run examples/poh/hello.poh
```

### Using PLHub (Recommended for Development)
For a complete development environment with project management, package system, and CLI tools:
1. Install [PLHub](https://github.com/AlhaqGH/PLHub)
2. Use: `plhub run examples/poh/hello.poh`

**Status**: The Rust runtime is under active development. Core features are implemented. See [ROADMAP.md](./ROADMAP.md) for progress.

## Current Features

### Core Language (Phase 1 Complete ✅)
- ✅ **Statements**: Write, Ask for, Set, Increase, Decrease, Return
- ✅ **Control Flow**: If/Otherwise blocks, inline If, While loops, Repeat loops
- ✅ **Functions**: Define with parameters, default values, closures, phrasal calls
- ✅ **Operators**: Arithmetic (`plus`, `minus`, `times`, `divided by`), Comparisons (`is greater than`, `is less than`, `is equal to`), Logical (`And`, `Or`, `Not`)
- ✅ **Collections**: Lists `[1, 2, 3]`, Dictionaries `{key: value}`, Indexing with negative support
- ✅ **Imports**: Local file imports, system module stubs

### Phrasal Built-in Expressions (20 total) ✅
**Mathematical**: `total of`, `smallest in`, `largest in`, `absolute value of`, `round`, `round down`, `round up`

**String**: `make uppercase`, `make lowercase`, `trim spaces from`

**Collection**: `first in`, `last in`, `reverse of`, `count of`, `join with`, `split by`, `contains in`, `remove from`, `append to`, `insert at`

**Aliases**: `size of`, `separate by`, `reverse`, `clean spaces from`

### Grammar & Quality ✅
- ✅ **Unambiguous grammar** with proper operator precedence
- ✅ **50 passing tests** covering all features
- ✅ **Enhanced error messages** with suggestions
- ✅ **Comprehensive documentation** (Guide, Vocabulary, Grammar Analysis)
- ✅ **Centralized phrase management** for easy extension

## Roadmap

### Phase 1: Core Features (Q4 2025) - ✅ **COMPLETE**
- ✅ Parser for all statement types
- ✅ VM execution for core statements
- ✅ Collections and indexing
- ✅ 20 phrasal built-in expressions
- ✅ Unambiguous grammar with proper precedence
- ✅ Enhanced error messages
- ✅ Complete test coverage (50 tests passing)
- ✅ Production-ready v0.5.0 release

### Phase 2: Standard Library (Q1 2026)
- Standard library modules in Rust
- Collections, random, datetime, math
- File I/O operations
- Islamic calendar utilities

### Phase 3: Bytecode Compiler & VM (Q2 2026)
- Compact bytecode format
- Bytecode compiler and VM
- Performance optimization
- `.pbc` file format

### Phase 4: AOT Native Compilation (Q3-Q4 2026)
- Native executable generation
- Cross-platform compilation
- Standalone binaries
- **v1.0.0**: Production-ready release

See [ROADMAP.md](./ROADMAP.md) for detailed plans.

## Directory Structure
```
runtime/         Rust runtime/VM implementation
  src/
    parser/      Lexer, parser, AST
    vm/          Virtual machine execution
    core/        Built-in functions
  tests/         Rust test suite
examples/poh/    Sample PohLang programs (.poh)
spec/            Language specification
doc/             Language syntax and vocabulary
.github/         CI workflows (builds runtime on all platforms)
ROADMAP.md       Detailed roadmap and milestones
```

## Related Projects
- **[PLHub](https://github.com/AlhaqGH/PLHub)**: Complete development environment with CLI tools, package management, project templates, and editor integration.

## Contributing

We're actively building the Rust runtime! Contributions are welcome:

- **Test the runtime**: Run your `.poh` programs and report issues
- **Add features**: Implement missing statements, built-in functions, or stdlib modules
- **Write tests**: Add test cases in `runtime/tests/`
- **Documentation**: Improve error messages, guides, tutorials
- **Spread the word**: Share PohLang with educators and learners!

Open to experiments—keep syntax consistent and simple. Prefer the phrasal form (`plus`, `minus`, `times`, `is greater than`). Avoid adding synonyms for core statements unless backed by learning outcomes.

### Development Setup
```bash
# Clone the repository
git clone https://github.com/AlhaqGH/PohLang.git
cd PohLang

# Build and test
cargo build --manifest-path runtime/Cargo.toml
cargo test --manifest-path runtime/Cargo.toml

# Run examples
cargo run --manifest-path runtime/Cargo.toml -- --run examples/poh/hello.poh
```

## License
MIT
