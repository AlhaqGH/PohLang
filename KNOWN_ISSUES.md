# Known Issues - PohLang

This document tracks known issues in PohLang releases.

---

## ✅ Fixed Issues

### Parser Issue with Multi-line If Statements (v0.5.0)

**Status**: ✅ **FIXED** in v0.5.1  
**Severity**: High  
**Impact**: Multi-line If statements with phrasal comparisons failed to parse in v0.5.0

#### Problem Description (v0.5.0)

The PohLang v0.5.0 official runtime had a parser issue where multi-line If statements using phrasal comparison operators failed to parse correctly.

#### Affected Syntax in v0.5.0

```pohlang
# ❌ This FAILED in v0.5.0:
If temperature is greater than 20
    Write "Warm"
Otherwise
    Write "Cool"
End If
```

#### Fix in v0.5.1

✅ **Fixed** by adding full phrasal comparison operator support:
- Added " is greater than " pattern
- Added " is less than " pattern  
- Added " is greater than or equal to " pattern
- Added " is less than or equal to " pattern
- Added " is equal to " pattern
- Added " is not equal to " pattern
- Fixed "End If" keyword recognition

✅ All multi-line If statements now work correctly in v0.5.1

#### Testing

All previously failing examples now pass:
```bash
✅ examples/poh/if_block_greeting.poh
✅ examples/poh/phrase_age_check.poh
✅ All 50 tests passing
```

---

## 📊 Current Issues

**None** - All known issues resolved in v0.5.1 ✅

--- What Still Works ✅

#### 1. All Phrasal Built-in Expressions (20 Total)
```pohlang
# ✅ Mathematical operations
Set sum to total of [1, 2, 3, 4, 5]              # Works perfectly
Set max to largest in numbers                     # Works perfectly
Set min to smallest in [10, 5, 8]                 # Works perfectly
Set abs to absolute value of -42                  # Works perfectly
Set r1 to round 3.7                               # Works perfectly
Set r2 to round down 3.9                          # Works perfectly
Set r3 to round up 3.1                            # Works perfectly

# ✅ String operations
Set upper to make uppercase "hello"               # Works perfectly
Set lower to make lowercase "WORLD"               # Works perfectly
Set clean to trim spaces from "  text  "          # Works perfectly

# ✅ Collection operations
Set first_item to first in my_list                # Works perfectly
Set last_item to last in my_list                  # Works perfectly
Set reversed to reverse of numbers                # Works perfectly
Set size to count of items                        # Works perfectly
Set text to join words with ", "                  # Works perfectly
Set parts to split "a,b,c" by ","                 # Works perfectly
Set has_item to contains 5 in numbers             # Works perfectly
Set removed to remove 3 from list                 # Works perfectly
Set added to append 6 to list                     # Works perfectly
Set inserted to insert 10 at 2 in list            # Works perfectly
```

#### 2. Inline If Statements
```pohlang
# ✅ Single-line If statements work
If 5 is greater than 3 Write "Yes" Otherwise Write "No"
If temperature is greater than 20 Write "Warm" Otherwise Write "Cool"
If age is less than or equal to 18 Write "Minor" Otherwise Write "Adult"
```

#### 3. All Other Core Features
- ✅ Variables and assignments
- ✅ User-defined functions
- ✅ Collections (lists and dictionaries)
- ✅ Arithmetic operations (`+`, `-`, `*`, `/`, `%`)
- ✅ String concatenation
- ✅ Loops (`While`, `Repeat ... times`)
- ✅ Symbol-based comparisons (`>`, `<`, `=`, `≠`, `≥`, `≤`)

### Workarounds

Choose one of these approaches until v0.5.1 is released:

#### Option 1: Use Inline If Statements (Recommended)
```pohlang
# Convert multi-line to single-line
If temperature is greater than 20 Write "Warm" Otherwise Write "Cool"
```

#### Option 2: Use Symbol Syntax
```pohlang
# Use symbol-based operators instead of phrasal
If temperature > 20
    Write "Warm"
Otherwise
    Write "Cool"
End If
```

#### Option 3: Use Intermediate Variables
```pohlang
# Store comparison result first
Set is_warm to temperature is greater than 20
If is_warm
    Write "Warm"
Otherwise
    Write "Cool"
End If
```

#### Option 4: Use Nested Expression
```pohlang
# Put comparison in parentheses (if supported)
If (temperature is greater than 20)
    Write "Warm"
Otherwise
    Write "Cool"
End If
```

### Verification Details

**Tested with**:
- **Runtime**: `pohlang.exe` v0.5.0 from official release
- **Source**: https://github.com/AlhaqGH/PohLang/releases/download/v0.5.0/pohlang-v0.5.0-windows-x64.zip
- **Date**: October 5, 2025
- **Platform**: Windows x64

**Example Test Results**:
```bash
# Working examples from official release
✅ examples/poh/hello.poh                    - Basic output
✅ examples/poh/arithmetic.poh               - Math operations
✅ examples/poh/math_functions.poh           - Phrasal math expressions
✅ examples/poh/string_functions.poh         - Phrasal string operations
✅ examples/poh/collections_phrasal.poh      - Phrasal collection operations
✅ examples/poh/collection_functions.poh     - Collection manipulation
✅ examples/poh/fact.poh                     - Recursion
✅ examples/poh/phrase_function.poh          - Function definitions
✅ examples/poh/phrase_repeat.poh            - Loops

❌ examples/poh/if_block_greeting.poh        - Multi-line If with comparison
❌ examples/poh/phrase_age_check.poh         - Multi-line If with comparison
```

### Impact Assessment

#### Affected Use Cases
1. **Multi-line conditional logic** with phrasal operators
2. **Examples in official release** that use block-style If statements
3. **PLHub templates** that include multi-line conditionals

#### Not Affected
1. **All 20 phrasal built-in expressions** - Work perfectly
2. **Inline If statements** - Work perfectly
3. **Core language features** - All functional
4. **50 passing tests** - Test suite validates core functionality

### PLHub Template Compatibility

**PLHub v0.5.0 Impact**:
- Some templates include multi-line If statements with phrasal comparisons
- These will fail when run with v0.5.0 runtime
- Templates need to be updated to use inline If statements

**Affected PLHub Templates**:
- `examples/example.poh` - Basic template with conditionals
- Other templates with multi-line conditional logic

**Recommendation**:
1. Update PLHub templates to use inline If statements for v0.5.0 compatibility
2. Add note in template comments about known issue
3. Re-test templates with v0.5.1 when released

### Fix Plan

**Target Release**: v0.5.1 (patch release)

**Proposed Solution**:
1. Fix parser to properly handle phrasal operators in multi-line contexts
2. Add regression tests for multi-line If statements with all phrasal operators
3. Verify all affected examples work correctly
4. Update official release with fixed runtime

**Testing Checklist**:
- [ ] Multi-line If with `is greater than`
- [ ] Multi-line If with `is less than`
- [ ] Multi-line If with `is greater than or equal to`
- [ ] Multi-line If with `is less than or equal to`
- [ ] Multi-line If with `is equal to`
- [ ] Multi-line If with `is not equal to`
- [ ] Nested multi-line If statements
- [ ] Multi-line If with complex expressions
- [ ] All affected examples pass

### Timeline

- **2025-10-05**: Issue identified in official v0.5.0 release
- **2025-10-06**: Known issue documented
- **Target**: v0.5.1 patch release within 1-2 weeks
- **Follow-up**: Update PLHub templates after v0.5.1 release

---

## 📊 Issue Summary

| Issue | Status | Severity | Workaround | Fix Version |
|-------|--------|----------|------------|-------------|
| Multi-line If with phrasal comparisons | 🔴 Open | High | Use inline If | v0.5.1 |

---

## 🔍 Reporting New Issues

If you encounter additional issues:

1. **Check this document** - Issue may already be known
2. **Try workarounds** - See if there's a temporary solution
3. **Report on GitHub**: https://github.com/AlhaqGH/PohLang/issues
4. **Include**:
   - PohLang version (`pohlang --version`)
   - Minimal code example that reproduces the issue
   - Error message (if any)
   - Expected vs actual behavior
   - Platform (Windows, Linux, macOS)

---

## 📝 Related Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history
- [RELEASE_NOTES_v0.5.0.md](RELEASE_NOTES_v0.5.0.md) - v0.5.0 details
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development guidelines
- [PohLang_Guide.md](doc/PohLang_Guide.md) - Language documentation

---

## ⚠️ Important Notes

1. **Phrasal Built-ins Are Not Affected**: All 20 phrasal built-in expressions work perfectly. This issue is specific to phrasal comparison operators in multi-line If statements.

2. **Core Functionality Intact**: 50/50 tests pass. The language core is stable and production-ready for educational use.

3. **Workarounds Available**: Multiple workarounds exist (see above). The issue is inconvenient but not blocking.

4. **Patch Release Coming**: v0.5.1 will address this issue specifically.

---

**Last Updated**: October 5, 2025  
**Applies To**: PohLang v0.5.0 official release  
**Next Release**: v0.5.1 (patch)
