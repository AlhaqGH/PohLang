# Changelog

All notable changes to PohLang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-09-21 - First Experimental Release

### Overview
This is the first experimental release of PohLang, marking the initial public availability of this beginner-focused, fully phrasal programming language.

**Release Components:**
- **PohLang Core**: v0.1.0 (First Experimental Release)
- **Python Interpreter**: v0.5.0 (Stable)
- **Dart Transpiler**: v0.3.5 (Fourth Experimental Release)

### Added

#### Core Language Features
- **Phrasal Syntax**: Complete natural English-like programming
  - `Write "Hello World"` - Output statements
  - `Ask for name` - Input statements
  - `Set x to 5` - Variable assignment
  - `If x is greater than 3` - Conditional statements
  - `Repeat 5` and `While x is less than 10` - Loop constructs
  - `Make function with parameters` - Function definitions
  - Natural operators: `plus`, `minus`, `times`, `divided by`
  - Comparison operators: `is greater than`, `is less than`, `is equal to`

#### Python Interpreter (v0.5.0)
- Complete tree-walking interpreter implementation
- Robust expression evaluation system
- Proper variable scoping and environment management
- Function definitions with parameter binding
- Control flow: if/otherwise blocks, repeat loops, while loops
- Module import system with cycle detection
- Debug tracing with detailed execution information
- Comprehensive error reporting with line/column positions
- Built-in functions and standard operations
- File I/O and system interaction capabilities
- Collection support (lists, basic data structures)

#### Dart Transpiler (v0.3.5)
- Direct PohLang-to-Dart code generation
- Function and variable name translation
- Control flow structure mapping
- Expression and operator translation
- Shared runtime library for transpiled execution
- Command-line transpilation interface
- Generated code readability for learning transitions

#### Development Environment
- Command-line tools for program execution
- Comprehensive example programs
- Educational tutorials and guides
- Extensive test suite coverage
- Development documentation

### Language Example

```pohlang
# Welcome program demonstrating core features
Write "Welcome to PohLang v0.1.0!"

# Variables and expressions
Ask for name
Set greeting to "Hello " plus name plus "!"
Write greeting

# Control flow
Ask for age
If age is greater than 17
    Write "You are an adult"
Otherwise
    Write "You are a minor"
End

# Loops
Set counter to 1
Write "Counting to 3:"
Repeat 3
    Write "Count: " plus counter
    Set counter to counter plus 1
End

# Functions
Make calculate_square with number
    Set result to number times number
    Return result
End

Set my_number to 5
Set squared to calculate_square(my_number)
Write my_number plus " squared is " plus squared
```

### Installation & Usage

#### Installation
```bash
# Install from PyPI
pip install pohlang

# Or install from source
git clone https://github.com/AlhaqGH/PohLang
cd PohLang
pip install -e .
```

#### Running Programs
```bash
# Using Python interpreter (recommended)
pohlang my_program.poh

# Using Dart transpiler (requires Dart SDK)
dart run bin/pohlang.dart my_program.poh
```

### Documentation
- **[PohLang Guide](PohLang_Guide.md)** - Complete language tutorial
- **[Syntax Documentation](doc/syntax.md)** - Detailed syntax reference
- **[Vocabulary Reference](doc/vocabulary.md)** - All language keywords

### Technical Improvements
- Feat: Unified error formatting with capitalized `Line`/`Col` markers
- Feat: Consistent error prefix format: `[file: Line N: Col M]`
- Docs: Updated all documentation to reflect new error format
- Test: Comprehensive test suite for interpreter and language features

### Known Limitations
- Dart transpiler requires separate Dart SDK installation
- Limited standard library (expanding in future releases)
- Error messages could be more educational/beginner-friendly
- No IDE integrations yet (planned for upcoming releases)
- Performance optimized for learning, not production use

### Development Notes
This release establishes the foundation for PohLang as an educational programming language. The focus has been on creating a clean, understandable syntax that allows beginners to focus on programming concepts rather than syntax complexity.

### Future Roadmap (v0.2.0 and beyond)
- Enhanced error diagnostics with suggestions
- IDE/editor plugins (VS Code, Sublime Text)
- Expanded standard library with educational modules
- Web-based interactive playground
- More comprehensive educational resources
- Performance optimizations
- Additional transpilation targets

### Contributing
- **GitHub Repository**: https://github.com/AlhaqGH/PohLang
- **Issues & Feature Requests**: GitHub Issues
- **Documentation**: Contributions welcome
- **Educational Feedback**: Highly valued from educators and learners

### License
MIT License - Open source and free for educational use

---

## Earlier Development History
- Runtime error system development and unification
- Parser improvements with positional metadata
- Expression system implementation
- Control flow and function system development
- Import system with cycle detection
- Debug tracing implementation

Note: See `transpiler/CHANGELOG.md` for detailed Dart transpiler development history.