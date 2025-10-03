# PohLang Testing Guide

**Welcome, Testers!** 🧪

This guide will help you test PohLang effectively. There are two implementations to test:
1. **Python Interpreter** (stable, feature-complete)
2. **Rust Runtime** (experimental, in development)

---

## Quick Start for Testers

### Prerequisites

**For Python Interpreter:**
```bash
pip install pytest
# Or from source:
pip install -r requirements.txt
```

**For Rust Runtime:**
- No local installation needed if you only want to test via CI
- For local testing: Install Rust (`rustup`) and build tools (see `runtime-rs/README.md`)

---

## Testing Methods

### 1. **Run Automated Tests** (Recommended Start)

#### Python Tests
```bash
# Run all Python tests
python -m pytest tests_python/ -v

# Run specific test file
python -m pytest tests_python/test_interpreter_features.py -v

# Run with coverage
python -m pytest tests_python/ --cov=Interpreter --cov-report=html
```

#### Rust Tests (if you have Rust installed)
```bash
# Build and test
cargo test --manifest-path runtime-rs/Cargo.toml

# Run specific test
cargo test --manifest-path runtime-rs/Cargo.toml run_write_works
```

---

### 2. **Manual Testing with Example Programs**

#### Test with Python Interpreter

```bash
# Run any .poh file
python -m Interpreter.run_poh examples/poh/hello.poh

# Try different examples
python -m Interpreter.run_poh examples/poh/phrase_function.poh
python -m Interpreter.run_poh examples/poh/phrase_repeat.poh
python -m Interpreter.run_poh examples/poh/phrase_age_check.poh
```

#### Test with Rust Runtime (if available)

```bash
# If you built the Rust runtime locally:
cargo run --manifest-path runtime-rs/Cargo.toml -- --run examples/poh/hello.poh

# Or use the compiled binary:
./runtime-rs/target/debug/pohlangc --run examples/poh/hello.poh
```

---

### 3. **Write Your Own Test Programs**

Create test files and try different language features:

#### Basic Output and Variables
```poh
Write "Hello World"
Set x to 42
Write x
Set name to "PohLang"
Write "Welcome to " plus name
```

#### Arithmetic and Comparisons
```poh
Set a to 10
Set b to 5
Write a plus b
Write a is greater than b
Write a is equal to 15
```

#### Conditionals
```poh
Set age to 25
If age is greater than 18 Write "Adult" Otherwise Write "Minor"

Set score to 85
If score is greater than 90
    Write "Excellent"
Otherwise
    Write "Good job"
End
```

#### Loops
```poh
Repeat 5 times
    Write "Hello"
End

Set counter to 0
While counter is less than 3
    Write counter
    Set counter to counter plus 1
End
```

#### Functions
```poh
Make greet with name Write "Hello " plus name
Use greet with "Alice"
Write greet("Bob")

Make add with a, b
    Set result to a plus b
    Return result
End
Write add(5, 3)
```

---

## What to Test

### ✅ **Core Features to Verify**

#### 1. **Basic Statements**
- [ ] `Write` - output to console
- [ ] `Set` - variable assignment
- [ ] `Ask for` - input from user (Python only currently)

#### 2. **Data Types**
- [ ] Numbers: `42`, `3.14`, `-5`
- [ ] Strings: `"Hello"`, `"Multi word string"`
- [ ] Booleans: truthy/falsy values
- [ ] Lists: `Make a list of 1, 2, 3` (Python only)
- [ ] Dictionaries: `Make a dictionary with "key" as "value"` (Python only)

#### 3. **Operators**
- [ ] `plus` - addition/concatenation
- [ ] `minus` - subtraction
- [ ] `times` - multiplication
- [ ] `divided by` - division (Python)
- [ ] Comparisons: `is greater than`, `is less than`, `is equal to`, etc.
- [ ] Logical: `And`, `Or`, `Not`

#### 4. **Control Flow**
- [ ] Inline `If ... Write ... Otherwise ...`
- [ ] Block `If ... End` and `If ... Otherwise ... End`
- [ ] `Repeat N times ... End`
- [ ] `While condition ... End`

#### 5. **Functions**
- [ ] Inline functions: `Make name with params Write expr`
- [ ] Block functions with `Return`
- [ ] Default parameters: `Make greet with name set to "World"`
- [ ] Function calls: `Use name with args` or `name(args)`

#### 6. **Imports**
- [ ] Local imports: `Import "file.poh"`
- [ ] System imports: `Import system "collections"` (Python only)

---

## Testing Checklist

### **Phase 1: Smoke Tests** (Quick validation)

Run these to ensure basic functionality works:

```bash
# Python interpreter
python -m Interpreter.run_poh examples/poh/hello.poh
python -m Interpreter.run_poh examples/poh/phrase_function.poh
python -m Interpreter.run_poh examples/poh/phrase_repeat.poh

# Rust runtime (if available)
cargo run --manifest-path runtime-rs/Cargo.toml -- --run examples/poh/hello.poh
```

**Expected:** All should run without errors and produce output.

---

### **Phase 2: Feature Coverage** (Systematic testing)

Create a test file `my_test.poh` and test each feature:

#### Test 1: Variables and Output
```poh
Write "=== Test 1: Variables ==="
Set x to 5
Set y to 10
Set message to "Hello"
Write x
Write y
Write message
```

**Expected output:**
```
=== Test 1: Variables ===
5
10
Hello
```

#### Test 2: Arithmetic
```poh
Write "=== Test 2: Arithmetic ==="
Set a to 10
Set b to 3
Write a plus b
Write a minus b
Write a times b
```

**Expected output:**
```
=== Test 2: Arithmetic ===
13
7
30
```

#### Test 3: Conditionals
```poh
Write "=== Test 3: Conditionals ==="
Set x to 5
If x is greater than 3 Write "Pass" Otherwise Write "Fail"

If x is equal to 5
    Write "x is 5"
End
```

**Expected output:**
```
=== Test 3: Conditionals ===
Pass
x is 5
```

#### Test 4: Loops
```poh
Write "=== Test 4: Loops ==="
Repeat 3 times
    Write "Loop iteration"
End

Set counter to 0
While counter is less than 3
    Write counter
    Set counter to counter plus 1
End
```

**Expected output:**
```
=== Test 4: Loops ===
Loop iteration
Loop iteration
Loop iteration
0
1
2
```

#### Test 5: Functions
```poh
Write "=== Test 5: Functions ==="
Make double with n Write n times 2
Write double(5)

Make greet with name set to "World" Write "Hello " plus name
Write greet()
Write greet("Alice")
```

**Expected output:**
```
=== Test 5: Functions ===
10
Hello World
Hello Alice
```

---

### **Phase 3: Error Testing** (Edge cases)

Test that errors are caught properly:

#### Test division by zero (Python)
```poh
Write 10 divided by 0
```
**Expected:** Error message with line number

#### Test undefined variable
```poh
Write unknown_variable
```
**Expected:** Error or placeholder value like `<unknown_variable>`

#### Test function arity
```poh
Make add with a, b
    Return a plus b
End
Write add(1)
```
**Expected:** Error about wrong number of arguments

---

## Known Limitations

### **Rust Runtime (Experimental)**
- ⚠️ Missing: `Ask for` input
- ⚠️ Missing: `Increase`/`Decrease` statements
- ⚠️ Missing: Collection literals and operations
- ⚠️ Missing: System library imports
- ⚠️ Missing: Division operator
- ⚠️ Basic error messages (less detailed than Python)

### **Python Interpreter (Stable)**
- ✅ All features implemented
- ✅ Rich error messages with line numbers
- ✅ Full standard library support

---

## Reporting Issues

When you find a bug, please report it with:

### 1. **Minimal Test Case**
The smallest `.poh` program that reproduces the issue:
```poh
Set x to 5
Write x plus "hello"
```

### 2. **Expected vs Actual Behavior**
- **Expected:** Error message about type mismatch
- **Actual:** Program crashes with stack trace

### 3. **Environment Info**
- Which implementation? (Python or Rust)
- Python version: `python --version`
- OS: Windows/Linux/macOS

### 4. **How to Reproduce**
```bash
python -m Interpreter.run_poh test_case.poh
```

### 5. **GitHub Issue Template**
```markdown
**Title:** [Bug] Short description

**Code:**
```poh
Your test case here
```

**Expected:** What should happen

**Actual:** What actually happens

**Environment:**
- Implementation: Python/Rust
- Python: 3.12
- OS: Windows 11

**Steps:**
1. Create file test.poh
2. Run: python -m Interpreter.run_poh test.poh
3. See error
```

---

## Testing Tools

### **Run All Examples**
```bash
# Python: test all examples
for file in examples/poh/*.poh; do
    echo "Testing $file"
    python -m Interpreter.run_poh "$file"
done
```

### **Compare Python vs Rust** (for CI/automation)
```bash
# Create a test file
echo 'Write "Hello"' > test.poh

# Run with Python
python -m Interpreter.run_poh test.poh > python_output.txt

# Run with Rust (if available)
cargo run --manifest-path runtime-rs/Cargo.toml -- --run test.poh > rust_output.txt

# Compare
diff python_output.txt rust_output.txt
```

### **Performance Testing** (optional)
```bash
# Time Python
time python -m Interpreter.run_poh large_program.poh

# Time Rust
time cargo run --release --manifest-path runtime-rs/Cargo.toml -- --run large_program.poh
```

---

## Quick Reference: Testing Priorities

### **High Priority** (Test these first)
1. Basic output: `Write`
2. Variables: `Set`
3. Functions: `Make`, `Use`
4. Conditionals: `If ... End`
5. Loops: `Repeat`, `While`

### **Medium Priority**
6. Arithmetic: `plus`, `minus`, `times`
7. Comparisons: `is greater than`, etc.
8. Imports: `Import "file.poh"`
9. Error messages and line numbers

### **Low Priority** (Nice to have)
10. Collections (Python only)
11. System libraries (Python only)
12. Performance benchmarks

---

## Success Criteria

**A successful test session should verify:**
- ✅ All example programs run without crashing
- ✅ Output matches expected results
- ✅ Error messages are clear and helpful
- ✅ Edge cases are handled gracefully
- ✅ Both Python and Rust (when available) produce same results for core features

---

## Questions?

- **Documentation:** See `PohLang_Guide.md` for full language reference
- **Issues:** Report bugs at `https://github.com/AlhaqGH/PohLang/issues`
- **Discussions:** Ask questions in GitHub Discussions
- **Transition Plan:** See `TRANSITION.md` for Rust runtime status

---

**Happy Testing! 🚀**
