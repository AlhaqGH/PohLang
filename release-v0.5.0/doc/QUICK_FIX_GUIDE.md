# Quick Fix Guide for PohLang Spec/Runtime Alignment

## 🚨 URGENT: Use This Syntax (What Actually Works)

### Program Structure
```pohlang
Start Program
    <your code here>
End Program
```

### Functions
```pohlang
# ❌ DON'T (documented but wrong):
Function greet(name)
    Return "Hello " plus name
End Function

# ✅ DO (what actually works):
Make greet with name
    Return "Hello " plus name
End
```

### If Statements
```pohlang
# ❌ DON'T:
If x is greater than 5
    Write "big"
Else
    Write "small"
End If

# ✅ DO:
If x is greater than 5
    Write "big"
Otherwise
    Write "small"
End
```

### Loops
```pohlang
# ❌ DON'T:
While count
    Write count
End While

Repeat 5 times
    Write "hi"
End Repeat

# ✅ DO:
While count
    Write count
End

Repeat 5 times
    Write "hi"
End
```

### Collections
```pohlang
# ✅ Modern syntax (Phase 1):
Set nums to [1, 2, 3]
Set dict to {name: "Alice", age: 30}

# ✅ Legacy syntax (also works but undocumented):
Set nums to List contains 1, 2, 3
Set dict to Dictionary contains "name" set to "Alice", "age" set to 30
```

### Operators
```pohlang
# ✅ Arithmetic:
plus, minus, times, divided by

# ✅ Comparison:
is equal to, is not equal to
is greater than, is less than
is at least, is at most

# ⚠️ Logical (defined but needs testing):
And, Or, Not
```

### Input
```pohlang
# ✅ ONLY this works:
Ask for name

# ❌ These are documented but NOT implemented:
Ask for age expect Number
Ask for name with "Enter name:"
```

### Calling Functions
```pohlang
# ✅ Both work:
Use greet with "Alice"         # Statement form
Write greet("Alice")            # Expression form

# ⚠️ Also works but undocumented:
Call greet with "Alice"         # Synonym for Use
```

---

## 📝 Documentation Updates Needed

### Grammar.md Changes:

1. Line ~250: Change `Function` → `Make`
2. Line ~200: Change `Else` → `Otherwise`
3. All block terminators: Change `End If`, `End While`, etc. → `End`
4. Line ~117: Move `Ask for` options to Phase 2
5. Remove `Function` keyword completely or mark as alias

### Vocabulary.md Changes:

1. Update all code examples to use `Make` not `Function`
2. Standardize on `Otherwise` not `Else`
3. Show `End` not `End If`, `End While`, etc.
4. Mark advanced `Ask for` syntax as Phase 2
5. Document that `Call` and `Use` are synonyms
6. Add note about legacy collection syntax

---

## 🧪 Tests to Add

```rust
// test logical operators
#[test]
fn logical_and_or_not() {
    let path = write_program(&[
        "Set x to 1",
        "Set y to 0",
        "If x And Not y",
        "    Write \"pass\"",
        "End",
    ]);
    // ...
}

// test length builtin
#[test]
fn length_builtin() {
    let path = write_program(&[
        "Set nums to [1, 2, 3]",
        "Write length(nums)",
    ]);
    // should output 3
}

// test split builtin
#[test]
fn split_builtin() {
    let path = write_program(&[
        "Set words to split(\"a,b,c\", \",\")",
        "Write words",
    ]);
    // should output [a, b, c]
}

// test return without value
#[test]
fn return_no_value() {
    let path = write_program(&[
        "Make test with x",
        "    If x",
        "        Return",
        "    End",
        "    Write \"after\"",
        "End",
        "Use test with 1",
    ]);
    // should return early, not print "after"
}

// test exposing without as
#[test]
fn exposing_without_alias() {
    let path = write_program(&[
        "Import system \"collections\" exposing head",
        "Set nums to [1, 2]",
        "Write head(nums)",
    ]);
    // should work without alias
}
```

---

## 🔧 Code Changes Needed

### parser.rs:

```rust
// Option 1: Support both Make and Function
if let Some(rest) = t.strip_prefix("Make ")
    .or_else(|| t.strip_prefix("Function ")) {
    // ... existing code
}

// Option 2: Decide on Call vs Use
// Either remove Call support:
// - Delete lines 107-113
// Or document Call as official synonym
```

### Grammar.md:

```diff
- function-def = "Function" , ws1 , identifier , "(" , [ param-list ] , ")" , separator
+ function-def = "Make" , ws1 , identifier , ws1 , "with" , ws1 , [ param-list ] , separator

- "End If"
- "End While"  
- "End Repeat"
- "End Function"
+ "End"

- "Else"
+ "Otherwise"
```

### Vocabulary.md:

```diff
- | Block Function | `Function <name>(<params>) ... Return <expression> End Function`
+ | Block Function | `Make <name> with <params> ... Return <expression> End`

- | If Block with alternative | `If <condition> ... Else ... End If`
+ | If Block with alternative | `If <condition> ... Otherwise ... End`

- `Ask for <var> [expect <Type>] [with "Prompt"]`
+ `Ask for <var>`  (Phase 1 - simple form only)
```

---

## ⚡ Quick Command to Fix Examples

```bash
# Add Start/End Program wrapper to all examples
cd examples/poh
for file in *.poh; do
    if ! grep -q "Start Program" "$file"; then
        echo "Start Program" > temp.poh
        cat "$file" >> temp.poh
        echo "End Program" >> temp.poh
        mv temp.poh "$file"
    fi
done
```

---

## 📊 Alignment Status

| Feature | Spec | Runtime | Status |
|---------|------|---------|--------|
| `Start Program` / `End Program` | ✅ | ✅ | ✅ Aligned |
| `Write` | ✅ | ✅ | ✅ Aligned |
| `Set ... to ...` | ✅ | ✅ | ✅ Aligned |
| `Ask for` (simple) | ✅ | ✅ | ✅ Aligned |
| `Ask for ... expect ...` | ✅ | ❌ | ❌ Spec only |
| `Make` keyword | ❌ | ✅ | ❌ Runtime only |
| `Function` keyword | ✅ | ❌ | ❌ Spec only |
| `Otherwise` | ✅ | ✅ | ✅ Aligned |
| `Else` | ✅ | ❌ | ❌ Spec only |
| `End` (blocks) | ❌ | ✅ | ❌ Runtime only |
| `End If`, `End While`, etc. | ✅ | ❌ | ❌ Spec only |
| Collections `[1,2,3]` | ✅ | ✅ | ✅ Aligned |
| Legacy collections | ❌ | ✅ | ❌ Runtime only |
| `Use` statement | ✅ | ✅ | ✅ Aligned |
| `Call` statement | ❌ | ✅ | ❌ Runtime only |
| Logical operators | ✅ | ⚠️ | ⚠️ Needs tests |
| Built-ins | ⚠️ | ⚠️ | ⚠️ Partial |

**Summary**: 7 aligned ✅ | 6 spec-only ❌ | 3 runtime-only ❌ | 3 needs-verification ⚠️

---

## 🎯 Priority Actions

1. **TODAY**: Update Grammar.md and Vocabulary.md (2 hours)
2. **THIS WEEK**: Add missing tests (3 hours)
3. **THIS WEEK**: Fix or wrap all examples (1 hour)
4. **NEXT WEEK**: Decide on Function vs Make keyword (30 min + discussion)
5. **NEXT WEEK**: Implement or remove documented features (varies)

---

*Generated: October 5, 2025*  
*Based on: SPEC_VS_RUNTIME_ANALYSIS.md*
