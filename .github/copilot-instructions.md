## Copilot Instructions for PohLang

Purpose: enable fast, correct edits. PohLang is a tiny, fully phrasal English-like language that transpiles to Dart.

Architecture (read these first)
- `src/ast.dart`: AST node types (Program; Print/Input/Assign/If/Repeat/FunctionDef/Return/Call; Binary/Literal/Identifier/Call exprs).
- `src/parser.dart`: single-pass, line-based phrasal parser (one statement per line; trims blanks).
- `src/transpiler.dart`: CLI — reads `.poh`, parses to AST, emits Dart alongside the input file.
- `src/runtime.dart`: small helpers used by emitted Dart (I/O, random).
- Docs in `docs/` (syntax, vocabulary); examples in `examples/`.

Language subset (as implemented)
- Statements:
    - `Write <expr>`
    - `Ask for <name>`
    - `Set <name> [to] <expr>`
    - `Increase <name> [by] <n>` / `Decrease <name> [by] <n>`
    - `If <cond> Write <expr> [Otherwise Write <expr>]`
    - `Repeat <count> [times] Write <expr>`
- Functions: `Make …` and `Use …` are parsed but not fully emitted (see caveats).
- Expressions: identifiers, numbers, strings, `+`, `-`, basic `*`, verbal comparisons (`is greater than`, `is equal to`, `is less than`, `is not equal to`). Single `=` inside conditions is treated as equality.

Emitter rules and caveats
- Emits a `main()` that executes statements in order.
- First assignment/ask declares the variable once: `var name = ...`; subsequent writes omit `var`.
- `Ask for x` → `x = PohRuntime.inputText('x');` (string).
- Inline If/Repeat expand to normal Dart `if`/`for`.
- Functions are not emitted yet: `FunctionDefStmt` becomes a comment; `CallStmt` still emits `name(args);` and will not compile until functions are implemented.
- Runtime import path is computed relative to the output file automatically (no manual fixups). For example, code emitted to `examples/` will import `../src/runtime.dart`.
- Phrasal only: legacy `examples/symbol_*.poh` won’t parse.

Workflow
- Setup: run `dart pub get` at repo root.
- Transpile: `dart run src/transpiler.dart examples/phrase_repeat.poh`.
- Run the generated Dart: `dart run examples/phrase_repeat.dart`.
- Dev loop: edit `.poh` → re-run transpiler → run updated `.dart`.

Conventions
- Exactly one statement per line; no multi-line blocks yet; `Otherwise` is inline.
- No comments; strings use double quotes; keep vocabulary singular (use `Write`, not Print/Show/Say).
- Identifiers: start with a letter; letters/digits/underscore.

Extending safely
1) Add a node in `ast.dart`.
2) Parse it in `Parser._parsePhraseLine`.
3) Emit it in `_emitStmt` and/or `_emitExpr` within `src/transpiler.dart`.
4) Add runtime helpers in `src/runtime.dart` if needed.
- To finish functions: collect function definitions, emit Dart functions before `main()`, then emit calls normally.

Known gaps / gotchas
- Functions not emitted yet (calls will not resolve).
- Expressions are simple: basic `*` but no parentheses and limited precedence.
- Errors are generic `FormatException`s without line/column.
- `Ask for` returns a string; there is no implicit numeric parsing yet in emitted code.

Pointers
- Prefer code over docs if they ever disagree. Quick trace: `examples/phrase_repeat.poh` → parse → emit → run.
