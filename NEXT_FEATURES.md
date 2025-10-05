# PohLang Next Features & Improvements Plan

**Status**: Phase 1 Completion & Phase 2 Preparation  
**Target**: Q4 2025 - Q1 2026

---

## 🔥 Priority 1: Complete Phase 1 (90% → 100%)

### A. Missing Built-in Functions
Add these core functions to match Python interpreter feature parity:

#### Mathematical Functions
- [ ] `sum(list)` - Sum numeric list elements
- [ ] `min(list)` - Find minimum value in list
- [ ] `max(list)` - Find maximum value in list
- [ ] `abs(number)` - Absolute value
- [ ] `round(number)` - Round to nearest integer
- [ ] `floor(number)` - Round down
- [ ] `ceil(number)` - Round up

#### String Functions
- [ ] `uppercase(text)` - Convert to uppercase
- [ ] `lowercase(text)` - Convert to lowercase
- [ ] `trim(text)` - Remove leading/trailing whitespace
- [ ] `replace(text, old, new)` - String replacement
- [ ] `contains(text, substring)` - Check if substring exists

#### Collection Functions
- [ ] `append(list, item)` - Add item to list (returns new list)
- [ ] `prepend(list, item)` - Add item to start
- [ ] `reverse(list)` - Reverse list order
- [ ] `sort(list)` - Sort list (numeric or lexicographic)
- [ ] `slice(list, start, end)` - Extract sublist
- [ ] `first(list)` - Get first element
- [ ] `last(list)` - Get last element
- [ ] `rest(list)` - Get all but first element (tail)

#### Type Checking/Conversion
- [ ] `type(value)` - Return type name as string
- [ ] `is_number(value)` - Check if numeric
- [ ] `is_text(value)` - Check if string
- [ ] `is_list(value)` - Check if list
- [ ] `to_number(text)` - Parse string to number
- [ ] `to_text(value)` - Convert any value to string

---

## 🚀 Priority 2: Standard Library Foundation (Phase 2 Start)

### B. Collections Module (`collections.poh`)
Create first system module with:

```pohlang
Start Program
Make head with items
    Return items[0]
End

Make tail with items
    Set result to []
    Repeat length(items) minus 1 times
        # Add items[it plus 1] to result
    End
    Return result
End

Make map with items, fn
    Set result to []
    Repeat items
        # Apply fn to each item
    End
    Return result
End

Make filter with items, predicate
    Set result to []
    Repeat items
        # Add item if predicate(it) is true
    End
    Return result
End

Make reduce with items, fn, initial
    Set acc to initial
    Repeat items
        Set acc to fn(acc, it)
    End
    Return acc
End
End Program
```

### C. Math Module (`math.poh`)
Mathematical operations:
- `sqrt(n)` - Square root
- `pow(base, exp)` - Power/exponentiation
- `pi` - Constant 3.14159...
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometry (if feasible in pure PohLang, else Rust FFI)

### D. Random Module (Rust Implementation)
- `random_int(min, max)` - Random integer in range
- `random_float()` - Random float 0.0-1.0
- `random_choice(list)` - Pick random element
- `shuffle(list)` - Randomize list order

### E. File Module (Rust Implementation - Safe)
- `read_text(path)` - Read file as string
- `write_text(path, content)` - Write string to file
- `file_exists(path)` - Check if file exists
- `list_files(directory)` - List directory contents

---

## ⚡ Priority 3: Language Enhancements

### F. Improved Error Messages
- [ ] Add line/column tracking in parser
- [ ] Better error context (show surrounding code)
- [ ] Suggest fixes for common mistakes:
  - "Did you mean 'plus' instead of '+'?"
  - "Missing 'End' for 'If' statement"
  - "Function 'xyz' not found. Similar: 'xzy', 'abc'"

### G. Better REPL Experience
- [ ] Add interactive REPL mode (`pohlang --repl`)
- [ ] Multi-line input support
- [ ] Command history
- [ ] Tab completion for keywords
- [ ] Show variable values

### H. Documentation Improvements
- [ ] Auto-generate function reference from code
- [ ] More example programs
- [ ] Tutorial series (beginner to advanced)
- [ ] Video walkthroughs

---

## 🔧 Priority 4: Performance & Optimization

### I. Bytecode VM (Phase 3 Preview)
Start groundwork:
- [ ] Design bytecode instruction set
- [ ] Create simple bytecode compiler
- [ ] Implement stack-based bytecode VM
- [ ] Benchmark AST vs Bytecode execution

### J. Parser Improvements
- [ ] Cache parsed modules
- [ ] Parallel module loading
- [ ] Better Unicode support
- [ ] Faster tokenization

---

## 📦 Priority 5: Tooling & Ecosystem

### K. Testing Framework
Built-in testing support:
```pohlang
Start Program
Import system "testing"

Make test_addition
    Set result to 2 plus 2
    Assert result is 4 with "Addition works"
End

Make test_strings
    Set greeting to "Hello " plus "World"
    Assert greeting is "Hello World"
End

Run all tests
End Program
```

### L. Package Manager Foundation
- [ ] Package manifest format (`package.poh`)
- [ ] Dependency resolution
- [ ] Package repository structure
- [ ] `pohlang install <package>` command

### M. VS Code Extension
- [ ] Syntax highlighting
- [ ] Code snippets
- [ ] Error diagnostics
- [ ] Auto-formatting
- [ ] Go to definition

---

## 🎨 Priority 6: Advanced Features (Future)

### N. Optional Type Hints (Experimental)
```pohlang
Make greet with name as Text returns Text
    Return "Hello " plus name
End
```

### O. Async/Await (If Needed)
```pohlang
Make fetch_data as async
    Set response to await http_get("api.example.com")
    Return response
End
```

### P. Module System Enhancements
- [ ] Private/public exports
- [ ] Nested modules
- [ ] Re-exports
- [ ] Circular dependency detection

---

## 📊 Success Metrics

### Phase 1 Complete (Target: Dec 2025)
- ✅ All 20+ built-in functions implemented
- ✅ 100+ test cases passing
- ✅ Documentation coverage 90%+
- ✅ Example programs work flawlessly

### Phase 2 Started (Target: Jan 2026)
- ✅ 3+ system modules (`collections`, `math`, `random`)
- ✅ Module aliasing and exposing working
- ✅ 50+ stdlib functions available

### Phase 3 Preview (Target: Mar 2026)
- ✅ Bytecode compiler working
- ✅ 5-10x performance improvement
- ✅ `.pbc` files can be distributed

---

## 🛠️ Implementation Order (Recommended)

### Week 1-2: Built-in Functions
1. Mathematical: `sum`, `min`, `max`, `abs`, `round`
2. String: `uppercase`, `lowercase`, `trim`
3. Collection: `first`, `last`, `reverse`, `append`

### Week 3-4: Collections Module
1. Create `collections.poh` with common list operations
2. Add module loading tests
3. Document all functions

### Week 5-6: Random & Math Modules
1. Implement Rust-backed random functions
2. Create `math.poh` or Rust implementation
3. Add comprehensive examples

### Week 7-8: Error Improvements
1. Enhanced parser error messages
2. Better runtime error context
3. Suggestion system

### Week 9-10: REPL & Tools
1. Interactive REPL mode
2. Better CLI options
3. Performance profiling tools

---

## 💡 Quick Wins (Can Implement Today)

### 1. `sum()` Function
```rust
fn builtin_sum(args: &[Value]) -> Result<Value> {
    if args.is_empty() { return Ok(Value::Num(0.0)); }
    match &args[0] {
        Value::List(xs) => {
            let mut total = 0.0;
            for x in xs {
                match x {
                    Value::Num(n) => total += n,
                    _ => return Err(anyhow!("sum expects numeric values")),
                }
            }
            Ok(Value::Num(total))
        }
        _ => Err(anyhow!("sum expects a list")),
    }
}
```

### 2. `min()` and `max()` Functions
```rust
fn builtin_min(args: &[Value]) -> Result<Value> {
    if args.is_empty() { return Err(anyhow!("min expects 1 argument")); }
    match &args[0] {
        Value::List(xs) if !xs.is_empty() => {
            let mut min = f64::INFINITY;
            for x in xs {
                match x {
                    Value::Num(n) => if *n < min { min = *n; },
                    _ => return Err(anyhow!("min expects numeric values")),
                }
            }
            Ok(Value::Num(min))
        }
        _ => Err(anyhow!("min expects a non-empty list")),
    }
}
```

### 3. String Functions
```rust
fn builtin_uppercase(args: &[Value]) -> Result<Value> {
    if args.len() != 1 { return Err(anyhow!("uppercase expects 1 argument")); }
    Ok(Value::Str(to_string(&args[0]).to_uppercase()))
}

fn builtin_lowercase(args: &[Value]) -> Result<Value> {
    if args.len() != 1 { return Err(anyhow!("lowercase expects 1 argument")); }
    Ok(Value::Str(to_string(&args[0]).to_lowercase()))
}

fn builtin_trim(args: &[Value]) -> Result<Value> {
    if args.len() != 1 { return Err(anyhow!("trim expects 1 argument")); }
    Ok(Value::Str(to_string(&args[0]).trim().to_string()))
}
```

---

## 📝 Notes

- Prioritize functions that enable real-world programs
- Keep syntax simple and consistent
- Maintain excellent error messages
- Write tests for every new feature
- Update documentation immediately

**Next Meeting**: Review implementation progress and adjust priorities based on user feedback.
