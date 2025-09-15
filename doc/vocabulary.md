# PohLang Vocabulary (Phrasal Only v0.1)

PohLang uses natural, English-like commands. Each line is a statement. No symbol mode; the entire language is phrasal.

## Core Statements

| Intent | Syntax | Notes |
|--------|--------|-------|
| Output text/value | `Write <expression>` | Prints the evaluated expression (legacy: `Say`) |
| Input (string) | `Ask for <name>` | Prompts user; stores input in `<name>` |
| Assignment | `Set <name> to <expression>` | Also accepts `Set <name> <expression>` |
| Increase | `Increase <name> by <number>` | Makes the number bigger (default 1 if omitted) |
| Decrease | `Decrease <name> by <number>` | Makes the number smaller (default 1 if omitted) |
| Conditional (inline) | `If <condition> Write <expr> Otherwise Write <expr>` | Single-line conditional (legacy: `Say`) |
| Loop (inline) | `Repeat <count> times Write <expr>` | Also accepts `Repeat <count> Write <expr>` |
| While loop (inline) | `While <condition> Write <expr>` | Single-line while form |
| Make function (inline) | `Make <name> with <params> Write <expr>` | Creates a function that returns the expression |
| Use function | `Use <name> with <args>` | Runs a function |

## Expressions
Numbers (e.g., `5`, `12`), strings (`"Hello"`), booleans (`true`, `false`), identifiers (`age`, `name`), addition / subtraction (`a + b`, `a - 1`), multiplication (`a * b`).

## Conditions
Boolean connectors and phrase-based comparators:
- Logical: `And`, `Or`, `Not`
- Comparisons: `Equals`, `Not Equals`, `Greater Than`, `Less Than`, `Greater Or Equal`, `Less Or Equal`
- Back-compat verbal forms still work: `is equal to`, `is not equal to`, `is greater than`, `is less than`
- Symbolic `=` inside conditions is treated as equality.

Examples:
```
If age Greater Than 18 And hasID Equals true Write "You may enter"
If age Less Or Equal 12 Or isStudent Equals true Write "Discount"
If Not hasPaid Write "Payment required"
```

## Sugars (Desugaring)
| Sugar | Expansion |
|-------|-----------|
| `Increase x by 2` | `Set x x + 2` |
| `Decrease x by 1` | `Set x x - 1` |
| `Repeat 5 Write "Hi"` | Expand to internal counted loop |

## Planned Extensions
- Multi-line blocks (indentation): `If <cond>` then indented statements and an optional `Otherwise` section.
- Lists & dictionaries: `List numbers contains 1, 2, 3`.
- Predicates: `If age is greater than 18 ...` (verbal comparisons).
- Random numbers: `Set n random between 1 10` (syntax under consideration).
- Multi-line function bodies.

## Design Principles
1. One way per concept (no synonyms like Print/Say/Show).
2. Natural ordering: verb first (`Set`, `Write`, `Repeat`).
3. Punctuation minimized; quotes only for strings.
4. Friendly for reading aloud.

## Example Program
```
Ask for name
Write "Hello " + name
Set count 3
Repeat count Write "Hi"
If count > 1 Write "Many" Otherwise Write "Few"
Make greet with who Write "Hi " + who
Use greet with "Poh"
```

## Error Examples (Planned)
- `Sya "Hi"` → Did you mean `Write`?
- `Set x` → Missing value after variable name.

---
This vocabulary will expand cautiously to avoid overwhelming beginners.
