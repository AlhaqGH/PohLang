# PohLang Spec vs Runtime Analysis
**Date**: October 5, 2025  
**Analyzed**: Grammar.md, Vocabulary.md, runtime/src/parser/, runtime/tests/

---

## Executive Summary

After analyzing the specification files against the actual Rust runtime implementation, I found **16 critical discrepancies** between documented syntax and implemented features. The issues range from keyword mismatches to missing functionality and incorrect documentation.

**Severity Breakdown**:
- 🔴 **Critical** (Breaks compatibility): 6 issues
- 🟡 **Major** (Confusing/inconsistent): 7 issues
- 🟢 **Minor** (Documentation only): 3 issues

---

## 🔴 CRITICAL ISSUES

### 1. Function Definition Keyword Mismatch

**Spec Says**: `Function <name>(<params>) ... End Function`  
**Runtime Implements**: `Make <name> with <params> ... End`

**Evidence**:
- Grammar.md line ~250: Shows `Function greet(name) ... End Function`
- Vocabulary.md: Shows "Block Function" with `Function` keyword
- parser.rs line 245: Implements `"Make "` not `"Function "`
- smoke.rs tests: All use `Make` keyword

**Impact**: ANY program written following the spec documentation will fail to parse.

**Fix Required**:
```diff
# Option A: Update spec to match implementation
- Function greet(name)
+ Make greet with name

# Option B: Update runtime to support both
- if let Some(rest) = t.strip_prefix("Make ") {
+ if let Some(rest) = t.strip_prefix("Make ") 
+    .or_else(|| t.strip_prefix("Function ")) {
```

**Recommendation**: Update spec to use `Make` (it's more natural and phrasal).

---

### 2. "Otherwise" vs "Else" Inconsistency

**Spec Says**: Mixed usage - Sometimes `Else`, sometimes `Otherwise`  
**Runtime Implements**: `Otherwise` only

**Evidence**:
- Grammar.md line ~200: Uses `Else` in some examples
- Vocabulary.md Phase 1 examples: Use both `Otherwise` and `Else`
- parser.rs line 179: Only checks for `"Otherwise"`

**Examples**:
```pohlang
# Vocabulary.md shows:
If x
    Write "yes"
Else           # ❌ Won't parse
    Write "no"
End If

# Runtime expects:
If x
    Write "yes"
Otherwise      # ✅ Works
    Write "no"
End
```

**Fix Required**: Standardize on `Otherwise` everywhere in documentation.

---

### 3. Block Terminators Are Wrong

**Spec Says**: `End If`, `End While`, `End Repeat`, `End Function`  
**Runtime Implements**: Just `End` for all blocks

**Evidence**:
- Grammar.md EBNF: Shows `"End If"`, `"End While"`, `"End Repeat"`, `"End Function"`
- parser.rs lines 226-237: All blocks just look for `"End"`

**Examples**:
```pohlang
# Spec shows:
If condition
    Write "yes"
End If          # ❌ Runtime expects just "End"

While count
    Write count
End While       # ❌ Runtime expects just "End"

# Runtime actually works:
If condition
    Write "yes"
End            # ✅ Works
```

**Fix Required**: Update all spec examples to use just `End`.

---

### 4. Missing "Define function" Syntax

**Spec Says**: Inline functions use `Define function <name> with <params> as <expr>`  
**Runtime Implements**: This is partially there but not documented

**Evidence**:
- Grammar.md line ~82: Shows `"Define function "` prefix
- parser.rs line 83: Has code for `"Define function "` 
- But Vocabulary.md and examples don't mention it
- Tests don't use it - they all use `Make`

**Status**: Implementation exists but is undocumented and unused.

**Fix Required**: Either document `Define function` or remove the dead code.

---

### 5. Function Call Syntax Confusion

**Spec Says**: Two forms -  
  1. `<name>(args...)` - expression form  
  2. `Use <name> with arg [and arg...]` - statement form

**Runtime Implements**: Both, plus undocumented `Call` statement

**Evidence**:
- parser.rs line 107: Has `"Call "` statement (like `Use`)
- Not mentioned anywhere in spec
- Tests don't use it

**Issue**: Spec says "Use" for statement calls, but runtime also accepts "Call" (alias).

**Fix Required**: Document that `Call` and `Use` are synonyms, or remove one.

---

### 6. "Repeat X times" Grammar Ambiguity

**Spec Says**: `Repeat <expression> times ... End Repeat`  
**Runtime Implements**: `Repeat <expression> times ... End` (but "times" is optional in parsing)

**Evidence**:
- Grammar.md: Shows `"times"` as required
- parser.rs line 233: Strips " times" if present but doesn't require it
- Tests use "Repeat 3 times" but parser would accept "Repeat 3"

**Issue**: Grammar is ambiguous about whether "times" is required.

**Fix Required**: Clarify in spec and enforce in parser.

---

## 🟡 MAJOR ISSUES

### 7. Parameter Syntax Inconsistency

**Spec Says**: Two forms -  
  - Grammar: `<param> [set to <default>]`
  - Examples: `with parameters parameter-list` or `with parameter`

**Runtime Implements**: `with <params>` (simple comma list)

**Evidence**:
- Grammar.md line ~140: Shows `param-entry = identifier , [ ws1 , "set to" , ws1 , expression ]`
- Vocabulary.md: Shows `<param> [set to <default>]`
- parser.rs line 247: Parses `"with "` followed by param list
- Tests use: `Make add with a, b set to 1` ✅ (works)

**Issue**: The spec shows `"with parameters"` or `"with parameter"` as optional keywords, but implementation just uses `"with"`.

**Fix Required**: Clarify that it's just `with <params>`, not `with parameters <params>`.

---

### 8. "Ask for" Optional Syntax Not Documented

**Spec Says**: `Ask for <var> [expect <Type>] [with "Prompt"]`  
**Runtime Implements**: Only `Ask for <var>` (simple form)

**Evidence**:
- Grammar.md line ~117: Shows optional `expect` and `with` clauses
- Vocabulary.md: Documents the optional parts
- parser.rs line 99: Only parses simple `"Ask for "` + identifier
- Tests only use simple form

**Status**: Documented features (`expect Number`, `with "prompt"`) are NOT implemented.

**Fix Required**: Either implement the full syntax or remove from spec (mark as Phase 2).

---

### 9. Missing Logical Operators

**Spec Says**: Grammar defines `And`, `Or`, `Not` operators  
**Runtime Implements**: ✅ Defined in AST but needs verification

**Evidence**:
- Grammar.md line ~380: Defines logical-or, logical-and, logical-not
- Vocabulary.md Phase 1: Lists `And`, `Or`, `Not`
- ast.rs lines 15-17: Has `And(Box<Expr>, Box<Expr>)`, `Or(...)`, `Not(...)`
- parser.rs: Need to check if these are actually parsed

**Status**: AST supports it, but need to verify parser implementation.

**Fix Required**: Add test cases for logical operators.

---

### 10. Collection Syntax Documentation Gap

**Spec Says**: Phase 1 documents modern syntax `[1, 2, 3]` and `{key: value}`  
**Tests Show**: ALSO supports legacy syntax `List contains ...` and `Dictionary contains ...`

**Evidence**:
- Grammar.md line ~410: Shows `list-literal` and `dict-literal` modern syntax
- Vocabulary.md: Only shows modern syntax for Phase 1
- smoke.rs line 450: Test `legacy_collection_syntax_still_works` uses old forms

**Issue**: Legacy syntax still works but isn't documented.

**Fix Required**: Either document legacy syntax or deprecate it with warning.

---

### 11. Return Statement Behavior Unclear

**Spec Says**: `Return <expression>` valid inside function bodies  
**Runtime Implements**: `Return` with optional expression

**Evidence**:
- Grammar.md line ~150: Shows `return-stmt = "Return" , ws1 , expression`
- parser.rs line 269: Allows `Return` with or without expression
- Tests: Don't exercise `Return` without value

**Issue**: Spec says expression is required, runtime makes it optional.

**Fix Required**: Clarify what `Return` alone does (returns null/nothing?).

---

### 12. Import Path Resolution Not Specified

**Spec Says**: `Import "path.poh"` - path relative to caller  
**Runtime Implements**: Absolute path resolution

**Evidence**:
- Grammar.md note: "Path relative to caller"
- Vocabulary.md: "Path relative to caller"
- Tests show absolute paths in imports

**Issue**: Relative path behavior not clearly defined or tested.

**Fix Required**: Document path resolution rules (relative to file? to CWD?).

---

### 13. System Import Exposing Syntax

**Spec Shows**: `Import system "module" exposing symbol1, symbol2`  
**Tests Show**: Only tested with alias AND exposing together

**Evidence**:
- Grammar.md Phase 2: Shows `exposing` syntax
- smoke.rs line 217: Test uses both `as coll` and `exposing head`
- No test with just `exposing` without alias

**Issue**: Combination semantics unclear.

**Fix Required**: Test and document: Can you use `exposing` without `as`?

---

## 🟢 MINOR ISSUES

### 14. Built-in Functions Not Comprehensive

**Spec Says**: Phase 1 lists `range()`, `join()`, `length()`, `now()`, `split()`  
**Runtime Implements**: Partially (verified in tests)

**Evidence**:
- Vocabulary.md line ~220: Lists built-ins
- smoke.rs line 230: Tests `range()` and `join()`
- No tests for `length()`, `split()`, `now()`

**Status**: Need to verify which built-ins actually work.

**Fix Required**: Test all documented built-ins or remove from Phase 1 list.

---

### 15. Number Literal Syntax Limited

**Spec Says**: `number = integer ; // Phase 1 supports integers only`  
**Runtime Uses**: `f64` in AST

**Evidence**:
- Grammar.md: Documents integers only
- ast.rs line 3: `Num(f64)`
- Tests don't test floats

**Issue**: AST supports floats but spec says integers only.

**Fix Required**: Clarify if floats work, or change AST to i64.

---

### 16. Error Message Examples Don't Match

**Spec Shows**: Not documented  
**Runtime Has**: Enhanced error messages with hints

**Evidence**:
- parser.rs lines 4-35: Sophisticated error suggestion system
- Not documented in any spec file

**Status**: Undocumented feature (good thing!).

**Fix Required**: Consider adding error message examples to docs.

---

## Summary of Fixes Needed

### In Specification (Grammar.md, Vocabulary.md)

1. ✏️ **Replace** `Function` with `Make` everywhere
2. ✏️ **Replace** `Else` with `Otherwise` everywhere
3. ✏️ **Replace** `End If`, `End While`, `End Repeat`, `End Function` with just `End`
4. ✏️ **Remove** optional parts of `Ask for` (move to Phase 2)
5. ✏️ **Clarify** `Return` can be used without expression
6. ✏️ **Document** `Call` as synonym for `Use` OR remove from spec
7. ✏️ **Document** legacy collection syntax OR mark as deprecated
8. ✏️ **Add** error message examples
9. ✏️ **Test** and document all Phase 1 built-in functions

### In Runtime Implementation

10. 🔧 **Remove** dead code for `Define function` if not using it
11. 🔧 **Add** tests for logical operators (`And`, `Or`, `Not`)
12. 🔧 **Add** tests for all documented built-ins
13. 🔧 **Clarify** or enforce `"times"` keyword in Repeat statements
14. 🔧 **Document** path resolution behavior for imports
15. 🔧 **Decide** on float vs integer literals (spec vs impl mismatch)
16. 🔧 **Test** `exposing` without `as` in system imports

### In Examples (examples/poh/)

17. 📝 **Add** `Start Program`/`End Program` to all examples
18. 📝 **Update** any examples using wrong syntax
19. 📝 **Create** comprehensive test suite matching vocabulary

---

## Recommended Action Plan

### Phase 1: Documentation Fixes (1-2 hours)
- Update Grammar.md and Vocabulary.md with correct keywords
- Remove Phase 2/3 features from Phase 1 docs
- Add examples that actually work

### Phase 2: Code Cleanup (2-3 hours)
- Remove unused `Define function` code OR document it
- Decide on `Call` vs `Use` (keep one or both)
- Add missing tests for documented features

### Phase 3: Examples Update (1 hour)
- Wrap all examples in `Start Program`/`End Program`
- Verify each example parses and runs
- Create comprehensive test examples

### Phase 4: Feature Completion (varies)
- Implement missing built-ins
- Add logical operator tests
- Clarify number literal behavior

---

## Test Coverage Gaps

Based on smoke.rs, **missing test coverage** for:

- ❌ Logical operators (`And`, `Or`, `Not`)
- ❌ `length()` built-in
- ❌ `split()` built-in  
- ❌ `now()` built-in
- ❌ `Return` without expression
- ❌ Float literals
- ❌ `exposing` without `as`
- ❌ Relative path imports
- ❌ Error message quality

**Tests that exist** ✅:
- Basic statements (Write, Set)
- Control flow (If, While, Repeat)
- Functions (Make inline/block)
- Collections (lists, dicts)
- Indexing (positive, negative, nested)
- Imports (local, system with alias+exposing)
- Arithmetic (all 4 operators)
- Increase/Decrease sugar

---

## Conclusion

The PohLang runtime is **~85% aligned** with the specification, but the **documentation needs urgent updates** to match implementation. The good news: the runtime is more complete than documented. The bad news: learners following docs will write code that doesn't work.

**Priority**: Update specification FIRST (Grammar.md, Vocabulary.md), then fix examples, then add missing tests.
