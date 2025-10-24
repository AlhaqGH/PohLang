# PohLang Calculator Web App

A comprehensive demonstration of PohLang's natural language programming features.

## 🚀 Quick Start

### Run the Application

```bash
# From PohLang root directory
cargo run --manifest-path runtime/Cargo.toml -- --run examples/Calculator/app.poh

# Or using PLHub
plhub run examples/Calculator/app.poh
```

### Access the Application

Open your browser to: **http://localhost:8080**

## ✨ Features Demonstrated

### 1. Phrasal Collection Creation
```poh
Set numbers to Make a list of 1, 2, 3, 4, 5
Set person to Make a dictionary with "name" set to "Ada", "age" set to 36
```

### 2. Bracket Indexing
```poh
Write numbers[0]        # First element: 1
Write numbers[-1]       # Last element: 5
Write person["name"]    # Dictionary access: "Ada"
Write matrix[0][1]      # Nested indexing: 2
```

### 3. Parentheses Grouping
```poh
Write 10 + 5 * 2        # 20 (multiplication first - BIDMAS)
Write (10 + 5) * 2      # 30 (parentheses override)
```

### 4. Mixed Operators
```poh
Set result to 10 + 5 - 2          # All symbolic
Set result to 10 plus 5 minus 2   # All phrasal
Set result to (10 + 5) times 2    # Mixed: symbolic + phrasal
```

### 5. Phrasal Built-ins
```poh
Write total of numbers           # Sum
Write smallest in numbers        # Minimum
Write largest in numbers         # Maximum
Write first in numbers           # First element
Write last in numbers            # Last element
Write make uppercase "hello"     # "HELLO"
```

### 6. BIDMAS/PEMDAS Precedence
```poh
# Standard mathematical operator precedence:
# 1. Parentheses ()
# 2. Multiplication/Division (*, /)
# 3. Addition/Subtraction (+, -)

Set x to 10 + 5 * 2              # 20
Set y to (10 + 5) * 2            # 30
Set z to ((10 + 5) * 3) - 2      # 43
```

## 📡 API Endpoints

### GET /
Home page with interactive demo interface

### GET /api/calc
Simple calculator demonstration
```json
{
  "operation": "add",
  "a": 10,
  "b": 5,
  "result": 15
}
```

### GET /api/complex
Complex calculation with grouping and indexing
```poh
# Calculates: (nums[0] + nums[1]) * nums[2] - nums[3]
# Example: (10 + 5) * 3 - 2 = 43
```

### GET /api/stats
Statistics calculator using phrasal built-ins
```json
{
  "data": [23, 45, 12, 67, 34, 89, 15, 56],
  "statistics": {
    "sum": 341,
    "min": 12,
    "max": 89,
    "avg": 42.625,
    "range": 77,
    "count": 8
  }
}
```

### GET /api/demo
Complete syntax demonstration showing all features

## 🎯 Key Concepts

### Collections: Create vs Access

**Creating Collections (Phrasal Only):**
```poh
# ✅ CORRECT
Set list to Make a list of 1, 2, 3
Set dict to Make a dictionary with "key" set to "value"

# ❌ WRONG - These throw errors
Set list to [1, 2, 3]              # Bracket literals NOT supported
Set dict to {"key": "value"}       # Brace literals NOT supported
```

**Accessing Elements (Brackets Only):**
```poh
# ✅ CORRECT - Brackets only for indexing
Write list[0]                      # Access first element
Write dict["key"]                  # Access dictionary value
Write matrix[i][j]                 # Nested indexing
```

### Operator Flexibility

```poh
# All these are valid - choose what feels natural!
Set a to 10 + 5                    # Symbolic
Set b to 10 plus 5                 # Phrasal
Set c to 10 + 5 times 2            # Mixed (evaluates to 20)
Set d to (10 + 5) times 2          # Mixed with grouping (30)
```

### Grouping for Precedence

```poh
# Without grouping - BIDMAS applies
Set x to 10 + 5 * 2                # 20 (multiply first)

# With grouping - override precedence
Set y to (10 + 5) * 2              # 30 (add first)

# Complex grouping
Set z to ((a + b) * c) - (a / b)   # Nested parentheses
```

## 🛠️ Code Structure

```
examples/Calculator/
├── app.poh          # PohLang backend server
├── index.html       # Interactive web interface
└── README.md        # This file
```

### app.poh Components

1. **Calculator Functions**
   - `calculate` - Basic arithmetic operations
   - `complex_calc` - Demonstrates grouping and indexing
   - `calculate_stats` - Phrasal built-ins showcase

2. **Route Handlers**
   - `handle_home` - Serves HTML interface
   - `handle_simple_calc` - Simple calculations
   - `handle_complex_calc` - Complex expressions
   - `handle_stats` - Statistics calculator
   - `handle_demo` - Full feature demonstration

3. **Router**
   - Maps URLs to handlers
   - Returns JSON or HTML responses

## 💡 Learning Examples

### Example 1: Indexing in Expressions
```poh
Set numbers to Make a list of 5, 10, 15, 20
Set result to (numbers[0] + numbers[1]) * numbers[2]
# Result: (5 + 10) * 15 = 225
```

### Example 2: Nested Data Structures
```poh
Set matrix to Make a list of 
  Make a list of 1, 2, 3,
  Make a list of 4, 5, 6,
  Make a list of 7, 8, 9

Write matrix[1][1]                 # 5 (center element)
```

### Example 3: Statistics with Phrasal Built-ins
```poh
Set scores to Make a list of 85, 92, 78, 95, 88

Set total_score to total of scores         # 438
Set highest to largest in scores           # 95
Set lowest to smallest in scores           # 78
Set average to total_score / length(scores) # 87.6
```

### Example 4: Mixed Operator Styles
```poh
Make calculate with a, b, c
  # Mix symbolic and phrasal freely
  Set step1 to a + b
  Set step2 to step1 times c
  Set step3 to step2 - 10
  Return step3
End

Write calculate with 5, 3, 2       # ((5 + 3) * 2) - 10 = 6
```

## 📚 Documentation References

- **PohLang_Guide.md** - Complete language guide
- **CHEAT_SHEET.md** - Quick reference
- **SYNTAX_GUIDE.md** - Detailed syntax with test results
- **Vocabulary.md** - Full list of keywords

## 🧪 Testing the App

1. **Start the server**
   ```bash
   cargo run --manifest-path runtime/Cargo.toml -- --run examples/Calculator/app.poh
   ```

2. **Open browser to localhost:8080**

3. **Click the test buttons** to see API responses

4. **Try the API directly:**
   ```bash
   curl http://localhost:8080/api/calc
   curl http://localhost:8080/api/complex
   curl http://localhost:8080/api/stats
   curl http://localhost:8080/api/demo
   ```

## ✅ What This Demonstrates

- ✅ Phrasal collection creation (`Make a list of`, `Make a dictionary with`)
- ✅ Bracket indexing `[]` for accessing elements
- ✅ Parentheses grouping `()` for precedence control
- ✅ Mixed symbolic (`+`, `-`, `*`, `/`) and phrasal operators
- ✅ Phrasal built-ins (`total of`, `smallest in`, etc.)
- ✅ BIDMAS/PEMDAS operator precedence
- ✅ Nested data structures and indexing
- ✅ Web routing and JSON/HTML responses
- ✅ Real-world application structure

## 🎓 Next Steps

1. Modify the calculations in `app.poh`
2. Add new API endpoints
3. Create more complex statistical functions
4. Build your own PohLang web application!

## 🐛 Troubleshooting

**Server won't start?**
- Check that port 8080 is not already in use
- Verify PohLang runtime is built: `cargo build --manifest-path runtime/Cargo.toml`

**API returns errors?**
- Check server console output for error messages
- Verify PohLang syntax matches current version

**Browser can't connect?**
- Ensure server is running
- Try http://127.0.0.1:8080 instead of localhost

## 📝 License

Part of the PohLang project. See LICENSE file in root directory.
