# PohLang Language Guide (September 2025)

This is a step‑by‑step user manual for PohLang. It teaches absolute beginners and experienced developers how to write real programs with the current language and runtime. The language is phrasal (English‑like), avoids brackets and symbols in user code, and favors clarity. Equality uses the word form “is” and the symbol “=”; no other symbols are required in everyday code.

Table of contents
- 1. First steps (Hello World → Variables → Expressions)
- 2. Conditions and loops
- 3. Collections (lists and dictionaries)
- 4. Functions (inline and block), defaults, closures
- 5. Modules and imports
- 6. Built‑ins you will use every day
- 7. Errors you might see and how to fix them
- 8. Command‑line usage
- 9. Notes for contributors (optional)

---

## 1. First steps

Write a line to the console:

```
Write "Hello, PohLang"
```

Create and use variables with Set:

```
Set name to "Ada"
Write "Hi " plus name
```

Numbers and basic math use words, not symbols:

```
Set a to 2
Set b to 3
Write a plus b          # 5
```

Booleans and None:

```
Set ok to True
Set empty to None
Write ok                 # True
Write empty              # None
```

Function calls are phrasal. Use “Use” for statements; use “with” to pass arguments. In expressions you can also call by name with “with”:

```
Make greet with who Write "Hello " plus who
Use greet with "World"

# As an expression call
Set msg to greet with "Reader"
Write msg
```

Equality uses “is” (or “=”). Inequality uses “is not”. Logical operators can be lowercase (and, or, not):

```
If name is "Ada" Write "Welcome"
If ok and not (name is "Bob") Write "Proceed"   # parentheses shown only for grouping in the comment; you do not need them in code
```

Tip: You do not need brackets in PohLang code. Prefer phrasal forms like “name with a, b” and “List contains …”.

---

## 2. Conditions and loops

If/Otherwise/End:

```
If name is "Ada"
  Write "Welcome back"
Otherwise
  Write "Hello"
End
```

While/End:

```
Set n to 3
While n is greater than 0
  Write n
  Set n to n minus 1
End
```

Repeat/End:

```
Repeat 3 times
  Write "Tick"
End
```

You can also repeat over a collection (see lists and dictionaries below).

---

## 3. Collections (lists and dictionaries)

Lists (legacy literal supported):

```
Set xs to List contains 1, 2, 3
Write xs
Write length(xs)        # 3
Write len(xs)           # 3 (alias)
```

Dictionaries (legacy literal supported):

```
Set d to Dictionary contains "a" set to 1, "b" set to 2
Write d
Write length(d)         # 2
```

Repeat over a list or dictionary:

```
Set xs to List contains "red", "green", "blue"
Repeat length(xs) times
  # add your own indexing or iteration helpers as you grow the stdlib
End
```

Simple membership and size checks:

```
If length(xs) is 3 Write "OK"
If length(d) is not 0 Write "Has items"
```

Note: The language aims to keep collections immutable by default. Mutation helpers may live in the standard library modules you Import (see below).

---

## 4. Functions (inline and block), defaults, closures

Inline function (single expression return):

```
Make greet with who set to "World" Write "Hello " plus who
Write greet with "Ada"
Write greet              # Calls with default → Hello World
```

Block function (use Return):

```
Make add with a, b
  Return a plus b
End

Write add with 2, 3
```

First‑class functions and closures:

```
Make makeAdder with x
  Make inner with y Write x plus y
  Return inner
End

Set add2 to makeAdder with 2
Write add2 with 3         # 5
```

Arity and defaults:

```
Make hello with name set to "World" Write "Hi " plus name
Write hello               # Hi World
Write hello with "Poh"    # Hi Poh
```

---

## 5. Modules and imports

Local files:

```
Import "utils.poh"
Use helper with "data"
```

System (standard library) modules:

```
Import system "collections"
```

The runtime looks for `Interpreter/stdlib/<module>.poh` (or a folder you set in the `POHLANG_STDLIB` environment variable). If a system module is not found, the import quietly does nothing so your program still runs.

---

## 6. Built‑ins you will use every day

- length(x) / len(x) — size of a string, list, or dictionary
- range(n) or range(start, end[, step]) — list of numbers
- join(list, sep) — text
- split(text, sep) — list of strings
- now() — current timestamp (seconds)

Examples:

```
Write length("abc")            # 3
Write join(List contains 1, 2, 3, ", ")
Write split("a,b,c", ",")
Write range(3)                  # legacy parentheses accepted in examples; prefer phrasal forms elsewhere
```

---

## 7. Errors you might see and how to fix them

If something goes wrong, the runtime prints a helpful message with a line number when it can. Common issues:

- Unknown function name: check spelling, or define it with Make.
- Wrong number of arguments: make sure calls match the function’s header (defaults help).
- Type mismatches: use numbers with numbers; text with text (plus concatenates text).
- Index or key problems: verify bounds and keys before access.

Example messages:

```
[script.poh: Line 3] Unknown function 'gree'. Did you mean 'greet'?
[script.poh: Line 5] Function 'sumTwo' expects 2..2 args but got 1
```

---

## 8. Command‑line usage

Two ways to run PohLang today:

- Python interpreter (legacy reference): under `Interpreter/`.
- Rust runtime (recommended for standalone): under `runtime-rs/` with the `pohlangc` binary.

Run a file with the Rust runtime:

```
pohlangc --run path\to\script.poh
```

Optionally set a standard library path:

```
POHLANG_STDLIB=path\to\stdlib pohlangc --run script.poh
```

---

## 9. Notes for contributors (optional)

- Parser and VM live in `runtime-rs/src/parser.rs` and `runtime-rs/src/vm.rs`.
- Tests live in `runtime-rs/tests`.
- The language avoids brackets and favors phrasal forms:
  - Prefer `name with a, b` over `name(a, b)` in examples and docs.
  - Prefer `List contains ...` and `Dictionary contains ...`.
- Equality can be written as `is` or `=`; inequality as `is not`.

This guide follows the language as implemented in this repository. If you see a mismatch, the code is the source of truth. Contributions welcome!