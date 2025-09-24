# PohLang

A beginner-focused, fully phrasal (English-like) language with an authoritative Python interpreter and a standalone Rust runtime/VM. There is no symbol-based mode: every program is written as readable commands.

📘 Full documentation: see the comprehensive guide at [PohLang_Guide.md](./PohLang_Guide.md).

## Release Information
- **PohLang Core**: v0.1.0 (First Experimental Release)
- **Python Interpreter**: v0.5.0 (Stable)
- **Rust Runtime (pohlangc)**: v0.5.0 (Experimental VM)

## Goals
- Use plain-language statements: `Write "Hello"`, `Set count to 5`, `Repeat 3 ... End`.
- Keep one clear form per concept (no synonyms like Print/Show). Use `Write`, not `Say/Print`.
- Provide a gentle path to programming concepts (variables, loops, conditionals, functions) without punctuation noise.
-- Keep a simple path from readable phrases to execution via the Python interpreter or the Rust VM.

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

Option A: Python interpreter (recommended)
1. Install: pip install pohlang
2. Run: pohlang examples/poh/hello.poh
3. Or: python -m Interpreter.run_poh examples/poh/hello.poh (from a source checkout)

Option B: Rust runtime/VM (experimental)
1. Build Rust crate:
   - cargo build --manifest-path runtime-rs/Cargo.toml
2. Run a program with the VM:
   - target/debug/pohlangc --run examples/poh/hello.poh

## Current Features
- Write, Ask for, Set, Increase, Decrease
- If/Otherwise blocks and inline If
- Repeat and While loops
- Function definition (Make/End) and calls (Use or expression calls)
- Expression support: identifiers, numbers, strings, booleans; plus/minus/times; comparisons like `is greater than`, `is less than`, `is equal to`
- Desugaring for increase/decrease
 - File and process helpers (selected operations via runtime)

## Roadmap
- Multi-line blocks with indentation (If / Repeat / Define)
- Lists, dictionaries, predicates (is even, is greater than)
- Random numbers, timers, date utilities
- Error diagnostics with suggestions & line numbers
- Optional extensions: Islamic-friendly helpers (prayer times, Hijri date, Quran recitation)
- Flutter integration examples

## Directory Structure
```
doc/             Language syntax and vocabulary
examples/        Sample PohLang programs (.poh)
Interpreter/     Python reference interpreter and CLI
runtime-rs/      Rust runtime/VM (pohlangc)
tests_python/    Python test suite
```

## Contributing
Open to experiments—keep syntax consistent and simple. Prefer the phrasal form (`plus`, `minus`, `times`, `is greater than`). Avoid adding synonyms for core statements unless backed by learning outcomes.

## License
MIT
