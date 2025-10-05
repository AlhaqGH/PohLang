# Phrasal Built-ins Implementation Summary

**Date**: October 5, 2025  
**Version**: PohLang v0.5.0  
**Status**: ✅ Complete

## Overview

Successfully implemented **13 phrasal built-in expressions** that align with PohLang's mission of being beginner-friendly and using natural English-like syntax instead of traditional programming terminology.

## Problem Statement

The initial implementation added built-in functions using traditional programming terms:
- `sum()`, `min()`, `max()`, `abs()`, `round()`, `floor()`, `ceil()`
- `uppercase()`, `lowercase()`, `trim()`
- `first()`, `last()`, `reverse()`

**Issue**: These violated PohLang's core mission of being phrasal and beginner-friendly.

## Solution

Implemented **phrasal expressions** that read like natural English:

### Mathematical Operations (7 phrasal expressions)
| Traditional | Phrasal (Implemented) | Example |
|------------|----------------------|---------|
| `sum(list)` | `total of list` | `Set sum to total of numbers` |
| `min(list)` | `smallest in list` | `Write smallest in values` |
| `max(list)` | `largest in list` | `Write largest in scores` |
| `abs(n)` | `absolute value of n` | `Set dist to absolute value of -42` |
| `round(n)` | `round n` | `Set rounded to round 3.7` |
| `floor(n)` | `round down n` | `Set floored to round down 3.9` |
| `ceil(n)` | `round up n` | `Set ceiled to round up 3.1` |

### String Operations (3 phrasal expressions)
| Traditional | Phrasal (Implemented) | Example |
|------------|----------------------|---------|
| `uppercase(s)` | `make uppercase s` | `Set upper to make uppercase greeting` |
| `lowercase(s)` | `make lowercase s` | `Set lower to make lowercase loud` |
| `trim(s)` | `trim spaces from s` | `Set clean to trim spaces from messy` |

### Collection Operations (3 phrasal expressions)
| Traditional | Phrasal (Implemented) | Example |
|------------|----------------------|---------|
| `first(list)` | `first in list` | `Set head to first in numbers` |
| `last(list)` | `last in list` | `Set tail to last in numbers` |
| `reverse(list)` | `reverse of list` | `Set rev to reverse of word` |

## Implementation Details

### 1. AST Extensions (`runtime/src/parser/ast.rs`)

Added 13 new expression variants to the `Expr` enum:

```rust
pub enum Expr {
    // ... existing variants ...
    
    // Phrasal built-in expressions
    TotalOf(Box<Expr>),           // total of list
    SmallestIn(Box<Expr>),        // smallest in list
    LargestIn(Box<Expr>),         // largest in list
    AbsoluteValueOf(Box<Expr>),   // absolute value of number
    Round(Box<Expr>),             // round number
    RoundDown(Box<Expr>),         // round down number
    RoundUp(Box<Expr>),           // round up number
    MakeUppercase(Box<Expr>),     // make uppercase string
    MakeLowercase(Box<Expr>),     // make lowercase string
    TrimSpaces(Box<Expr>),        // trim spaces from string
    FirstIn(Box<Expr>),           // first in list/string
    LastIn(Box<Expr>),            // last in list/string
    ReverseOf(Box<Expr>),         // reverse of list/string
}
```

### 2. Parser Updates (`runtime/src/parser/parser.rs`)

Added phrasal expression parsing in `parse_term()`:

```rust
// Phrasal built-in expressions
if let Some(rest) = s.strip_prefix("total of ") {
    return Ok(Expr::TotalOf(Box::new(parse_expr(rest)?)));
}
if let Some(rest) = s.strip_prefix("smallest in ") {
    return Ok(Expr::SmallestIn(Box::new(parse_expr(rest)?)));
}
// ... (11 more patterns)
```

### 3. VM Evaluation (`runtime/src/vm/vm.rs`)

Updated **3 evaluation functions** to handle phrasal expressions:
- `eval()` - Main expression evaluation
- `eval_in_frame()` - Frame-scoped evaluation for closures
- `eval_in_scope()` - Scope-scoped evaluation
- `eval_in_scope_with_capture()` - Captured scope evaluation
- `dump_expr()` - Expression dumping for debugging

Each function now includes match arms for all 13 phrasal expressions:

```rust
Expr::TotalOf(expr) => {
    let val = self.eval(expr)?;
    builtin_sum(&[val])
}
// ... (12 more match arms)
```

### 4. Example Files

Created/updated 3 example files demonstrating phrasal syntax:

1. **`examples/poh/math_functions.poh`** - Mathematical operations
   - Demonstrates: total of, smallest in, largest in, absolute value of, round, round down, round up

2. **`examples/poh/string_functions.poh`** - String operations
   - Demonstrates: make uppercase, make lowercase, trim spaces from

3. **`examples/poh/collection_functions.poh`** - Collection operations
   - Demonstrates: first in, last in, reverse of

### 5. Documentation Updates

Updated 2 documentation files:

1. **`spec/Vocabulary.md`**
   - Added "Phrasal Built-in Expressions" section with complete reference tables
   - Explained phrasal vs function call syntax preference

2. **`doc/PohLang_Guide.md`**
   - Added comprehensive "Phrasal Expressions (Preferred)" section
   - Provided examples for all 13 phrasal built-ins
   - Added "Why Phrasal?" explanation

## Testing Results

All phrasal built-ins tested and verified working:

### Mathematical Operations Test Output
```
Total of [1, 2, 3, 4, 5] is 15
Smallest: 1
Largest: 5
Absolute value of -42.5 is 42.5
Round 3.7 -> 4
Round down 3.7 -> 3
Round up 3.7 -> 4
```

### String Operations Test Output
```
Original: hello world
Uppercase: HELLO WORLD
Original: STOP SHOUTING
Lowercase: stop shouting
Original: '   clean me up   '
Trimmed: 'clean me up'
```

### Collection Operations Test Output
```
List: [1, 2, 3, 4, 5]
First element: 1
Last element: 5
Reversed list: [5, 4, 3, 2, 1]
Word: PohLang
First char: P
Last char: g
Reversed: gnaLhoP
```

## Benefits

1. **Beginner-Friendly**: Natural English phrases are more intuitive than abbreviations
2. **Readable**: Code reads like prose: "total of numbers" vs "sum(numbers)"
3. **Consistent**: All operations follow the same phrasal pattern
4. **Mission-Aligned**: Maintains PohLang's core philosophy of being approachable

## Backward Compatibility

The traditional function call syntax remains supported:
- `sum(numbers)` still works (calls `builtin_sum()` via `call_function()`)
- Phrasal expressions are preferred but not mandatory
- Existing code continues to work without changes

## Files Modified

1. `runtime/src/parser/ast.rs` - Added 13 Expr variants
2. `runtime/src/parser/parser.rs` - Added phrasal parsing logic
3. `runtime/src/vm/vm.rs` - Updated 5 evaluation functions
4. `examples/poh/math_functions.poh` - Converted to phrasal syntax
5. `examples/poh/string_functions.poh` - Converted to phrasal syntax
6. `examples/poh/collection_functions.poh` - Converted to phrasal syntax
7. `spec/Vocabulary.md` - Added phrasal documentation
8. `doc/PohLang_Guide.md` - Added comprehensive phrasal guide

## Build Status

✅ **Release build successful**: `cargo build --release`  
✅ **All tests passing**: 43 tests (37 smoke + 6 function)  
✅ **All examples working**: 3 new phrasal examples validated

## Conclusion

The implementation successfully transforms PohLang from using traditional programming terminology to natural, beginner-friendly phrasal expressions. This aligns perfectly with the language's mission and makes it significantly more accessible to beginners while maintaining full backward compatibility.

---

**Next Steps**: Consider expanding phrasal expressions to cover more operations (type checking, string manipulation, list operations) as outlined in NEXT_FEATURES.md.
