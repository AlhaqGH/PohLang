# PohLang Vocabulary Roadmap (Phases 1 – 3)

This document is the source of truth for the phrases the PohLang parser accepts.
It mirrors the grammar file and highlights how the lexicon grows in Phases 2 and
3 so that docs, tooling, and runtime remain aligned.

| Phase | Status | Headline |
| ----- | ------ | -------- |
| 1     | ✅ shipping | Core statements, modern collections, `Make`/`Use` functions |
| 2     | 🛠 planned | Standard-library modules with aliased system imports |
| 3     | 🛠 planned | Bytecode compilation directives and artifacts |

---

## Phase 1 — Canonical Vocabulary (Current Runtime)

### Core Statements

| Intent        | Phrase | Notes |
| ------------- | ------ | ----- |
| Output        | `Write <expression>` | Prints any expression. |
| Input         | `Ask for <name> [expect <Type>] [with "Prompt"]` | Types: `Number`, `String`. |
| Assignment    | `Set <name> to <expression>` | Introduces or mutates a variable. |
| Increment     | `Increase <name> [by <expression>]` | Default `by 1` if omitted. |
| Decrement     | `Decrease <name> [by <expression>]` | Default `by 1` if omitted. |
| Import file   | `Import "path/to/file.poh"` | Path relative to caller. |
| Import module | `Import system "module"` | Loads a built-in module (Phase 1 exposes a stub). |

### Control Flow

| Construct | Phrase | Example |
| --------- | ------ | ------- |
| Conditional | `If <condition> ... Otherwise ... End If` | `If age is at least 18 ...` |
| While loop | `While <condition> ... End While` | `While count is greater than 0 ...` |
| Repeat loop | `Repeat <expression> times ... End Repeat` | Expression evaluated at runtime. |

### Functions & Calls

| Form | Phrase | Notes |
| ---- | ------ | ----- |
| Inline function | `Make <name> with <params> Write <expression>` | Returns last expression implicitly. |
| Block function  | `Make <name> with <params> ... Return <expression> ... End` | Allows multi-line bodies. |
| Parameters      | `<param> [set to <default>]` | Defaults optional in both inline and block forms. |
| Invoke in stmt  | `Use <name> with arg [and arg...]` | Sugar for positional calls with natural linking. |
| Invoke in expr  | `<name>(arg, ...)` | Traditional call usable inside expressions. |
| Return          | `Return <expression>` | Valid only inside function bodies. |

### Operators & Expressions

| Category | Vocabulary |
| -------- | ---------- |
| Arithmetic (Phrasal) | `plus`, `minus`, `times`, `divided by` |
| Arithmetic (Symbolic) | `+`, `-`, `*`, `/` |
| Comparisons (Phrasal) | `is equal to`, `is not equal to`, `is greater than`, `is less than`, `is at least`, `is at most` |
| Comparisons (Symbolic) | `==`, `!=`, `>`, `<`, `>=`, `<=` |
| Logic | `And`, `Or`, `Not` (case-insensitive) |
| Collections | `[expr, ...]`, `{key: value, ...}` |
| Indexing | `<expr>[index]` with optional negatives |

**Note:** Both phrasal and symbolic forms are fully supported and can be mixed in the same program.

### Built-in Functions

| Name | Purpose | Example |
| ---- | ------- | ------- |
| `range(limit)` | Produces `[0, 1, …, limit-1]`. | `Set nums to range(5)` |
| `join(list, sep)` | Joins list values into a single string. | `Write join(nums, ",")` |
| `split(text, sep)` | Splits text into a list of strings. | `Set parts to split("a,b,c", ",")` |
| `length(x)` / `len(x)` | Size of a string, list, or dictionary. | `Write length("hello")` |
| `now()` | Current timestamp (seconds). | `Set time to now()` |

### Phrasal Built-in Expressions (Phase 1)

PohLang provides phrasal expressions for common operations that read like natural English:

Notes:
- Phrases are case-insensitive: `Total of`, `total of`, and `TOTAL OF` are all valid.
- Some friendly aliases are accepted for readability:
    - `reverse X` is the same as `reverse of X`
    - `clean spaces from S` is the same as `trim spaces from S`

#### Mathematical Operations

| Phrase | Purpose | Example |
| ------ | ------- | ------- |
| `total of <list>` | Adds all numbers in a list. | `Set sum to total of numbers` |
| `smallest in <list>` | Finds the minimum value in a list. | `Write smallest in values` |
| `largest in <list>` | Finds the maximum value in a list. | `Write largest in scores` |
| `absolute value of <number>` | Returns the absolute value. | `Set distance to absolute value of -42` |
| `round <number>` | Rounds to nearest integer. | `Set rounded to round 3.7` |
| `round down <number>` | Rounds down (floor). | `Set floored to round down 3.9` |
| `round up <number>` | Rounds up (ceiling). | `Set ceiled to round up 3.1` |

#### String Operations

| Phrase | Purpose | Example |
| ------ | ------- | ------- |
| `make uppercase <string>` | Converts to uppercase. | `Set upper to make uppercase "hello"` |
| `make lowercase <string>` | Converts to lowercase. | `Set lower to make lowercase "HELLO"` |
| `trim spaces from <string>` | Removes leading/trailing whitespace. | `Set clean to trim spaces from messy` |

#### Collection Operations

| Phrase | Purpose | Example |
| ------ | ------- | ------- |
| `first in <collection>` | Gets the first element of a list or string. | `Set head to first in numbers` |
| `last in <collection>` | Gets the last element of a list or string. | `Set tail to last in numbers` |
| `reverse of <collection>` | Reverses a list or string. | `Set backwards to reverse of word` |
| `count of <x>` | Size of a list, string, or dictionary. | `Write count of names` |
| `join <list> with <sep>` | Join items into text with a separator. | `Write join names with ", "` |
| `split <text> by <sep>` | Split text into a list by a separator. | `Set parts to split email by "@"` |
| `contains <item> in <collection>` | Checks if item exists in list, string, or dict. | `Write contains 3 in numbers` |
| `remove <item> from <list>` | Removes first occurrence of item from list. | `Set cleaned to remove 0 from data` |
| `append <item> to <list>` | Adds item to the end of a list. | `Set extended to append 5 to nums` |
| `insert <item> at <index> in <list>` | Inserts item at specific position in list. | `Set result to insert 'x' at 2 in letters` |

Aliases:
- `size of <x>` → `count of <x>`
- `separate <text> by <sep>` → `split <text> by <sep>`

Aliases:
- `reverse <collection>` → `reverse of <collection>`
- `clean spaces from <string>` → `trim spaces from <string>`

### Phrasal vs Function Call Syntax

PohLang supports both phrasal expressions and traditional function calls:

```
# Phrasal (preferred for Phase 1)
Set total to total of numbers
Set upper to make uppercase "hello"

# Function call (legacy, still supported)
Set total to sum(numbers)
Set upper to uppercase("hello")
```

The phrasal form is more readable and aligns with PohLang's mission of being beginner-friendly.

Phase 2 will grow this list, but these two helpers are already stable in Phase 1
and used throughout the smoke tests.

### Start/End Wrapper

Every script **must** begin with `Start Program` and end with `End Program`. The
parser now emits a hard error if either sentinel is missing or duplicated.

### Example Program

```
Start Program
Set guests to ["Ada", "Grace", "Hadi"]
Make greet with name Write "Hello " plus name
Write greet(guests[0])
Write greet(guests[1])
Write greet(guests[2])
End Program
```

---

## Phase 2 — Vocabulary Additions (Standard Library)

Phase 2 expands the import vocabulary and establishes module terminology. The
goal is to make system imports explicit and ergonomic without introducing new
core statements.

### New Import Forms

| Phrase | Meaning |
| ------ | ------- |
| `Import system "collections" as coll` | Binds the module to the alias `coll`. |
| `Import system "random" exposing shuffle, choice` | Brings only the listed symbols into the surrounding scope. |

These forms are additive; the original `Import system "module"` remains valid.

### Qualified Calls

| Form | Example | Notes |
| ---- | ------- | ----- |
| Module scope | `collections::map(list, fn)` | Double colon keeps the symbol namespaced. |
| Exposed symbol | `shuffle(list)` | Works only if the module exposed the symbol or if the runtime preloads it. |

### Module Cheat Sheet

| Module | Signature Highlights |
| ------ | -------------------- |
| `collections` | `head(list)`, `tail(list)`, `map(list, fn)`, `filter(list, fn)`, `reduce(list, fn, seed)` |
| `random` | `rand_int(min, max)`, `rand_float()`, `choice(list)` |
| `datetime` | `now_iso()`, `today_hijri()`, `format(dt, pattern)` |
| `math` | `abs(x)`, `sqrt(x)`, `pow(base, exp)`, trig helpers |
| `file` | `read_text(path)`, `write_text(path, data)` |
| `process` | `run(cmd, args)`, `exit(code)` |
| `islamic` | `prayer_times(city)`, `qibla(location)` |

_All module names are lower case; aliases follow standard identifier rules._

### Diagnostic Expectations

- Referencing a module symbol without importing it should produce: `Unknown
    symbol 'shuffle'. Did you mean collections::shuffle?`
- Duplicate aliases are rejected with a hint to rename one of the imports.

---

## Phase 3 — Vocabulary Additions (Bytecode Toolchain)

Phase 3 introduces compile-time directives. These phrases are reserved starting
now to avoid future breaking changes.

| Phrase | Purpose |
| ------ | ------- |
| `Compile to "file.pbc"` | Request bytecode emission for the current script. |
| `Compile to "file.pbc" with debug, optimize` | Provide compile options (comma-separated). |
| `Emit bytecode to "file.pbc"` | Write the current bytecode buffer to disk. |
| `Load bytecode "lib.pbc"` | Load and execute precompiled bytecode. |

Guidelines:

- Directives must appear at top level (same indentation as other statements).
- Options currently planned: `debug`, `optimize`, `entry <FunctionName>`.
- When the runtime lacks bytecode support it should surface: `Bytecode feature
    not available. Build with --features bytecode` (wording TBD).

---

## Change Log

- **2025‑10‑05**: Converted document into a phased roadmap, aligned tables with
    the Rust runtime, and documented the planned Phase 2/3 vocabulary.
- Legacy references retained in `spec/old-vocabulary.md` for posterity.

# PohLang Phase 1 Vocabulary (Canonical, Updated)

This document captures the **official Phase 1 vocabulary** for PohLang. It is the authoritative reference for learners and developers.

---

## Core Statements

| Intent     | Syntax                           | Notes                                            |
| ---------- | -------------------------------- | ------------------------------------------------ |
| Output     | `Write <expression>`             | Prints a value or expression.                    |
| Input      | `Ask for <var>`                  | Reads input from the user and stores in `<var>`. |
| Assignment | `Set <var> to <expression>`      | Assigns a value.                                 |
| Increase   | `Increase <var> by <expression>` | Increments a variable. Default +1 if omitted.    |
| Decrease   | `Decrease <var> by <expression>` | Decrements a variable. Default -1 if omitted.    |

### Control Flow

| Statement                 | Syntax                                    | Notes                              |
| ------------------------- | ----------------------------------------- | ---------------------------------- |
| If Block                  | `If <condition> ... End If`               | Multi-line block.                  |
| If Block with alternative | `If <condition> ... Otherwise ... End If` | Use `Otherwise` instead of `Else`. |
| While Loop                | `While <condition> ... End While`         | Loop while condition true.         |
| Repeat Loop               | `Repeat <n> times ... End Repeat`         | Loop N times.                      |

### Functions

| Statement      | Syntax                                                           | Notes               |
| -------------- | ---------------------------------------------------------------- | ------------------- |
| Block Function | `Function <name>(<params>) ... Return <expression> End Function` | Defines a function. |
| Call Function  | `<name>(args...)`                                                | Invokes a function. |

### Program Structure

| Statement | Syntax                          |
| --------- | ------------------------------- |
| Program   | `Start Program ... End Program` |

### Operators

| Operator       | Phrasal Form | Symbolic Form | Notes                      |
| -------------- | ------------ | ------------- | -------------------------- |
| Addition       | `plus`       | `+`           | `Set x to 3 plus 2` or `Set x to 3 + 2` |
| Subtraction    | `minus`      | `-`           | `Set x to 5 minus 2` or `Set x to 5 - 2` |
| Multiplication | `times`      | `*`           | `Set x to 4 times 2` or `Set x to 4 * 2` |
| Division       | `divided by` | `/`           | `Set x to 10 divided by 2` or `Set x to 10 / 2` |

**Note:** Both phrasal and symbolic forms work identically. Mix them freely!

### Comparisons

| Comparison            | Phrasal Form      | Symbolic Form | Notes |
| --------------------- | ----------------- | ------------- | ----- |
| Equal                 | `is equal to`     | `==`          | Both forms: `If x is equal to 5` or `If x == 5` |
| Not equal             | `is not equal to` | `!=`          | Both forms: `If x is not equal to 5` or `If x != 5` |
| Less than             | `is less than`    | `<`           | Both forms: `If age is less than 18` or `If age < 18` |
| Greater than          | `is greater than` | `>`           | Both forms: `If score is greater than 90` or `If score > 90` |
| Less than or equal    | `is at most`      | `<=`          | Both forms: `If x is at most 10` or `If x <= 10` |
| Greater than or equal | `is at least`     | `>=`          | Both forms: `If y is at least 5` or `If y >= 5` |

**Note:** Symbolic operators like `>`, `<`, `>=`, `<=`, `==`, `!=` are fully supported alongside phrasal forms.

### Logical Connectors

| Connector | Phrase |
| --------- | ------ |
| And       | `And`  |
| Or        | `Or`   |
| Not       | `Not`  |

### Example Program

```
Start Program
Write "Enter your name:"
Ask for name
If name is equal to "Alice"
    Write "Hello Alice!"
Otherwise
    Write "Hello Stranger!"
End If
Set count to 1
Repeat 5 times
    Write "Loop number " plus count
    Increase count by 1
End Repeat
Function square(x)
    Return x times x
End Function
Write "Square of 4 is " plus square(4)
End Program
```