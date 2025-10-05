# PohLang Rust Runtime

This crate is the Rust-based runtime and compiler for PohLang.
It provides a standalone engine for parsing and executing PohLang programs.

## Prerequisites (Windows)

Rust for MSVC requires the Visual C++ linker (link.exe). Install the following:

- Visual Studio Build Tools (2019 or later) with the "Desktop development with C++" workload
- Windows 10/11 SDK
- Rust toolchain (stable, MSVC target)

Quick setup using winget:

1. Install VS Build Tools (C++ workload):
   - Launch the installer GUI and select "Desktop development with C++"
   - Or use this command to install core build tools:

```pwsh
winget install --id Microsoft.VisualStudio.2022.BuildTools --source winget
```

Then, in the installer, select:
- MSVC v143 - VS 2022 C++ x64/x86 build tools
- Windows 10/11 SDK

2. Ensure Rust is installed with the MSVC toolchain:

```pwsh
rustup default stable-x86_64-pc-windows-msvc
rustup update
```

3. Use the "Developer PowerShell for VS" or ensure `link.exe` is on PATH.

## Build

From the repo root:

```pwsh
cargo build --manifest-path runtime/Cargo.toml
```

Release build:

```pwsh
cargo build --release --manifest-path runtime/Cargo.toml
```

## Test

```pwsh
cargo test --manifest-path runtime/Cargo.toml
```

## Run a .poh file

### Program Structure

PohLang programs must be wrapped in `Start Program` ... `End Program` blocks:

```pohlang
Start Program
Write "Hello World"
Set x to 42
Write x
End Program
```

### Running

```pwsh
cargo run --manifest-path runtime/Cargo.toml -- --run examples/poh/hello.poh
```

Or use the compiled binary:

```pwsh
./target/debug/pohlang --run examples/poh/hello.poh
```

## CLI Options

- `--run <file.poh>`: Parse and execute with the embedded VM
- `--compile <file.poh> [-o out.pbc]`: Compile to placeholder bytecode (experimental)
- `--aot <file.poh>`: Reserved for future ahead-of-time compilation

## Current Status

**Implemented Features**:
- ✅ Parser for all core statement types (Write, Set, If, While, Repeat, Make, Use, Import)
- ✅ VM execution for all core statements
- ✅ Collections: lists `[1, 2, 3]` and dictionaries `{key: value}`
- ✅ Indexing: `list[0]`, `dict["key"]`, negative indices
- ✅ Arithmetic: `plus`, `minus`, `times`, `divided by`
- ✅ Comparisons: `is greater than`, `is less than`, `is equal to`
- ✅ Functions: inline and block functions with closures and default parameters
- ✅ `Ask for` input statement
- ✅ `Increase`/`Decrease` desugaring
- ✅ Enhanced error messages with suggestions
- ✅ Local file imports
- 🚧 Built-in functions: `now()`, `range()`, `join()`, `split()`, `length()`
- 🚧 System imports (partial)

**In Development**:
- Standard library modules
- Bytecode compiler and VM
- AOT native compilation

## Roadmap

See [../ROADMAP.md](../ROADMAP.md) for detailed development plans:
- **Phase 1** (Q4 2025): Core features - ~90% complete
- **Phase 2** (Q1 2026): Standard library
- **Phase 3** (Q2 2026): Bytecode compiler & VM
- **Phase 4** (Q3-Q4 2026): AOT native compilation

## Contributing

See the main [README.md](../README.md) for contribution guidelines.
