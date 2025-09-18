# PohLang Syntax (v0.1 Draft)

This document describes the dual grammar (phrase mode + symbol mode) for the early prototype.

## High-Level Concepts
# PohLang Syntax (Phrasal v0.1)

This version defines ONLY the phrasal/English-like grammar. Symbol mode has been removed to keep the language focused and approachable.

## High-Level Concepts
- A program is a sequence of statements (one per line).
- Each statement starts with a verb-like keyword: Write / Ask / Set / Increase / Decrease / If / Repeat / Make / Use.
- Functions support both single-line (inline) and multi-line block forms with parameters and returns.

## Scoping Rules (Current)
PohLang uses lexical (static) scoping with block boundaries introduced by control structures (`If`, `While`, `Repeat`, function bodies) and explicit anonymous blocks using `Begin ... End`.

Rules:
1. Variables are created on first assignment (`Set`) in the current block scope.
2. An assignment to an existing variable name updates the nearest enclosing scope that already defines it (no implicit shadowing).
3. A name defined inside an `If`/`While`/`Repeat`/`Begin` block is not visible after the corresponding `End` unless it already existed outside (then it is a mutation instead of a new definition).
4. Loop iteration helper `it` (for `Repeat <list>` or `Repeat <dictionary>`) exists only inside each iteration body.
5. Function parameters live in their own function scope; blocks inside a function can define new locals that disappear when the block ends, while mutations to existing variables (including parameters) persist for the rest of the function.
6. Returning from inside a nested block immediately unwinds to the caller.
7. New variables created inside a block (`If`/`While`/`Repeat`/`Begin`) are confined to that block and do not leak outside.

### Function Scoping Specifics
- Parameters shadow outer variables with the same name.
- New names created via `Set` inside a function become function-locals unless they already exist in an outer scope.
- Inner blocks do not leak their fresh variables to the function body (true lexical block scoping).
- Recursion is supported; each call gets a fresh function environment.

### Anonymous Blocks
Use `Begin ... End` to introduce a new scope without control flow:
```
Set x to 1
Begin
	Set y to 2
	Set x to x + y   # mutates outer x
End
Write x    # 3
Write y    # error: y is not defined here
```

Nested `Begin` blocks behave exactly like nested `If` blocks with respect to scoping.

## Grammar (Current Subset)
Program          ::= { Statement NEWLINE }
Statement        ::= WriteStmt | AskStmt | SetStmt | IncStmt | DecStmt | IfInline | WhileInline | RepeatInline | FuncInline | CallStmt

WriteStmt        ::= 'Write' Expression
AskStmt          ::= 'Ask for' IDENT
SetStmt          ::= 'Set' IDENT 'to' Expression | 'Set' IDENT Expression
IncStmt          ::= 'Increase' IDENT 'by' NUMBER | 'Increase' IDENT NUMBER
DecStmt          ::= 'Decrease' IDENT 'by' NUMBER | 'Decrease' IDENT NUMBER
IfInline         ::= 'If' Condition 'Write' Expression 'Otherwise' 'Write' Expression
WhileInline      ::= 'While' Condition 'Write' Expression
RepeatInline     ::= 'Repeat' (NUMBER|IDENT) 'times' 'Write' Expression | 'Repeat' (NUMBER|IDENT) 'Write' Expression
FuncInline       ::= 'Make' IDENT 'with' ParamList 'Write' Expression  # inline form (implicit Return)
FuncBlock        ::= 'Make' IDENT 'with' ParamList NEWLINE { Statement | ReturnStmt } 'End'
ReturnStmt       ::= 'Return' [ Expression ]
CallStmt         ::= 'Use' IDENT 'with' ArgList

ParamList        ::= IDENT { ',' IDENT } | /* empty */
ArgList          ::= Expression { ',' Expression } | /* empty */

Condition        ::= BoolExpr
BoolExpr         ::= BoolExpr ' Or ' BoolExpr | BoolExpr ' And ' BoolExpr | 'Not ' BoolExpr | CompareExpr
CompareExpr      ::= Expression CompareOp Expression | Expression '=' Expression
CompareOp        ::= 'Greater Or Equal' | 'Less Or Equal' | 'Equals' | 'Not Equals' | 'Greater Than' | 'Less Than'

Expression       ::= Term { ('+' | '-') Term }
Term             ::= Factor { ('*' | '/') Factor }          // * and / may not yet be parsed in implementation
Factor           ::= NUMBER | STRING | IDENT | '(' Expression ')'

## Functions

### Inline Function
```
Make greet with name Write "Hello, " + name
Write greet("World")   # Hello, World
```

### Block Function
```
Make add with a, b
	Set total to a + b
	Return total
End
Write add(3,4)   # 7
```

### Recursive Function
```
Make fact with n
	If n is 0
		Return 1
	End
	Return n * fact(n - 1)
End
Write fact(5)   # 120
```

Arity must match exactly; a mismatch raises a runtime error.

## Built-In Functions
The interpreter ships with a small standard library of pure helpers:

| Name | Forms | Result | Notes |
|------|-------|--------|-------|
| `length(x)` | `length List contains 1,2,3` | Number | Works on strings, lists, dictionaries |
| `sum(list)` | `sum List contains 1,2,3` | Number | Elements must be numeric |
| `min(list)` | `min List contains 4,2,9` | Number | Non-empty numeric list |
| `max(list)` | `max List contains 4,2,9` | Number | Non-empty numeric list |
| `range(n)` | `range(5)` | List | 0..n-1 |
| `range(start,end)` | `range(2,5)` | List | start..end-1 |
| `range(start,end,step)` | `range(0,10,2)` | List | step progression (no zero step) |
| `join(list, sep)` | `join(nums, ":")` | String | Coerces elements; sep optional (default "") |
| `split(text, sep)` | `split("a,b,c", ",")` | List | Simple separator split |
| `now()` | `now()` | String | ISO-8601 timestamp |

Example:
```
Set nums to range(5)
Write join(nums, ",")    # 0,1,2,3,4
Write length(nums)        # 5
Write now()
```

### Misuse Examples (Runtime Errors)
These demonstrate error conditions the runtime reports:

| Code | Error (summary) | Reason |
|------|------------------|--------|
| `range(1,5,0)` | "range expects 1 to 3 arguments" or infinite-step prevention (future explicit check) | Zero step unsupported |
| `join(5, ",")` | "join expects a list" | First arg must be list/tuple |
| `sum List contains 1,"a"` | "sum expects numeric values" | Non-numeric in numeric fold |
| `min List contains` | parse error | Empty literal / syntax issue |
| `split("abc", "")` | Valid (returns every char) | Edge but allowed |

Future improvement: specific message for zero step in `range`.

## Recipes
Practical multi-line idioms using existing primitives.

### Join Numbers 1..N
```
Set out to ""
Repeat range(1,6)
	Set out to out + it + ","
End
Write out
```

### Sum Only Even Numbers 0..9
```
Set total to 0
Repeat range(10)
	If it is even
		Set total to total + it
	End
End
Write total   # 20
```

### Filter (Collect) Odd Numbers
```
Set odds to List contains
Repeat range(10)
	If it is odd
		Add it to odds
	End
End
Write odds
```

### Timestamped Log Line
```
Write "Started at " + now()
```

### Split, Transform, Rejoin
```
Set parts to split("a-b-c", "-")
Repeat parts
	Set it to it + it   # (planned: future map sugar)
End
Write join(parts, ":")
```

### Factorial (Recursion Recipe)
```
Make fact with n
	If n is 0
		Return 1
	End
	Return n * fact(n - 1)
End
Write fact(6)  # 720
```

### Helper Function For Filtering
```
Set evens to List contains
Make pushIfEven with n
	If n is even
		Add n to evens
	End
	Return nothing
End
Repeat range(10)
	Use pushIfEven with it
End
Write evens
```

## Desugaring
Increase X by N => Set X X + N (make number bigger)
Decrease X by N => Set X X - N (make number smaller)
Repeat N Write E => internal counted loop => for i in [0..N)
Make f with a,b Write E => create a function returning E

## Planned (Not Implemented Yet)
- Multi-line If / Repeat blocks:
	If age > 18
		Write "Adult"
		Otherwise
		Write "Minor"
- Parentheses for grouping in boolean expressions.
- Data structures: lists, dictionaries.
- Random numbers, predicates (is even, etc.).

## Error Strategy (Planned)
- Unrecognized keyword suggestion (edit distance).
- Provide line/column in messages.
- Show expected pattern after failing a statement parse.

## Example
```
Ask for name
Write "Hello " + name
Set count 3
Repeat count Write "Hi"
If count is greater than 1 Write "Many" Otherwise Write "Few"
Make greet with who Write "Hi " + who
Use greet with "Poh"
```

## Implementation Notes
- Current parser is single-pass, line-based.
- Expression parsing presently supports + and - only (improve with precedence parser later).
- All variables are implicitly declared on first assignment.

---
The syntax will evolve carefully; simplicity and readability take priority over feature breadth.
- Comparison chaining.
- Predicates (is even, etc.).

