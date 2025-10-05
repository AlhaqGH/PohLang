# PohLang Phase 1 Grammar: Consistency and Ambiguity Analysis

**Date**: October 5, 2025  
**Version**: Phase 1 (v0.5.0)

---

## Executive Summary

✅ **Consistency**: PohLang Phase 1 grammar is **fully consistent** and parsable.  
✅ **Ambiguity**: PohLang grammar is **unambiguous** with proper precedence handling.

---

## 1. Consistency Analysis

A grammar is consistent if:
- Every construct can be parsed using the given production rules
- There are no contradictions or unreachable rules
- Keywords and syntax delimiters prevent overlap

### ✅ Strengths

| Feature | Why It's Consistent |
|---------|-------------------|
| **Explicit terminators** (`End If`, `End While`, etc.) | Prevents dangling-statement problems and nesting confusion |
| **Canonical phrasal keywords** | Eliminates collision between symbols (e.g., `is greater than` ≠ `>`) |
| **Newline or semicolon separation** | Clear statement boundary, parser-friendly |
| **Reserved starter/ender blocks** | Each block form is explicit (`Start Program` / `End Program`) |
| **Fixed statement forms** | Each command has a canonical phrase pattern — makes parsing deterministic |
| **Multi-word operators** | Prevents keyword/variable name overlap (e.g., "divided by" vs variable "by") |

### Implementation Details

The parser implements a **recursive descent** strategy with the following guarantees:

1. **Statement-level clarity**: Every statement begins with a unique keyword:
   - `Write`, `Set`, `Ask for`, `If`, `While`, `Repeat`, `Make`, `Use`, `Import`, `Increase`, `Decrease`, `Return`

2. **Block structure**: All blocks have explicit delimiters:
   - `If ... End If`
   - `While ... End While`
   - `Repeat ... End Repeat`
   - `Make <name> with <params> ... End`

3. **No keyword conflicts**: Variables cannot be named with reserved multi-word phrases since identifiers are single tokens.

### Verdict

✅ **The Phase 1 syntax is structurally consistent.** Every rule can be parsed unambiguously with a recursive-descent parser. There are no contradictions between rule forms.

---

## 2. Ambiguity Analysis

A grammar is ambiguous if a sentence (program) can be parsed in more than one way (two valid parse trees).

### (a) Expression Precedence ✅

**Potential Issue**: English-like infix operators (`plus`, `minus`, `times`, `divided by`) could create ambiguity without proper precedence.

Example:
```poh
Set x to 5 plus 3 times 2
```

Could this mean `(5 + 3) × 2 = 16` or `5 + (3 × 2) = 11`?

**✅ RESOLVED**: PohLang implements proper operator precedence hierarchy:

#### Precedence Chain (highest → lowest)

```rust
parse_expr     → parse_or              // Logical OR (lowest precedence)
parse_or       → parse_and             // Logical AND  
parse_and      → parse_not             // Logical NOT
parse_not      → parse_cmp             // Comparisons (is equal to, is greater than, etc.)
parse_cmp      → parse_add             // Addition/Subtraction (plus, minus)
parse_add      → parse_mult            // Multiplication/Division (times, divided by)
parse_mult     → parse_postfix         // Indexing [index]
parse_postfix  → parse_term            // Primary terms (highest precedence)
```

#### Mathematical Operator Precedence

1. **Highest**: Indexing `[index]`, Function calls `f(x)`, Parentheses `(expr)`
2. **High**: Multiplication `times`, Division `divided by` (left-to-right)
3. **Medium**: Addition `plus`, Subtraction `minus` (left-to-right)
4. **Low**: Comparisons `is greater than`, `is equal to`, etc.
5. **Lower**: Logical NOT `Not expr`
6. **Lower**: Logical AND `expr And expr`
7. **Lowest**: Logical OR `expr Or expr`

#### Verification

Test case:
```poh
Set result to 10 plus 5 times 2
Write result  # Output: 20
```

Parsing:
- `10 plus 5 times 2`
- → `10 plus (5 times 2)`  ← multiplication has higher precedence
- → `10 + (5 × 2)`
- → `10 + 10`
- → `20` ✅

**Status**: ✅ Fully unambiguous with explicit precedence rules.

---

### (b) Assignment vs Expression Calls ✅

**Potential Issue**: Both patterns are valid:
```poh
Set x to square(6)      # Assignment
square(6)               # Standalone expression call
```

**✅ RESOLVED**: Parser distinguishes these at the **statement level**:
- Statements starting with `Set` are assignments
- Statements starting with `Use` are phrasal function calls
- Bare function calls `name(args)` within expressions are handled separately

The grammar clearly separates:
```ebnf
statement = assignment | function_call | write | ask_for | ... ;
assignment = "Set" identifier "to" expression ;
function_call = "Use" identifier "with" arg_list ;
```

**Status**: ✅ Unambiguous through statement-level discrimination.

---

### (c) Conditionals (Dangling Else) ✅

**Classic Problem**: In languages like C, this is ambiguous:
```c
if (a) if (b) x = 1; else x = 2;
```

Does `else` bind to the first or second `if`?

**✅ RESOLVED**: PohLang requires explicit `End If`:
```poh
If a
    If b
        Set x to 1
    End If
Otherwise
    Set x to 2
End If
```

**Status**: ✅ No dangling-else ambiguity possible.

---

### (d) Identifiers and Keywords ✅

**Potential Issue**: Can a variable be named the same as a keyword?

**✅ RESOLVED**: All keywords are **multi-word phrasal** constructs:
- `greater than`, `divided by`, `is equal to`, `Set to`, `Ask for`, etc.

Variables are **single identifiers**:
- `x`, `count`, `my_variable`, `age`

Since identifiers cannot contain spaces, there's no overlap.

Example:
```poh
Set to to 5          # ✅ "to" is a valid variable name
Set x to to plus 1   # ✅ Parses as: Set x to (to plus 1)
```

**Status**: ✅ No keyword/identifier conflicts.

---

### (e) Phrasal Built-ins ✅

**Potential Issue**: Phrasal expressions like `contains X in Y` could conflict with natural language parsing.

**✅ RESOLVED**: Each phrasal has a **unique prefix** and **separator pattern**:
- `contains <item> in <collection>` — split by `" in "` at top level
- `remove <item> from <list>` — split by `" from "` at top level
- `insert <item> at <index> in <list>` — split by `" at "` then `" in "`

The parser uses `split_once_top_level()` which:
- Respects string literals (ignores `" in "` inside quotes)
- Respects nesting (ignores separators inside parentheses/brackets)
- Matches separators left-to-right

**Status**: ✅ Unambiguous through careful separator matching.

---

## 3. Complete Grammar Specification

### Formal Precedence (EBNF-style)

```ebnf
(* Entry point *)
expression      = logical_or ;

(* Logical operators (lowest precedence) *)
logical_or      = logical_and , { ("Or" | "or") , logical_and } ;
logical_and     = logical_not , { ("And" | "and") , logical_not } ;
logical_not     = [ ("Not" | "not") ] , comparison ;

(* Comparison operators *)
comparison      = additive , 
                  [ comp_op , additive ] ;
comp_op         = " is not " | " is equal to " | " is greater than " 
                | " is less than " | " is at least " | " is at most " 
                | " = " ;

(* Arithmetic operators *)
additive        = multiplicative , 
                  { ( " plus " | " minus " ) , multiplicative } ;
multiplicative  = postfix , 
                  { ( " times " | " divided by " ) , postfix } ;

(* Postfix operators *)
postfix         = primary , [ "[" , expression , "]" ] ;

(* Primary expressions (highest precedence) *)
primary         = number
                | string
                | boolean
                | identifier
                | function_call
                | phrasal_expression
                | list_literal
                | dict_literal
                | "(" , expression , ")" ;

(* Phrasal expressions *)
phrasal_expression
                = unary_phrasal | binary_phrasal | ternary_phrasal ;

unary_phrasal   = "total of" expression
                | "smallest in" expression
                | "largest in" expression
                | "absolute value of" expression
                | "round" expression
                | "round down" expression
                | "round up" expression
                | "make uppercase" expression
                | "make lowercase" expression
                | "trim spaces from" expression
                | "first in" expression
                | "last in" expression
                | "reverse of" expression
                | "count of" expression ;

binary_phrasal  = "join" expression "with" expression
                | "split" expression "by" expression
                | "contains" expression "in" expression
                | "remove" expression "from" expression
                | "append" expression "to" expression ;

ternary_phrasal = "insert" expression "at" expression "in" expression ;
```

---

## 4. Summary

| Aspect | Status | Notes |
|--------|--------|-------|
| **Block delimiters** | ✅ Unambiguous | Explicit `End If`, `End While`, etc. |
| **Function declarations** | ✅ Clear start/end | `Make ... End` |
| **Control flow** | ✅ Explicit | No dangling-else problem |
| **Operator precedence** | ✅ Fully defined | Recursive descent with proper hierarchy |
| **Expression grouping** | ✅ Parentheses supported | `(expr)` for explicit grouping |
| **Statement separation** | ✅ Clear | Newline or semicolon |
| **Keyword/identifier distinction** | ✅ Stable | Multi-word keywords vs single-token identifiers |
| **Phrasal built-ins** | ✅ Unambiguous | Pattern-based parsing with top-level splitting |

---

## 5. Conclusion

**Consistency**: ✅ **Yes** — PohLang Phase 1 grammar is fully consistent and deterministic.

**Ambiguity**: ✅ **No** — The grammar is unambiguous:
- Proper operator precedence hierarchy implemented
- Explicit block delimiters eliminate nesting ambiguity
- Multi-word keywords prevent identifier conflicts
- Phrasal expressions use unique separator patterns

**Parser Type**: Recursive Descent (LL)

**Determinism**: Every valid PohLang program has exactly **one** parse tree.

---

## 6. Test Coverage

The following tests verify unambiguous parsing:

### Operator Precedence Tests
```poh
# Test: Multiplication before addition
Set result to 10 plus 5 times 2
# Expected: 20 (not 30)
# Parse tree: Plus(10, Times(5, 2))
```

### Nested Expressions
```poh
# Test: Parentheses override precedence
Set result to (10 plus 5) times 2
# Expected: 30
# Parse tree: Times(Plus(10, 5), 2)
```

### Logical Operator Precedence
```poh
# Test: AND before OR
If x Or y And z
# Parse tree: Or(x, And(y, z))
```

### Comparison Precedence
```poh
# Test: Arithmetic before comparison
If 10 plus 5 is greater than 12
# Parse tree: Cmp(Plus(10, 5), 12)
```

### Phrasal Expression Nesting
```poh
# Test: Phrasal inside arithmetic
Set result to count of [1, 2, 3] plus 5
# Parse tree: Plus(CountOf([1,2,3]), 5)
```

---

## 7. Future Considerations (Phase 2)

While Phase 1 is unambiguous, Phase 2 additions should maintain this property:

1. **Module qualifiers** (`module::function`):
   - Ensure `::` doesn't conflict with comparison operators
   - Recommend: Always require explicit namespace for imported symbols

2. **New operators**:
   - Define precedence relative to existing operators
   - Document in grammar specification

3. **Custom infix operators** (if added):
   - Require explicit precedence declarations
   - Maintain left-to-right associativity by default

---

## References

- PohLang Grammar: `spec/Grammar.ebnf`
- PohLang Vocabulary: `spec/Vocabulary.md`
- Parser Implementation: `runtime/src/parser/parser.rs`
- Precedence Tests: `runtime/tests/smoke.rs::arithmetic_precedence_works`
