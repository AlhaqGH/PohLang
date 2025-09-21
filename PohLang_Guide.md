# PohLang Guide (September 2025)

This guide teaches PohLang from the first steps to the most advanced features available in this repository today. It serves both users (writing PohLang programs) and developers (extending the language/transpiler/interpreter).

Contents
- What is PohLang?
- Quick Start
- Core Concepts
- Language Reference (Statements)
- Language Reference (Expressions)
- Collections (Immutable-by-default, Mutable opt-in)
- Built-in Functions
- Control Flow
- Functions
- Modules & Imports
- Debugging & Tracing
- Errors & Warnings
- CLI & Tooling
- Developer Reference (Architecture & Extending)
- Testing
- Migration Notes (Legacy forms)

---

## What is PohLang?

PohLang is a phrasal, English-like programming language that prioritizes readability and explicitness. It currently targets two runtimes:
- A Python interpreter for experimentation and tests.
- A Dart transpiler that emits runnable Dart code.

Programs are written in simple statements (one per line), with blocks delimited by `End`.

---

## Quick Start

- Write a script:
  ```
  Write "Hello, PohLang"
  ```
- Run with the Python interpreter (see repo CLI in `Interpreter/cli.py`) or use the Dart transpiler to generate and run Dart.

---

## Core Concepts

- One statement per line. Blocks end with `End`.
- Variables are created on first assignment with `Set`.
- Lexical scoping with block boundaries at `If`, `While`, `Repeat`, function bodies, and `Begin ... End`.
- Expressions support arithmetic, comparisons, boolean logic, predicates, random, list/dict operations, and function calls.
- Collections are immutable by default; use the keyword `mutable` to opt into mutability.
 - Functions are first-class values: you can store them in variables, pass them as arguments, return them from other functions, and call them via expression form. Closures are supported. Default parameters can be provided with `set to`.

---

## Language Reference (Statements)

The following statements are recognized at the start of a line. Keywords are case-insensitive (the guide shows canonical capitalization):

- Write
  - `Write <expr>`
  - Prints the textual representation of `<expr>`.

- Ask
  - `Ask for <name>`
  - `Ask for <name> number`
  - `Ask for <name> decimal`
  - Reads input and assigns to `<name>` (number → int with fallback 0, decimal → float with fallback 0.0, otherwise string).

- Set
  - `Set <name> to <expr>`
  - `Set <name> <expr>` (alias)
  - Assigns the evaluated expression to `<name>` in the appropriate scope.

- Increase / Decrease
  - `Increase <name> by <expr>` or `Increase <name> <expr>` (default 1)
  - `Decrease <name> by <expr>` or `Decrease <name> <expr>` (default 1)

- If
  - Block form:
    ```
    If <condition>
      ...
    Otherwise
      ...
    End
    ```
  - Inline form:
    `If <condition> Write <expr> Otherwise Write <expr>`

- While
  - Block form:
    ```
    While <condition>
      ...
    End
    ```
  - Inline form:
    `While <condition> Write <expr>`

- Repeat
  - Block form:
    ```
    Repeat <count-or-collection>
      ...
    End
    ```
    - If number: repeats N times.
    - If list: iterates elements; implicit `it` is each element.
    - If dictionary: iterates keys; implicit `it` is each key.
  - Inline form (write only): `Repeat <count-or-collection> Write <expr>`

- Stop / Skip
  - `Stop` breaks out of the nearest loop.
  - `Skip` continues to the next loop iteration.

- Begin / End (Anonymous Block)
  - `Begin` ... `End` introduces a new lexical block scope without control flow.

- Make (Function Definition)
  - Block form:
    ```
    Make <name> with a, b, c
      ...
      Return <expr>
    End
    ```
  - Inline form:
    `Make <name> with a, b Write <expr>` (implicit return)

- Return
  - `Return` or `Return <expr>` inside function bodies.

- Use (Function Call)
  - `Use <name> with arg1, arg2, ...`
  - `Call <expr> with arg1, arg2, ...` (first‑class function call)
  - Expression call form is also supported: `f(arg1, arg2, ...)` where `f` is a function value.

- Import
  - `Import "path/to/file.poh"`
  - Executes another PohLang file in its own module scope; exports made available via module registry and injected for module-to-module imports.

- Debug
  - `Debug on` / `Debug off` toggles runtime tracing.

- List-specific statements
  - `Set the <N> item in <list> to <value>` (1-based index)
  - `Remove the last item from <list>` (no-op on empty)
  - `Add <value> to <list>`
  - `Remove <value> from <list>`

- Dictionary-specific statements
  - `Add "key": <value> to <dict>`
  - `Remove "key" from <dict>`

---

## Language Reference (Expressions)

- Literals
  - Numbers: `123`, `12.34`
  - Strings: `"text"` (supports `\n`, escaped quotes `\"`)
  - Booleans: `true`, `false`
  - Nothing: `nothing` (evaluates to null/None)

- Identifiers
  - Refer to variables or functions by name.

- Arithmetic
  - Phrasal operators are preferred in examples: `plus`, `minus`, `times` (and concatenation with `plus`). The symbols `+`, `-`, `*`, `/` may be accepted but are discouraged when writing PohLang. String concatenation uses `plus`.

- Comparisons
  - `is`, `is not`, `is greater than`, `is less than`, `is at least`, `is at most` (normalized internally to `==`, `!=`, `>`, `<`, `>=`, `<=`).

- Boolean logic
  - `and`, `or`, `not` (normalized to `&&`, `||`, `!`).

- Predicates (numeric)
  - `x is even`, `x is odd`, `x is positive`, `x is negative`.

- Random
  - `random between A and B` → integer
  - `random decimal between A and B` → float
  - `random from <collection>` → random element or value

- Collections
  - Lists (immutable): `Make a list of 1, 2, and 3` → immutable list (tuple)
  - Lists (mutable): `Make a mutable list of 1, 2, and 3`
  - Dictionaries (immutable): `Make a dictionary with "a" as 1 and "b" as 2` → frozen dict
  - Dictionaries (mutable): `Make a mutable dictionary with "a" as 1 and "b" as 2`

- Collection access
  - `Take the <N> item from <list>` (1-based index)
  - `Take the value of <key> from <dict>`
  - Postfix `at` operator (advanced): `d at "a"`, `xs at 1` (1-based for the phrasal Nth and 0-based for direct index access; prefer the phrasal forms for clarity)
  - `keys of <dict>`, `values of <dict>`
  - `contains <item> in <collection>` or `Check if <dict> has <key>` returns boolean

---

## Collections (Immutable-by-default, Mutable opt-in)

- Default behavior
  - Lists and dictionaries are immutable unless you use the `mutable` keyword when creating them.
  - Immutable lists are represented internally as tuples; immutable dictionaries as frozen dictionaries.

- Mutable opt-in
  - `Make a mutable list of ...`
  - `Make a mutable dictionary with ...`

- Legacy literal expressions
  - `List contains 1, 2, 3`
  - `Dictionary contains "a": 1`
  - These are supported for migration. They evaluate to wrappers that allow mutation but emit a Warning on mutation:
    - `Warning: Implicit mutable list/dictionary is deprecated. Use 'mutable list/dictionary' instead.`

- Mutation operations and constraints
  - Lists: Add/Remove value; Set Nth; Remove last.
  - Dictionaries: Add/Remove key.
  - Mutating an immutable collection raises a clear runtime error advising to create a mutable collection.

---

## Built-in Functions

- `length(x)` → number; works for strings, lists, dictionaries.
- `sum(list)` → number; elements must be numeric.
- `min(list)` / `max(list)` → number; non-empty numeric list.
- `range(n)` / `range(start, end[, step])` → list of numbers.
- `join(list, sep?)` → string; coerces elements to strings.
- `split(text, sep)` → list.
- `now()` → ISO-8601 timestamp.

---

## Control Flow

- If / Otherwise / End
- While / End
- Repeat / End
  - Repeat with a number repeats N times.
  - Repeat over a list/dict iterates with implicit `it` inside the loop.
- Stop / Skip inside loops.

---

## Functions

- Define with `Make`, call with `Use` (statement), `Call f with ...`, or by expression `f(args)`.
- Inline form returns the expression value; block form uses `Return`.
- Default parameters can be declared with `set to` in the function header.
- Exact arity enforcement; mismatch raises a runtime error.
- Lexical scoping for locals and closures is respected (closures capture surrounding variables when functions are created).

### First‑Class Functions
Functions are values that you can store, pass, and return.

```
Make greet with who Write "Hi " plus who
Set f to greet
Write f("Alice")
Call f with "Bob"
Use greet with "Poh"
```

### Closures
Functions capture variables from their defining scope.

```
Make makeAdder with x
  Make inner with y
    Return x plus y
  End
  Return inner
End

Set add2 to makeAdder(2)
Write add2(3)   # 5
```

### Default Parameters
Provide defaults using `set to` in the header. Omitted arguments use defaults.

Inline form:
```
Make greet with name set to "World" Write "Hello " plus name
Call greet              # Hello World
Call greet with "Alice" # Hello Alice
```

Block form:
```
Make greet with name set to "World"
  Write "Hello " plus name
End
```

---

## Modules & Imports

- Local file imports: `Import "file.poh"` executes the file once (cached), detects circular imports, and records exports.
- System imports (stdlib): `Import system "module"` loads a standard library module (e.g., `collections`).
- Imported modules run in a module scope; functions and variables can be injected into the importing module's scope and become available to use.

Example (system import):
```
Import system "collections"
Set xs to List contains 10, 20, 30
Write head(xs)    # 10
```

---

## Debugging & Tracing

- `Debug on` / `Debug off` statements toggle tracing.
- CLI flag `--debug` (Python) also enables rich tracing.
- Trace lines include file, line, column, and activity:
  - Executing statements
  - Evaluating expressions
  - Variable assignments
  - Function enter/return
  - Import notices

---

## Errors & Warnings

- All runtime and parse errors are reported with this format when positional info is available:
  `[file.poh: Line N: Col M] <Message>`

- Common runtime errors:
  - Undefined variable `x`.
  - Function arity mismatch.
  - Division by zero.
  - Type mismatch (numeric ops with strings).
  - Collection mutation on immutable values.
  - Index out of range / key not found.
  - Unknown function `name` (with suggestion if similar exists).

Examples:
- Unknown function:
```
[script.poh: Line 3: Col 1] Unknown function 'gree'. Did you mean 'greet'?
```

- Wrong number of arguments (referencing the definition site):
```
[script.poh: Line 5: Col 2] Function 'sumTwo' defined at line 1 expects 2 arguments but got 1
```

- Warnings:
  - Legacy collection literal mutations emit deprecation warnings advising explicit `mutable` usage.

---

## CLI & Tooling

- Python interpreter: `Interpreter/` (with `cli.py` for running files; tests live under `tests_python/`).
- Dart transpiler: `transpiler/src/` (CLI in root `bin/pohlang.dart` and `transpiler/bin/pohlang.dart`).
- Generated Dart files import a shared runtime from `lib/runtime.dart`.

---

## Developer Reference (Architecture & Extending)

High-level components:
- Python interpreter:
  - `Interpreter/poh_ast.py` — AST node definitions.
  - `Interpreter/poh_parser.py` — single-pass, line-based parser with expression parsing.
  - `Interpreter/poh_interpreter.py` — interpreter with scoping, functions, control flow, collections, I/O, imports, tracing.

- Dart transpiler core:
  - `src/ast.dart`, `src/parser.dart`, `src/transpiler.dart` (or `transpiler/src/*` based on recent restructuring).
  - Emits a single Dart program with function definitions above `main()`.
  - Runtime imported via `package:pohlang/runtime.dart` or a computed relative fallback.

Extending the language:
1. Add a node (statement/expression) in AST.
2. Update parser to recognize the new syntax (statement forms in `_parse_stmt_or_block` / `_parse_single_stmt` and/or expression forms in `_parse_expr`).
3. Implement evaluation/transpilation in the interpreter and/or transpiler.
4. Add runtime helpers if needed (in Dart: `lib/runtime.dart`).
5. Add tests (unit and E2E where needed).

Testing strategy:
- Python: unit + E2E tests in `tests_python/`.
- Dart: unit tests in `tests_dart/` (parser/control flow/arithmetic/IO/strict warnings, etc.).

---

## Migration Notes (Legacy forms)

- The legacy expressions `List contains ...` and `Dictionary contains ...` are temporarily supported.
- Mutating these legacy values prints a deprecation Warning recommending explicit `mutable` forms.
- Prefer using `Make a mutable list of ...` and `Make a mutable dictionary with ...` going forward.

---

This guide will evolve alongside the language. If code and docs disagree, treat the interpreter and transpiler source directories as the source of truth.