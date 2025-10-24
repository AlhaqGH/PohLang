# PohLang Cheat Sheet

Quick reference for PohLang syntax and common patterns.

---

## 📋 Basic Syntax

### Variables
```poh
Set name to "Ada"
Set age to 36
Set price to 19.99
Set active to true
```

### Output
```poh
Write "Hello World"
Write name
Write "Hello " plus name
```

---

## 🔢 Operators

### Arithmetic (Symbolic or Phrasal)
```poh
# Symbolic
Set sum to 10 + 5
Set diff to 10 - 5
Set product to 10 * 5
Set quotient to 10 / 5

# Phrasal
Set sum to 10 plus 5
Set diff to 10 minus 5
Set product to 10 times 5
Set quotient to 10 divided by 5

# Mix both styles freely!
Set result to (10 + 5) times 2    # 30
```

### Comparison (Symbolic or Phrasal)
```poh
# Symbolic
If x == 10 Write "Equal"
If x != 10 Write "Not equal"
If x > 10 Write "Greater"
If x < 10 Write "Less"
If x >= 10 Write "Greater or equal"
If x <= 10 Write "Less or equal"

# Phrasal
If x is 10 Write "Equal"
If x is not 10 Write "Not equal"
If x is greater than 10 Write "Greater"
If x is less than 10 Write "Less"
```

### Logical
```poh
If x and y Write "Both true"
If x or y Write "At least one true"
If not x Write "False"
If (x > 5) and (y < 10) Write "Both conditions"
```

### Operator Precedence (BIDMAS/PEMDAS)
```poh
Write 10 + 5 * 2        # 20 (multiply first)
Write (10 + 5) * 2      # 30 (parentheses override)
Write ((a + b) * c) - d # Complex grouping
```

---

## 📦 Collections

### Creating Collections (Phrasal Only)
```poh
# Lists
Set numbers to Make a list of 1, 2, 3, 4, 5
Set colors to Make a list of "red", "green", "blue"
Set mixed to Make a list of 1, "two", true, 4.5

# Dictionaries
Set person to Make a dictionary with "name" set to "Ada", "age" set to 36
Set config to Make a dictionary with "debug" set to true, "port" set to 8080

# ❌ Bracket literals NOT supported
# Set nums to [1, 2, 3]              # ERROR!
# Set dict to {"key": "value"}       # ERROR!
```

### Accessing Elements (Brackets for Indexing)
```poh
# List indexing
Write numbers[0]        # First element: 1
Write numbers[-1]       # Last element: 5
Write numbers[2]        # Third element: 3

# Dictionary indexing
Write person["name"]    # "Ada"
Write person["age"]     # 36

# String indexing
Set word to "Hello"
Write word[0]           # "H"
Write word[-1]          # "o"

# Nested indexing
Set matrix to Make a list of Make a list of 1, 2, Make a list of 3, 4
Write matrix[0][1]      # 2
Write matrix[1][0]      # 3

# Using in expressions
Set sum to numbers[0] + numbers[1]           # 3
Set product to (numbers[0] + numbers[1]) * 2 # 6
```

---

## 🔁 Control Flow

### If/Otherwise/End
```poh
If age >= 18
  Write "Adult"
Otherwise
  Write "Minor"
End

# One-line if
If age >= 18 Write "Adult"
```

### While Loop
```poh
Set count to 0
While count < 5
  Write count
  Set count to count + 1
End
```

### Repeat Loop
```poh
Repeat 5 times
  Write "Hello"
End

# With collection
Set items to Make a list of 1, 2, 3
Repeat length(items) times
  Write "Item"
End
```

---

## 🛠️ Functions

### Inline Function (Single Expression)
```poh
Make greet with name Write "Hello " plus name
Write greet with "Ada"
```

### Block Function
```poh
Make add with a, b
  Set sum to a + b
  Return sum
End

Write add with 10, 5
```

### Default Parameters
```poh
Make hello with name set to "World"
  Write "Hi " plus name
End

Write hello              # Hi World
Write hello with "Ada"   # Hi Ada
```

### First-Class Functions
```poh
Make double with x Return x * 2
Set myFunc to double
Write myFunc with 5      # 10
```

---

## 📚 Built-in Functions

### Traditional Syntax
```poh
length(x)     # Size of string, list, or dict
len(x)        # Alias for length
range(n)      # List of numbers 0 to n-1
range(start, end, step)
join(list, separator)
split(text, separator)
now()         # Current timestamp
```

### Phrasal Syntax (Preferred)
```poh
# Math
total of numbers              # Sum
smallest in numbers           # Minimum
largest in numbers            # Maximum
absolute value of -42         # 42
round 3.7                     # 4
round down 3.9                # 3
round up 3.1                  # 4

# Strings
make uppercase "hello"        # "HELLO"
make lowercase "WORLD"        # "world"
trim spaces from "  text  "   # "text"

# Collections
first in numbers              # First element
last in numbers               # Last element
reverse of numbers            # Reversed list
count of numbers              # Size (alias of length)

# Membership
contains 3 in numbers         # True/False
contains "x" in "text"        # True/False

# Modification (returns new collection)
append 6 to numbers           # Add to end
remove 3 from numbers         # Remove element
```

---

## 📥 Modules

### Import Local File
```poh
Import "utils.poh"
Use helper with "data"
```

### Import System Module
```poh
Import system "collections"
```

---

## 🌐 Web Framework (Phase 6)

### Basic Server
```poh
Import system "web"

Make handler with request
  Return Make a dictionary with "status" set to 200, "body" set to "Hello World"
End

Start server on 8080 with handler
```

### JSON Response
```poh
Make api_handler with request
  Set data to Make a dictionary with "message" set to "OK", "code" set to 200
  Return Make a dictionary with "status" set to 200, "body" set to data
End

Start server on 8080 with api_handler
```

### Routes
```poh
Make router with request
  Set path to request["path"]
  If path is "/"
    Return Make a dictionary with "status" set to 200, "body" set to "Home"
  End
  If path is "/about"
    Return Make a dictionary with "status" set to 200, "body" set to "About"
  End
  Return Make a dictionary with "status" set to 404, "body" set to "Not Found"
End

Start server on 8080 with router
```

---

## 💡 Tips & Tricks

### Grouping with Parentheses
```poh
# Use () to override operator precedence
Write (10 + 5) * 2                  # 30
Write ((a + b) * c) - (a / b)       # Complex grouping

# Works with indexing
Write (numbers[0] + numbers[1]) * 2
```

### Mix Symbolic and Phrasal
```poh
# All valid - choose what feels natural!
Set x to 10 + 5 * 2                # All symbolic
Set y to 10 plus 5 times 2         # All phrasal
Set z to (10 + 5) times 2          # Mixed
```

### String Concatenation
```poh
Set full_name to first_name plus " " plus last_name
Set message to "Hello " plus name plus "!"
```

### Chaining Operations
```poh
Set result to make uppercase trim spaces from "  hello  "  # "HELLO"
```

---

## ❌ Common Mistakes

### Don't Use Bracket/Brace Literals
```poh
# ❌ WRONG
Set nums to [1, 2, 3]
Set dict to {"key": "value"}

# ✅ CORRECT
Set nums to Make a list of 1, 2, 3
Set dict to Make a dictionary with "key" set to "value"
```

### Brackets Only for Indexing
```poh
# ❌ Creating collections
Set nums to [1, 2, 3]              # ERROR

# ✅ Accessing elements
Set first to nums[0]               # Correct
Set value to dict["key"]           # Correct
```

### Function Calls Need Arguments
```poh
# ❌ Missing arguments
Write greet                        # Only valid if function has defaults

# ✅ Provide arguments
Write greet with "Ada"             # Correct
```

---

## 🚀 Quick Start Example

```poh
# Variables and output
Set name to "Ada"
Write "Hello " plus name

# Collections
Set numbers to Make a list of 1, 2, 3, 4, 5
Write numbers[0]                   # 1
Write total of numbers             # 15

# Functions
Make double with x Return x * 2
Write double with 5                # 10

# Conditionals
If length(numbers) > 3
  Write "Long list"
Otherwise
  Write "Short list"
End

# Loops
Repeat 3 times
  Write "Loop iteration"
End

# Mix operators
Set result to (numbers[0] + numbers[1]) * 2
Write result                       # 6
```

---

## 📖 Further Reading

- **PohLang_Guide.md** - Complete language guide
- **Vocabulary.md** - Full list of keywords and built-ins
- **SYNTAX_GUIDE.md** - Detailed syntax reference with test results
- **examples/** - Sample programs demonstrating features
