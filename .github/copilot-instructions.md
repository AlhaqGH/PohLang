## Copilot Instructions for PohLang

Purpose: Fast, correct edits. PohLang is a fully phrasal, English-like language that transpiles to Dart (with a Python interpreter for experimentation / tests).

### Current Architecture (after restructure)
- Dart transpiler core: `transpiler/src/`
    - `ast.dart` – AST node definitions (Program; statements: Print/Input/InputNumber/InputDecimal/Assign/If/Repeat/While/Stop/Skip/FunctionDef/Return/Call/File + FS ops; expressions: Literal/Identifier/Binary/Unary/Call).
    - `parser.dart` – single-pass, line-based parser (one statement per line). Handles imports expansion logic indirectly via transpiler helper.
    - `transpiler.dart` – CLI pipeline: parse, expand imports, emit Dart, optional run/compile. Handles strict-phrasal warnings.
- Public runtime (single source): `lib/runtime.dart` (also re-exported via `lib/pohlang.dart`). Transpiled programs import this either via `package:pohlang/runtime.dart` or a computed relative path fallback.
- Executables:
    - Root: `bin/pohlang.dart` (delegates to `transpiler/src/transpiler.dart`).
    - Legacy helper: `transpiler/bin/pohlang.dart` (still delegates; kept for backward compatibility).
- Examples: `examples/`
    - `poh/` – Source `.poh` examples.
    - `dart/` – Transpiled `.dart` examples and moved invoice demos.
    - `python/` – Interpreter-oriented examples.
- Tests:
    - Dart unit tests: `tests_dart/` (parser, control flow, arithmetic, strict warnings, IO constructs, etc.).
    - Python E2E tests: `tests_python/test_transpiler_e2e.py` (transpile + run integration; includes I/O and imports).
- Python interpreter (reference implementation): `Interpreter/` (AST, parser, interpreter, CLI). Its legacy test file now inert; real tests moved into `tests_python`.

### Language Subset
- Statements currently supported:
    - `Write <expr>`
    - `Ask for <name>` / `Ask for <name> number` / `Ask for <name> decimal`
    - `Set <name> [to] <expr>`
    - `Increase <name> [by] <n>` / `Decrease <name> [by] <n>`
    - `If <cond> ... Otherwise ... End` (multi-line) and inline `If <cond> Write ... Otherwise Write ...`
    - `Repeat <count>` ... `End`
    - `While <cond>` ... `End`
    - `Stop` (break), `Skip` (continue)
    - File system: (Open/Write/Delete file, List files, Change/Create/Delete directory) via dedicated statements
    - Process run statements (wait/background/plain) mapped to runtime helpers
    - Function forms: `Make` (definition) & `Use` / calls parsed; full emission for function definitions now implemented (definitions emitted before `main`).
- Expressions: identifiers, numbers, strings, booleans (`true`/`false`), binary ops `+ - * /` (limited precedence; basic times support), comparisons (`is greater than`, `is less than`, `is equal to`, `is not equal to`, `is at least`, etc.), unary negation.
- String concatenation auto-coerces when `+` used with any string operand.

### Emission Rules & Behavior
- Generates a single `main()` with sequential execution; function definitions are emitted above `main`.
- First assignment or input to a variable declares it (`var` once); subsequent uses reassign.
- Inputs:
    - `Ask for x` → `PohRuntime.inputText('x')`
    - `Ask for x number` → `PohRuntime.inputInt('x') ?? 0`
    - `Ask for x decimal` → `PohRuntime.inputDouble('x') ?? 0.0`
- Control constructs expand to idiomatic Dart (`if`, `for`, `while`).
- `Stop` → `break;`, `Skip` → `continue;` inside loops.
- Import expansion: `Import "foo.poh"` recursively inlines (cycle guarded) before emission.
- Runtime import logic in emitted code:
    1. Attempt `Isolate.resolvePackageUri` for `package:pohlang/runtime.dart`; if successful, embed concrete file: URI.
    2. Else, fall back to relative path from output directory to `lib/runtime.dart` (supports raw repo dev + temp dirs for tests).
- Functions: Function definitions now fully emitted (with implicit `return null;`). Calls generate `name(args);` or in expressions `name(args)`.

### Workflow (Updated)
1. Install dependencies: `dart pub get` (root).
2. Transpile (no run): `dart run bin/pohlang.dart examples/poh/phrase_repeat.poh --no-run`.
3. Run generated: `dart run examples/poh/phrase_repeat.dart` (or let transpiler run it by omitting `--no-run`).
4. Python E2E: `python tests_python/test_transpiler_e2e.py`.
5. Dev loop: edit `.poh` → transpile → run → adjust.

### Conventions
- One statement per line; blocks terminated by `End` (If/While/Repeat/Function).
- Inline conditional form allowed: `If condition Write "yes" Otherwise Write "no"`.
- Strings use double quotes; no comments inside `.poh` (but `#` in a line after code is treated as comment for strict warning scan only).
- Keep vocabulary phrasal: prefer words over symbols; `--strict-phrases` warns on symbolic `+ - * /`.

### Extending Safely
1. Add node in `ast.dart` (statement or expression).
2. Update parser (search for keyword handling; add in `_parseLine` or related helpers).
3. Teach emission in `_emitStmt` / `_emitExpr` (transpiler).
4. Add runtime helpers (only if needed) to `lib/runtime.dart`.
5. Add tests: 
     - Dart unit: create file in `tests_dart/`.
     - Cross-language behavior (if it depends on final runtime or process): augment `tests_python/test_transpiler_e2e.py`.

### Known Gaps / Future Work
- Expression precedence is simplistic (no parentheses grouping beyond current linear parse approach).
- Error reporting lacks line/column detail (generic `FormatException`).
- No type system; everything is `dynamic` at runtime; limited numeric parsing fallback defaults to 0 / 0.0.
- Potential duplication risk: ensure any runtime feature additions stay single-sourced (only modify `lib/runtime.dart`).
- Performance: Import expansion re-parses each imported file fully; acceptable for small examples.

### Testing Strategy
- Dart: `dart test tests_dart` (covers parsing, control flow, validation constraints, numeric and boolean logic, strict phrasal warnings).
- Python E2E: ensures the transpilations still execute end-to-end (I/O prompts, arithmetic, imports, functions, loops, conditionals, file operations if added).
- When adding new syntax, write a minimal Dart test first; if runtime semantics matter, add/extend an E2E case.

### Troubleshooting
- Package import failures inside generated temp dirs: fallback relative import should engage automatically; if not, check working directory when invoking the transpiler.
- Variables not declared: ensure first assignment uses `AssignStmt` or an input statement; incremental increases assume prior declaration.
- Inline If mis-parsed: verify spacing and presence of `Otherwise` on same line.

### Quick Trace
`examples/poh/phrase_repeat.poh` → `transpiler/src/parser.dart` → AST → `_expandImports` → `_emitDart` (functions + main + runtime import) → generated Dart file → executed by VM or compiled.

### Core Principle
Favor clarity + minimal hidden magic. All transformations are explicit; no implicit type conversions beyond string coercion in `+` concatenations.

---
If documentation and code disagree, treat `transpiler/src/` as the source of truth.
