# PohLang v0.5.0 - Quick Start Guide

Welcome to PohLang! This package contains everything you need to start programming in natural language.

## What's Included

- `pohlang.exe` - PohLang compiler and runtime
- `README.md` - Project overview
- `RELEASE_NOTES.md` - What's new in v0.5.0
- `LICENSE` - MIT License
- `doc/` - Complete documentation
- `examples/` - Sample programs

## Quick Start (3 Steps)

### 1. Verify Installation

Open PowerShell or Command Prompt in this folder and run:

```powershell
.\pohlang.exe --version
```

You should see: `pohlang 0.5.0`

### 2. Run Your First Program

Create a file named `hello.poh`:

```pohlang
Start Program

Write "Hello, World!"
Write "Welcome to PohLang v0.5.0!"

Set name to "Alice"
Write "Hello " plus name

Set numbers to [10, 20, 30]
Write "Total: " plus total of numbers

End Program
```

Run it:

```powershell
.\pohlang.exe --run hello.poh
```

### 3. Try the Examples

Explore the included examples:

```powershell
# Mathematical operations
.\pohlang.exe --run examples\poh\math_functions.poh

# String operations
.\pohlang.exe --run examples\poh\string_functions.poh

# Collection operations
.\pohlang.exe --run examples\poh\collection_functions.poh

# Complete demo
.\pohlang.exe --run examples\poh\collections_phrasal.poh
```

## Language Features

### Variables
```pohlang
Set x to 42
Set name to "PohLang"
Set numbers to [1, 2, 3, 4, 5]
```

### Arithmetic
```pohlang
Set result to 10 plus 5 times 2    # 20 (correct precedence!)
Set diff to 100 minus 30
Set product to 6 times 7
Set quotient to 50 divided by 5
```

### Comparisons
```pohlang
If x greater than 10
    Write "x is large"
End If

If name equals "PohLang"
    Write "Correct!"
End If
```

### Loops
```pohlang
Repeat 5 times
    Write "Hello!"
End Repeat

Set i to 0
While i less than 10
    Write i
    Increase i by 1
End While
```

### Functions
```pohlang
Define function greet with parameter name as
    Write "Hello " plus name
End Function

Call greet with "World"
```

### Phrasal Built-ins (NEW in v0.5.0!)

#### Mathematical (7)
```pohlang
Write total of [1, 2, 3, 4, 5]        # 15
Write smallest in [5, 2, 8, 1]        # 1
Write largest in [5, 2, 8, 1]         # 8
Write absolute value of -42           # 42
Write round 3.7                       # 4
Write round down 3.9                  # 3
Write round up 3.1                    # 4
```

#### String (3)
```pohlang
Write make uppercase "hello"          # HELLO
Write make lowercase "WORLD"          # world
Write trim spaces from "  text  "     # text
```

#### Collection (10)
```pohlang
Set nums to [10, 20, 30, 40, 50]

Write first in nums                   # 10
Write last in nums                    # 50
Write reverse of nums                 # [50, 40, 30, 20, 10]
Write count of nums                   # 5

Set text to join nums with ", "       # "10, 20, 30, 40, 50"
Set parts to split "a,b,c" by ","     # ["a", "b", "c"]

Write contains 30 in nums             # True
Set removed to remove 30 from nums    # [10, 20, 40, 50]
Set added to append 60 to nums        # [10, 20, 30, 40, 50, 60]
Set inserted to insert 25 at 2 in nums # [10, 20, 25, 30, 40, 50]
```

## Command Line Options

```powershell
# Run a program
.\pohlang.exe --run script.poh

# Compile to bytecode
.\pohlang.exe --compile script.poh

# Show version
.\pohlang.exe --version

# Show help
.\pohlang.exe --help
```

## Documentation

- **Language Guide**: `doc\PohLang_Guide.md` - Complete tutorial
- **Vocabulary**: `spec\Vocabulary.md` - All keywords and phrases
- **Grammar**: `doc\GRAMMAR_ANALYSIS.md` - Formal specification
- **Contributing**: `CONTRIBUTING.md` - How to contribute

## Examples Directory

```
examples/poh/
├── hello.poh                    # Basic hello world
├── arithmetic.poh               # Basic math operations
├── math_functions.poh           # Phrasal math operations
├── string_functions.poh         # String manipulation
├── collection_functions.poh     # List operations
├── collections_phrasal.poh      # Phrasal collection ops
├── if_block_greeting.poh        # Conditionals
├── phrase_repeat.poh            # Loops
├── phrase_function.poh          # Functions
└── ...and more!
```

## Common Patterns

### Input from User
```pohlang
Ask for name
Write "Hello " plus name
```

### List Processing
```pohlang
Set numbers to [5, 10, 15, 20]
Set doubled to []

Repeat for each num in numbers
    Set doubled to append (num times 2) to doubled
End Repeat

Write doubled  # [10, 20, 30, 40]
```

### Error Handling
```pohlang
# PohLang provides helpful error messages
Set nums to [1, 2, 3]
Write nums[10]  # Error: Index 10 out of bounds for list of length 3
```

## Tips & Tricks

1. **Comments**: Use `#` for single-line comments
   ```pohlang
   # This is a comment
   Set x to 42  # Comments work here too
   ```

2. **Precedence**: Multiplication before addition (like math!)
   ```pohlang
   Set result to 5 plus 3 times 2  # 11, not 16
   ```

3. **List Syntax**: Use brackets or "Make a list of"
   ```pohlang
   Set nums1 to [1, 2, 3]
   Set nums2 to Make a list of 1, 2 and 3
   ```

4. **String Concatenation**: Use "plus"
   ```pohlang
   Set greeting to "Hello" plus " " plus "World"
   ```

## Troubleshooting

### Problem: "Command not found"
**Solution**: Make sure you're in the release directory or use full path:
```powershell
C:\path\to\release-v0.5.0\pohlang.exe --run hello.poh
```

### Problem: File not found error
**Solution**: Check the file path is correct:
```powershell
# Use relative path from current directory
.\pohlang.exe --run .\examples\poh\hello.poh
```

### Problem: Syntax error
**Solution**: 
- Check you have `Start Program` at the beginning
- Check you have `End Program` at the end
- Verify all blocks have proper `End` statements

## System Requirements

- **Windows**: Windows 10 or later (64-bit)
- **Memory**: 50 MB RAM minimum
- **Disk Space**: 5 MB

## Getting Help

- **Documentation**: Read `doc\PohLang_Guide.md`
- **Examples**: Check the `examples\poh\` folder
- **Issues**: https://github.com/AlhaqGH/PohLang/issues
- **Discussions**: https://github.com/AlhaqGH/PohLang/discussions

## What's Next?

1. **Learn the Basics**: Read `doc\PohLang_Guide.md`
2. **Try Examples**: Run all programs in `examples\poh\`
3. **Write Your Own**: Create your first program!
4. **Join Community**: Participate in GitHub Discussions
5. **Contribute**: See `CONTRIBUTING.md`

## Phase 2 Preview

Coming soon in Phase 2:
- Standard library modules (collections, random, datetime, file I/O)
- Module system with imports
- Enhanced error messages
- More phrasal expressions
- Performance improvements

## License

PohLang is licensed under the MIT License. See `LICENSE` file for details.

Free for educational and commercial use!

## About PohLang

PohLang is a natural-language programming system designed to make programming accessible through English-like syntax. Perfect for:

- **Beginners**: Learn programming concepts without complex syntax
- **Education**: Teach programming logic in plain English
- **Prototyping**: Quickly express algorithmic ideas
- **Accessibility**: Programming for those who struggle with traditional syntax

## Version Info

- **Version**: 0.5.0
- **Release Date**: October 5, 2025
- **Status**: Phase 1 Complete ✅
- **Tests**: 50/50 passing (100% coverage)
- **License**: MIT

---

**Thank you for using PohLang!**

Making programming accessible through natural language.

Visit: https://github.com/AlhaqGH/PohLang
