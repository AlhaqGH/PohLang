# PohLang Rust Runtime (pohlangc)

This crate is the new Rust-based runtime and compiler for PohLang.
It removes the Dart dependency and aims to become the standalone engine for the language.

## Prerequisites (Windows)

Rust for MSVC requires the Visual  linker (link.exe). Install the following:

- Visual Studio Build Tools (2019 or later) with the "Desktop development with " workload
- Windows 10/11 SDK
- Rust toolchain (stable, MSVC target)

Quick setup using winget:

1. Install VS Build Tools ( workload):
   - Launch the installer GUI and select "Desktop development with "
   - Or use this command to install core build tools:

```
pwsh
winget install --id Microsoft.VisualStudio.2022.BuildTools --source winget
```

Then, in the installer, select:
- MSVC v143 - VS 2022 x64/x86 build tools
- Windows 10/11 SDK

2. Ensure Rust is installed with the MSVC toolchain:

```
pwsh
rustup default stable-x86_64-pc-windows-msvc
rustup update
```

3. Use the "Developer PowerShell for VS" or ensure `link.exe` is on PATH.

## Build

From the repo root:

```
pwsh
cargo build --manifest-path runtime-rs/Cargo.toml
```

Release build:

```
pwsh
cargo build --manifest-path runtime-rs/Cargo.toml --release
```

## Test

```
pwsh
cargo test --manifest-path runtime-rs/Cargo.toml
```

## Run a .poh file

```
pwsh
cargo run --manifest-path runtime-rs/Cargo.toml -- --run examples/poh/hello.poh
```

## CLI

- `--run <file.poh>`: parse and execute with the embedded VM
- `--compile <file.poh> [-o out.pbc]`: compile to placeholder bytecode
- `--aot <file.poh>`: reserved for future ahead-of-time compilation

## Status

- Parser and VM support: Write, Set, If (inline/block), While, Repeat, inline and block functions (with defaults), simple closures, imports (local), and a few built-ins (now, range, join, split, length).
- System imports are stubbed.
- Bytecode format is placeholder.

## Roadmap

- Flesh out bytecode VM (real opcodes)
- Standard library and modules
- Cross-platform CI builds and release artifacts
- Replace legacy Dart usage entirely
