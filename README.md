# PohLang

A beginner-focused, fully phrasal (English-like) programming language designed to be a **real co### 🚀 Option 2: VS Code Extension (Best IDE Experience)

**For VS Code users** - Get syntax highlighting, IntelliSense, snippets, and integrated runtime without leaving VS Code.

1. Install [VS Code](https://code.visualstudio.com/)
2. Install [PohLang Hub extension](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)
3. Create a `.poh` file and start coding!
4. Press `Ctrl+F5` to run your program

**Why use the extension:**
- ✅ **Syntax highlighting** for `.poh` files
- ✅ **IntelliSense & completions** with parameter hints
- ✅ **40+ code snippets** for rapid development
- ✅ **Bundled runtime** - No separate download needed
- ✅ **One-click execution** - Press `Ctrl+F5` to run
- ✅ **Local VS Code support** - Works offline once installed
- ✅ **Improved VS Code compatibility** - Native `.poh` file support

**Note:** The extension includes its own copy of the PohLang runtime, so you don't need to download binaries from GitHub separately. Perfect for VS Code users who want everything in one place!

### 🔧 Option 3: Using PLHub (Advanced)e**—not a script on top of another runtime. PohLang is built with a standalone Rust toolchain, aiming for native executables and full independence.

[![VS Code Extension](https://img.shields.io/visual-studio-marketplace/v/pohlang.pohlang-hub?label=VS%20Code%20Extension&color=blue)](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)
[![VS Code Installs](https://img.shields.io/visual-studio-marketplace/i/pohlang.pohlang-hub)](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

📘 Full documentation: see the comprehensive guide at [PohLang_Guide.md](./doc/PohLang_Guide.md).  
🗺️ **Roadmap**: See [ROADMAP.md](./doc/ROADMAP.md) for detailed milestones.  
🔌 **VS Code Extension**: Get the full IDE experience with [PohLang Hub](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)!  
📦 **Installation**: See [INSTALL.md](./INSTALL.md) for one-line install commands!

> **Note**: This repository contains the **core Rust runtime only**. For the complete development environment including CLI tools, package management, and editor integration, see [PLHub](https://github.com/AlhaqGH/PLHub).

## Repository Overview

**PohLang** = The core language runtime (this repository)  
**PLHub** = The complete development environment (separate repository)

Think of it like:
- **PohLang** is to **Rust** (the language core)
- **PLHub** is to **Cargo** (the development tooling)

## Release Information
- **PohLang Rust Runtime**: v0.5.4 (**Phase 5 Complete** ✅)
- **Core Language**: Phase 5 Production-Ready (Error Handling)
- **Test Coverage**: 50+ passing tests (100% core features + error handling)
- **New in v0.5.4**: Comprehensive error handling with try/catch/finally, custom error types, natural English error messages, and file location reporting
- **Previous (v0.5.2)**: Symbolic operators (+, -, *, /, >, <, ==, etc.) alongside phrasal forms
- ⚠️ **Known Issues**: See [KNOWN_ISSUES.md](KNOWN_ISSUES.md) for issues and workarounds

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

## Quick Start

### 🎯 Option 1: Download Pre-built Binary (Recommended for End Users)

**For End Users** - Just download and run! No installation of Rust, Visual Studio, or build tools required.

#### Windows

**One-line install (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.ps1 | iex
```

Or download manually:
1. Download the latest release: [pohlang-v0.5.2-windows-x64.zip](https://github.com/AlhaqGH/PohLang/releases/latest)
2. Extract the zip file
3. Open PowerShell or Command Prompt in the extracted folder
4. Run: `.\pohlang.exe --run examples\hello.poh`

SDK bundle: [pohlang-sdk-v0.5.2-windows-x64.zip](https://github.com/AlhaqGH/PohLang/releases/latest)

#### Linux

**One-line install:**
```bash
curl -sSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash
```

Or download manually:
```bash
# Download and extract
wget https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-v0.5.2-linux-x64.tar.gz
tar -xzf pohlang-v0.5.2-linux-x64.tar.gz

# Run PohLang
./pohlang --run examples/hello.poh

# Optional: Add to PATH for system-wide access
sudo mv pohlang /usr/local/bin/
```

SDK bundle: [pohlang-sdk-v0.5.2-linux-x64.tar.gz](https://github.com/AlhaqGH/PohLang/releases/latest)

#### macOS

**One-line install:**
```bash
curl -sSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash
```

Or download manually:
```bash
# Download and extract
curl -L https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-v0.5.2-macos-x64.tar.gz -o pohlang.tar.gz
tar -xzf pohlang.tar.gz

# Run PohLang
./pohlang --run examples/hello.poh

# Optional: Add to PATH
sudo mv pohlang /usr/local/bin/
```

SDK bundle: [pohlang-sdk-v0.5.2-macos-x64.tar.gz](https://github.com/AlhaqGH/PohLang/releases/latest)

**Creating Your First Program:**
```bash
# Create a file called hello.poh
echo 'Start Program
Write "Hello from PohLang!"
Set count to 5
Write "Count: " plus count
End Program' > hello.poh

# Run it
pohlang --run hello.poh
```

### 🚀 Option 2: VS Code Extension (Best for Development)
1. Install [VS Code](https://code.visualstudio.com/)
2. Install [PohLang Hub extension](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)
3. Create a `.poh` file and start coding!
4. Press `Ctrl+F5` to run your program

**Features included:**
- ✅ Syntax highlighting
- ✅ IntelliSense & completions
- ✅ 40+ code snippets
- ✅ Integrated runtime (no separate installation!)
- ✅ One-click execution

### � Option 3: Using PLHub (Advanced)
For a complete development environment with project management, package system, and CLI tools:
1. Install [PLHub](https://github.com/AlhaqGH/PLHub)
2. Use: `plhub run examples/poh/hello.poh`

---

## For Developers: Building from Source

**⚠️ Note**: Building from source requires Rust toolchain. End users should use pre-built binaries above.

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- On Windows: Visual Studio Build Tools with C++ workload (see `runtime/README.md`)

### Building
```bash
# Clone the repository
git clone https://github.com/AlhaqGH/PohLang.git
cd PohLang

# Build the runtime
cargo build --manifest-path runtime/Cargo.toml

# For optimized release build
cargo build --release --manifest-path runtime/Cargo.toml
```

### Running
```bash
# Run a program directly (development)
cargo run --manifest-path runtime/Cargo.toml -- --run examples/poh/hello.poh

# Or use the compiled binary
./target/debug/pohlang --run examples/poh/hello.poh

# With release build (faster)
./target/release/pohlang --run examples/poh/hello.poh
```

**Status**: The Rust runtime is under active development. Core features are implemented. See [ROADMAP.md](./ROADMAP.md) for progress.

## Current Features

### Core Language (Phase 1 Complete ✅)
- ✅ **Statements**: Write, Ask for, Set, Increase, Decrease, Return
- ✅ **Control Flow**: If/Otherwise blocks, inline If, While loops, Repeat loops
- ✅ **Functions**: Define with parameters, default values, closures, phrasal calls
- ✅ **Operators**: 
  - **Phrasal forms**: `plus`, `minus`, `times`, `divided by`, `is greater than`, `is less than`, `is equal to`
  - **Symbolic forms**: `+`, `-`, `*`, `/`, `>`, `<`, `>=`, `<=`, `==`, `!=`
  - **Both forms work!** Mix phrasal and symbolic operators freely
- ✅ **Logical**: `And`, `Or`, `Not`
- ✅ **Collections**: Lists `[1, 2, 3]`, Dictionaries `{key: value}`, Indexing with negative support
- ✅ **Imports**: Local file imports, system module stubs

### Error Handling (Phase 5 Complete ✅)
- ✅ **Try/Catch/Finally**: Natural English syntax (`try this:`, `if error as e`, `finally:`)
- ✅ **Error Types**: 7 built-in types (RuntimeError, TypeError, MathError, FileError, JsonError, NetworkError, ValidationError) + custom types
- ✅ **Type-Specific Catching**: `if error of type "FileError" as err`
- ✅ **Natural Error Messages**: "Error occurred: a file error - message"
- ✅ **Error Creation**: `error of type "ValidationError" with message "Invalid input"`
- ✅ **Throw Errors**: `throw err` or `throw "Error message"`
- ✅ **Error Operations**: Extract message and type from errors
- ✅ **File Location Reporting**: Errors show filename context

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
