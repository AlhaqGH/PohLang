# PohLang Language Guide (September 2025)

This is a step‑by‑step user mEquality uses "is" (or "="). Tip: You do not need brackets in PohLang code. Prefer phrasal forms like "name with a, b" and "List contains …".

**Operator Flexibility:** PohLang supports BOTH phrasal and symbolic operators:
- **Phrasal forms** (`plus`, `is greater than`) are great for beginners and readability
- **Symbolic forms** (`+`, `>`) are familiar to experienced programmers
- **Mix both styles** freely in the same program - use what feels natural!

---

## 2. Conditions and loopsality uses "is not". Logical operators can be lowercase (and, or, not):

Comparisons - phrasal forms OR symbolic operators:

```
# Phrasal forms (beginner-friendly)
If name is "Ada" Write "Welcome"
If age is greater than 18 Write "Adult"
If score is less than 60 Write "Failed"
If x is greater than or equal to 10 Write "Big"
If y is not equal to 0 Write "Non-zero"

# Symbolic forms (familiar to programmers)
If age > 18 Write "Adult"
If score < 60 Write "Failed"
If x >= 10 Write "Big"
If y != 0 Write "Non-zero"
If count == 5 Write "Five"

# Mixed styles work too!
If ok and not (name is "Bob") Write "Proceed"
If (a + b) > 10 Write "Large sum"
```for PohLang. It teaches absolute beginners and experienced developers how to write real programs with the current language and runtime. The language is phrasal (English‑like), avoids brackets and symbols in user code, and favors clarity. Equality uses the word form “is” and the symbol “=”; no other symbols are required in everyday code.

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

Numbers and basic math - you can use phrasal forms OR symbolic operators:

```
Set a to 2
Set b to 3

# Phrasal form (beginner-friendly)
Write a plus b           # 5
Write a minus b          # -1
Write a times b          # 6
Write a divided by b     # 0.666...

# Symbolic form (familiar to programmers)
Write a + b              # 5
Write a - b              # -1
Write a * b              # 6
Write a / b              # 0.666...

# You can mix both styles!
Set result to (a + b) times 2
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

The runtime looks for standard library modules in its stdlib directory. If a system module is not found, the import quietly does nothing so your program still runs. Standard library development is ongoing—see ROADMAP.md for details.

---

## 6. Built‑ins you will use every day

### Traditional Function Calls (Legacy)

- length(x) / len(x) — size of a string, list, or dictionary
- range(n) or range(start, end[, step]) — list of numbers
- join(list, sep) — text
- split(text, sep) — list of strings
- now() — current timestamp (seconds)

### Phrasal Expressions (Preferred)

PohLang provides natural, English-like phrases for common operations. These are more readable and beginner-friendly:

#### Mathematical Operations

```
Set numbers to Make a list of 1, 2, 3, 4, 5

Set sum to total of numbers              # Adds all numbers
Set minimum to smallest in numbers        # Finds minimum
Set maximum to largest in numbers         # Finds maximum

Set distance to absolute value of -42     # Absolute value: 42
Set rounded to round 3.7                  # Round to nearest: 4
Set floored to round down 3.9             # Round down: 3
Set ceiled to round up 3.1                # Round up: 4
```

#### String Operations

```
Set greeting to "hello world"
Set loud to "STOP SHOUTING"
Set messy to "  spaces  "

Set upper to make uppercase greeting      # "HELLO WORLD"
Set lower to make lowercase loud          # "stop shouting"
Set clean to trim spaces from messy       # "spaces"
```

#### Collection Operations

```
Set numbers to Make a list of 1, 2, 3, 4, 5
Set word to "PohLang"

Set first_num to first in numbers         # 1
Set last_num to last in numbers           # 5
Set reversed to reverse of numbers        # [5, 4, 3, 2, 1]

Set first_char to first in word           # "P"
Set last_char to last in word             # "g"
Set backwards to reverse of word          # "gnaLhoP"
```

### Why Phrasal?

Phrasal expressions make code more readable:

```
# Traditional (harder to read)
Write sum(filter(map(numbers, double), is_positive))

# Phrasal (easier to understand)
Set doubled to map numbers with double
Set positive to filter doubled with is_positive
Set result to total of positive
Write result
```

Examples:

```
Write length("abc")                         # 3
Write join(Make a list of 1, 2, 3, ", ")   # "1, 2, 3"
Write split("a,b,c", ",")                  # ["a", "b", "c"]
Write total of Make a list of 10, 20, 30  # 60
Write count of Make a list of 1, 2 and 3 # 3
Write join Make a list of "a", "b", "c" with "-"   # "a-b-c"
Write split "a,b,c" by ","               # ["a", "b", "c"]

# Collection operations
Write contains 3 in [1, 2, 3]           # True
Write contains "world" in "hello world" # True
Set cleaned to remove 0 from [1, 0, 2]  # [1, 2]
Set extended to append 4 to [1, 2, 3]   # [1, 2, 3, 4]
Set result to insert 2.5 at 2 in [1, 2, 3]  # [1, 2, 2.5, 3]

# Aliases
Write size of Make a list of 1 and 2   # 2 (alias of count of)
Write separate "a,b,c" by ","          # ["a", "b", "c"] (alias of split)
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

### Building the Runtime

```bash
# Build the Rust runtime
cargo build --manifest-path runtime/Cargo.toml

# For optimized builds
cargo build --release --manifest-path runtime/Cargo.toml
```

### Running Programs

Run a file with the Rust runtime:

```bash
# Using cargo (development)
cargo run --manifest-path runtime/Cargo.toml -- --run path/to/script.poh

# Using the compiled binary
./target/debug/pohlang --run path/to/script.poh

# Using release build (faster)
./target/release/pohlang --run path/to/script.poh
```

### Using PLHub

For a complete development environment, use [PLHub](https://github.com/AlhaqGH/PLHub):

```bash
plhub run path/to/script.poh
```

Optionally set a standard library path:

```bash
POHLANG_STDLIB=path/to/stdlib cargo run --manifest-path runtime/Cargo.toml -- --run script.poh
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