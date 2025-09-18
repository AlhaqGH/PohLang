# PohLang Syntax (v0.1 Draft)

This document describes the dual grammar (phrase mode + symbol mode) for the early prototype.

## High-Level Concepts
# PohLang Syntax (Phrasal v0.1)

This version defines ONLY the phrasal/English-like grammar. Symbol mode has been removed to keep the language focused and approachable.

## High-Level Concepts
- A program is a sequence of statements (one per line).
- Each statement starts with a verb-like keyword: Write / Ask / Set / Increase / Decrease / If / Repeat / Make / Use.
- Functions are currently single-line (inline). Multi-line blocks planned.

## Scoping Rules (Current)
PohLang uses lexical (static) scoping with block boundaries introduced by control structures (`If`, `While`, `Repeat`, function bodies) and explicit anonymous blocks using `Begin ... End`.

Rules:
1. Variables are created on first assignment (`Set`) in the current block scope.
2. An assignment to an existing variable name updates the nearest enclosing scope that already defines it (no implicit shadowing).
3. A name defined inside an `If`/`While`/`Repeat`/`Begin` block is not visible after the corresponding `End` unless it already existed outside (then it is a mutation instead of a new definition).
4. Loop iteration helper `it` (for `Repeat <list>` or `Repeat <dictionary>`) exists only inside each iteration body.
5. Function parameters live in their own function scope; blocks inside a function can define new locals that disappear when the block ends, while mutations to existing variables (including parameters) persist for the rest of the function.
6. Returning from inside a nested block immediately unwinds to the caller.

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
FuncInline       ::= 'Make' IDENT 'with' ParamList 'Write' Expression
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

