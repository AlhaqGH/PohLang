# PohLang Syntax (v0.1 Draft)

This document describes the dual grammar (phrase mode + symbol mode) for the early prototype.

## High-Level Concepts
# PohLang Syntax (Phrasal v0.1)

This version defines ONLY the phrasal/English-like grammar. Symbol mode has been removed to keep the language focused and approachable.

## High-Level Concepts
- A program is a sequence of statements (one per line).
- Each statement starts with a verb-like keyword: Write / Ask / Set / Increase / Decrease / If / Repeat / Make / Use.
- Functions are currently single-line (inline). Multi-line blocks planned.

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

Comments        : Lines may include comments starting with '#'.
				  The parser strips everything from '#' to end of line.
				  Example: Set x to 1  # this is a comment

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

## Error Strategy
- Provide line numbers in error messages (e.g., "Line 3: Repeat block missing End").
- Suggest canonical phrasing (e.g., "Use 'Write', not 'Say'").
- Future: columns and edit-distance suggestions.

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

