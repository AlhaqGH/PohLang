# Phase 2 Kickoff: Standard Library Development
**Start Date:** October 9, 2025  
**Duration:** Q1 2026 (3 months)  
**Focus:** Standard Library Foundation

---

## 🎯 Phase 2 Overview

Phase 2 focuses on building PohLang's standard library to provide essential functionality for real-world applications. This phase transforms PohLang from a demonstration language into a practical tool for building useful programs.

### Goals
1. ✅ Implement core standard library modules
2. ✅ Improve developer experience with better error messages
3. ✅ Create comprehensive example programs
4. ✅ Build community through tutorials and documentation

---

## 📚 Standard Library Modules (Priority Order)

### 1. File I/O Module ⭐⭐⭐ (FIRST)
**Status:** Starting Now  
**Priority:** Critical - Most requested feature

**Features:**
- Read text files
- Write text files
- Append to files
- Check if file exists
- Get file information (size, modified date)
- Create/delete files and directories

**API Design:**
```pohlang
Start Program
    # Read entire file
    Make content = Read file "data.txt"
    Write content
    
    # Write to file (creates or overwrites)
    Make text = "Hello, World!"
    Write to file "output.txt" the text
    
    # Append to file
    Append to file "log.txt" the text
    
    # Check if file exists
    If file exists "config.txt":
        Make config = Read file "config.txt"
    Otherwise:
        Write "Config file not found"
    End If
    
    # File information
    Make size = file size of "data.txt"
    Make modified = last modified of "data.txt"
    
    # Directory operations
    Create directory "outputs"
    If directory exists "outputs":
        Write to file "outputs/result.txt" the data
    End If
    
    # Read lines as list
    Make lines = Read lines from "data.txt"
    Make count = count of lines
    Write count
    
    # Write lines from list
    Make items = ["apple", "banana", "cherry"]
    Write lines to file "fruits.txt" from items
End Program
```

**Implementation Plan:**
1. Add file I/O functions to Rust runtime
2. Implement phrasal syntax parsing
3. Add error handling for file operations
4. Write comprehensive tests
5. Create 5+ example programs
6. Update documentation

**Timeline:** 2-3 weeks

---

### 2. Collections Module ⭐⭐
**Status:** Planned  
**Priority:** Medium

**Features:**
- Advanced list operations (sort, filter, map)
- Set operations (union, intersection, difference)
- Dictionary helpers (keys, values, merge)
- Stack and queue operations

**API Examples:**
```pohlang
# Sort list
Make numbers = [5, 2, 8, 1, 9]
Make sorted = sort ascending numbers
Make descending = sort descending numbers

# Filter list
Make evens = filter numbers where x % 2 == 0

# Map/transform list
Make doubled = map numbers with x * 2

# Sets
Make set1 = Make a set from [1, 2, 3, 4]
Make set2 = Make a set from [3, 4, 5, 6]
Make union = union of set1 and set2
Make intersection = intersection of set1 and set2
```

**Timeline:** 2 weeks

---

### 3. Math Module ⭐⭐
**Status:** Planned  
**Priority:** Medium

**Features:**
- Trigonometric functions (sin, cos, tan)
- Logarithms and exponentials
- Power and square root
- Constants (pi, e)
- Random number generation

**API Examples:**
```pohlang
# Constants
Make pi = pi constant
Make e = e constant

# Trigonometry
Make angle = 45
Make sine = sine of angle
Make cosine = cosine of angle

# Power and roots
Make squared = power of 2 to 8  # 2^8
Make root = square root of 16

# Logarithms
Make log = log of 100 base 10
Make natural = natural log of 2.718
```

**Timeline:** 1 week

---

### 4. Random Module ⭐
**Status:** Planned  
**Priority:** Low-Medium

**Features:**
- Random integers
- Random floats
- Random choice from list
- Shuffle list
- Random seed setting

**API Examples:**
```pohlang
# Random integer
Make dice = random integer from 1 to 6
Make lottery = random integer from 1 to 100

# Random float
Make rand = random float from 0 to 1

# Random choice
Make colors = ["red", "blue", "green"]
Make chosen = random choice from colors

# Shuffle
Make deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Make shuffled = shuffle deck
```

**Timeline:** 1 week

---

### 5. DateTime Module ⭐
**Status:** Planned  
**Priority:** Low-Medium

**Features:**
- Get current date/time
- Format dates
- Parse dates
- Date arithmetic
- Time zones

**API Examples:**
```pohlang
# Current date/time
Make now = current datetime
Make today = current date
Make time = current time

# Format date
Make formatted = format date now as "YYYY-MM-DD"

# Date arithmetic
Make tomorrow = add days to today 1
Make next_week = add weeks to today 1

# Parse date
Make date = parse date "2025-10-09" as "YYYY-MM-DD"
```

**Timeline:** 1-2 weeks

---

## 🐛 Error Message Improvements

### Current Issues
- Parse errors don't always show exact location
- Missing "Did you mean...?" suggestions for typos
- No stack traces for function calls
- Generic error messages

### Improvements Planned

**1. Better Parse Errors**
```
Before:
Error: Could not parse expression

After:
Error at line 5, column 12: Could not parse expression
  | Make result = totl of numbers
  |               ^^^^
Did you mean: total of
```

**2. Stack Traces**
```
Error at line 15 in function calculate:
  Division by zero
  Called from line 8 in function process
  Called from line 3 in main program
```

**3. Helpful Suggestions**
```
Error: Keyword 'Mak' not recognized
Did you mean: Make

Error: Function 'pint' not found
Did you mean: print, Write
```

**Implementation:** Throughout Phase 2

---

## 📖 Example Programs to Create

### Beginner Level
1. **Todo List App** (File I/O)
   - Add/remove/list tasks
   - Save to file
   - Load from file

2. **Simple Calculator** (Math)
   - Basic operations
   - Scientific functions
   - History

3. **Number Guessing Game** (Random)
   - Generate random number
   - Get user guesses
   - Track attempts

### Intermediate Level
4. **Log Analyzer** (File I/O + Collections)
   - Read log files
   - Count errors/warnings
   - Generate summary

5. **Contact Manager** (File I/O + Collections)
   - Add/search/delete contacts
   - Save to JSON-like format
   - Search by name

6. **Dice Roller** (Random)
   - Roll multiple dice
   - Calculate statistics
   - Simulate probabilities

### Advanced Level
7. **Text Adventure Game** (File I/O + Collections)
   - Load game state
   - Save progress
   - Room navigation

8. **Expense Tracker** (File I/O + DateTime + Math)
   - Track expenses
   - Calculate totals by category
   - Monthly reports

9. **Simple Database** (File I/O + Collections)
   - CRUD operations
   - Query data
   - Export results

---

## 📝 Documentation Tasks

### 1. Standard Library Reference
- API documentation for each module
- Code examples for every function
- Usage patterns and best practices

### 2. Tutorial Series
- "Your First PohLang Program"
- "Working with Files in PohLang"
- "Building a Todo List App"
- "Creating a Simple Game"

### 3. Video Content
- 5-minute "Getting Started" video
- File I/O tutorial video
- Project showcase video

### 4. Community Resources
- Contributing guide for beginners
- Code of conduct
- Issue templates
- Discussion categories on GitHub

---

## 🏗️ Implementation Strategy

### Week 1-2: File I/O Module
- [ ] Design API (phrasal syntax)
- [ ] Implement in Rust runtime
- [ ] Add tests (20+ test cases)
- [ ] Create 3 example programs
- [ ] Write documentation

### Week 3-4: Error Messages
- [ ] Improve parse error reporting
- [ ] Add "Did you mean" suggestions
- [ ] Implement stack traces
- [ ] Add location tracking

### Week 5-6: Collections Module
- [ ] Design API
- [ ] Implement sort, filter, map
- [ ] Add set operations
- [ ] Create examples
- [ ] Documentation

### Week 7-8: Math Module
- [ ] Implement trig functions
- [ ] Add logarithms/exponentials
- [ ] Constants and utilities
- [ ] Examples and docs

### Week 9-10: Random + DateTime
- [ ] Random module implementation
- [ ] DateTime module implementation
- [ ] Integration examples
- [ ] Complete documentation

### Week 11-12: Polish & Release
- [ ] Create example programs (9 total)
- [ ] Record tutorial videos
- [ ] Update all documentation
- [ ] Release v0.6.0

---

## 🎬 File I/O Module - Detailed Design

### Phrasal Syntax

**Read Operations:**
```pohlang
Read file "path.txt"
Read lines from "path.txt"
Read bytes from "path.txt"
```

**Write Operations:**
```pohlang
Write to file "path.txt" the data
Write lines to file "path.txt" from list
Write bytes to file "path.txt" from bytes
```

**Append Operations:**
```pohlang
Append to file "path.txt" the data
Append line to file "path.txt" the line
```

**File Checks:**
```pohlang
file exists "path.txt"  # Returns true/false
directory exists "path"
is file "path.txt"
is directory "path"
```

**File Information:**
```pohlang
file size of "path.txt"  # Returns bytes
last modified of "path.txt"  # Returns timestamp
file extension of "path.txt"  # Returns ".txt"
file name of "path/to/file.txt"  # Returns "file.txt"
```

**Directory Operations:**
```pohlang
Create directory "outputs"
Create directories "path/to/nested"  # Creates all parents
Remove file "path.txt"
Remove directory "path"
List files in "path"  # Returns list of filenames
```

### Rust Implementation Structure

```rust
// src/stdlib/io.rs
pub mod file_io {
    use std::fs;
    use std::path::Path;
    
    pub fn read_file(path: &str) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))
    }
    
    pub fn write_file(path: &str, content: &str) -> Result<(), String> {
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file '{}': {}", path, e))
    }
    
    pub fn append_file(path: &str, content: &str) -> Result<(), String> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
            
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write to file '{}': {}", path, e))
    }
    
    // ... more functions
}
```

### Parser Updates

```rust
// Recognize "Read file" phrasal expression
if matches!((keyword1, keyword2), ("Read", "file")) {
    let path = parse_string_literal()?;
    return Ok(Expr::PhrasalBuiltin("read_file", vec![path]));
}

// Recognize "Write to file" phrasal expression
if matches!((keyword1, keyword2, keyword3), ("Write", "to", "file")) {
    let path = parse_string_literal()?;
    // expect "the"
    let data = parse_expression()?;
    return Ok(Expr::PhrasalBuiltin("write_file", vec![path, data]));
}
```

### Error Handling

```pohlang
Start Program
    # Try to read file with error handling
    If file exists "data.txt":
        Make content = Read file "data.txt"
        Write content
    Otherwise:
        Write "Error: File not found"
    End If
End Program
```

### Test Cases

```rust
#[test]
fn test_read_file() {
    // Create temp file
    // Read with PohLang
    // Assert content matches
}

#[test]
fn test_write_file() {
    // Write with PohLang
    // Read back and verify
    // Clean up
}

#[test]
fn test_append_file() {
    // Create file
    // Append multiple times
    // Verify all content present
}

#[test]
fn test_file_not_found_error() {
    // Try to read non-existent file
    // Assert proper error message
}
```

---

## 📊 Success Metrics

### Technical
- ✅ All 5 standard library modules implemented
- ✅ 100+ new test cases passing
- ✅ Error messages improved with suggestions
- ✅ No regressions from Phase 1

### Documentation
- ✅ Complete API reference for all modules
- ✅ 9 example programs created
- ✅ 3 tutorial videos recorded
- ✅ Community contribution guide published

### Community
- ✅ GitHub Discussions set up with categories
- ✅ First beta testers recruited
- ✅ Installation tested on all platforms
- ✅ Positive feedback from early users

---

## 🚀 Next Steps (Immediate)

1. **Start File I/O Implementation** (This Week)
   ```bash
   cd runtime/src
   mkdir -p stdlib
   touch stdlib/mod.rs
   touch stdlib/io.rs
   ```

2. **Design Phrasal Syntax** (Today)
   - Review proposed syntax above
   - Refine based on PohLang's natural language goals
   - Document in spec/

3. **Set Up Tests** (This Week)
   ```bash
   cd runtime/tests
   mkdir -p stdlib
   touch stdlib/io_tests.rs
   ```

4. **Create Example** (This Week)
   ```bash
   cd examples/poh
   touch file_io_basics.poh
   touch todo_list.poh
   ```

---

## 📅 Milestones

| Milestone | Target Date | Deliverables |
|-----------|-------------|--------------|
| File I/O Module | Oct 23, 2025 | Module + Tests + Examples |
| Error Improvements | Oct 30, 2025 | Better errors + Suggestions |
| Collections Module | Nov 13, 2025 | Module + Tests + Examples |
| Math + Random | Nov 27, 2025 | Both modules complete |
| DateTime Module | Dec 11, 2025 | Module + Integration examples |
| Phase 2 Complete | Dec 25, 2025 | v0.6.0 Release |

---

## 🎯 Phase 2 Completion Criteria

Phase 2 is complete when:
- ✅ All 5 standard library modules implemented and tested
- ✅ Error messages significantly improved
- ✅ 9 example programs created and documented
- ✅ Tutorial videos published
- ✅ GitHub Discussions active
- ✅ Installation tested on Windows/Linux/macOS
- ✅ Beta user feedback incorporated
- ✅ v0.6.0 released successfully

---

**Document Owner:** GitHub Copilot  
**Created:** October 9, 2025  
**Status:** Active - Phase 2 Starting  
**Next Review:** Weekly progress updates
