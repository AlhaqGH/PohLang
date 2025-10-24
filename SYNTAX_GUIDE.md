# PohLang Syntax Summary

## ✅ FULLY SUPPORTED SYNTAX

### 1. Lists (Phrasal Creation)
```pohlang
// Creating lists - ONLY phrasal syntax
Set nums to Make a list of 1, 2, 3, 4, 5
Set words to Make a list of "hello", "world"
Set mixed to Make a list of 1, "two", 3.0, True

// Accessing elements - brackets for indexing ✅
Write nums[0]           // First element: 1
Write nums[-1]          // Last element: 5
Set val to nums[2]      // Store indexed value: 3
Set idx to 3
Write nums[idx]         // Variable index: 4
```

### 2. Dictionaries (Phrasal Creation)
```pohlang
// Creating dictionaries - ONLY phrasal syntax
Set person to Make a dictionary with "name" set to "Alice", "age" set to 30
Set config to Make a dictionary with "host" set to "localhost", "port" set to 8080

// Accessing values - brackets for indexing ✅
Write person["name"]        // Alice
Set age_val to person["age"] // 30
```

### 3. Nested Collections
```pohlang
// Lists of lists
Set row1 to Make a list of 1, 2, 3
Set row2 to Make a list of 4, 5, 6
Set matrix to Make a list of row1, row2
Write matrix[0][1]      // 2
Write matrix[1][2]      // 6

// String indexing
Set text to "Hello"
Write text[0]           // H
Write text[-1]          // o
```

### 4. Arithmetic Operators
```pohlang
// Symbolic forms ✅
Set sum to a + b
Set diff to a - b
Set prod to a * b
Set quot to a / b

// Phrasal forms ✅ (recommended for readability)
Set sum to a plus b
Set diff to a minus b
Set prod to a times b
Set quot to a divided by b

// Mixed (both work together)
Set result to a + b times 2
Set result2 to (a plus b) * 2
```

### 5. Grouping with Parentheses ✅ **FIXED!**
```pohlang
// Override operator precedence
Set result to (10 + 5) * 2      // 30 (not 20)
Set calc to (a + b) / (c - d)

// Nested grouping
Set nested to ((10 + 5) * 2) - 3    // 27

// With phrasal operators
Set mixed to (a plus b) times c

// In conditions
If (x + y) > 10
    Write "Sum is greater than 10"
End

// Complex expressions
Set complex to (nums[0] + nums[1]) * nums[2]
```

### 6. Comparison Operators
```pohlang
// Symbolic forms ✅
If x > y
If x < y
If x >= y
If x <= y
If x == y
If x != y

// Phrasal forms ✅ (recommended)
If x is greater than y
If x is less than y
If x is at least y
If x is at most y  
If x is equal to y
If x is not equal to y

// With grouping
If (a + b) > (c * d)
    Write "Left side wins"
End
```

### 7. Function Calls
```pohlang
// Phrasal with 'with' keyword ✅ (recommended)
Set msg to greet with "World"
Use print_message with "Hello"

// Parentheses form ✅ (works for built-ins)
Set r to range(5)               // Built-in function
Set txt to join(nums, ", ")     // Built-in function

// Note: Custom functions should use 'with' syntax
```

### 8. Phrasal Built-ins
```pohlang
// Mathematical
Set sum_val to total of numbers
Set max_val to largest in values
Set min_val to smallest in values
Set abs_val to absolute value of -42
Set rounded to round 3.7

// String operations
Set upper to make uppercase text
Set lower to make lowercase text
Set trimmed to trim spaces from text

// Collection operations
Set len to count of items
Set first_item to first in list
Set last_item to last in list
Set reversed to reverse of list
```

## ❌ NOT SUPPORTED (Error Messages)

### Bracket List Literals
```pohlang
Set nums to [1, 2, 3]           // ❌ ERROR!
Set items to []                 // ❌ ERROR!
```
**Error:** "Bracket list literals '[]' are not supported. Use: Make a list of ..."

**Use instead:** 
```pohlang
Set nums to Make a list of 1, 2, 3
```

### Brace Dictionary Literals
```pohlang
Set person to {name: "Alice"}   // ❌ ERROR!
Set data to {}                  // ❌ ERROR!
```
**Error:** "Brace dictionary literals '{}' are not supported. Use: Make a dictionary with ..."

**Use instead:** 
```pohlang
Set person to Make a dictionary with "name" set to "Alice"
```

### Legacy Syntax
```pohlang
Set nums to List contains 1, 2, 3                        // ❌ ERROR!
Set dict to Dictionary contains "key" set to value       // ❌ ERROR!
```
**Error:** "Legacy syntax is not supported. Use: Make a list of / Make a dictionary with"

## 📊 Complete Test Results

### ✅ Indexing Tests - ALL PASS
- ✅ Basic list indexing: `list[0]`, `list[2]`
- ✅ Negative indexing: `list[-1]`, `list[-2]`
- ✅ Variable index: `list[idx]`
- ✅ Dictionary indexing: `dict["key"]`
- ✅ String indexing: `text[0]`, `text[-1]`
- ✅ Nested indexing: `matrix[0][1]`
- ✅ Indexing in expressions: `list[0] + list[1]`
- ✅ Mixed operations: `list[0] + list[1] * 2`

### ✅ Grouping Tests - ALL PASS  
- ✅ Basic grouping: `(10 + 5) * 2`
- ✅ Nested grouping: `((10 + 5) * 2) - 5`
- ✅ Phrasal with grouping: `(a plus b) times c`
- ✅ Mixed operators: `(a + b) times c`
- ✅ Multiple groups: `(10 + 5) * (3 - 1)`
- ✅ In conditions: `If (x + y) > 12`
- ✅ With indexing: `(nums[0] + nums[1]) * nums[2]`
- ✅ Complex nested: `((10 + 5) * (3 - 1)) / 2`
- ✅ Logical operators: `(True And True) Or False`
- ✅ Precedence override: `(10 + 5) * (2 - 3)`

## 🎯 Design Philosophy

PohLang achieves **natural language programming** by:

1. **Phrasal collection creation** - Reads like English
   - `Make a list of` instead of `[]`
   - `Make a dictionary with` instead of `{}`

2. **Symbolic operators for math** - Familiar to programmers
   - `+`, `-`, `*`, `/` work alongside `plus`, `minus`, `times`, `divided by`

3. **Brackets ONLY for accessing** - Clear distinction
   - `list[0]` for indexing (accessing data)
   - NOT for creating data structures

4. **Parentheses for grouping** - Control precedence
   - `(a + b) * c` works correctly
   - Overrides default operator precedence

5. **Phrasal function calls** - Natural syntax
   - `func with arg1, arg2` instead of `func(arg1, arg2)`
   - Reads like giving instructions

This gives you **the best of both worlds**: 
- ✅ Familiar symbolic operators for math and comparisons
- ✅ Natural English for data structures and control flow
- ✅ Clear syntax that's easy to read and understand
