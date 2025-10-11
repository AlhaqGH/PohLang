# Changelog

All notable changes to PohLang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2025-10-11 - Web Framework & Hot Reload (Phase 6)

### Overview
Major feature release adding complete web framework with HTTP server, route handling, and Flutter-style hot reload. This release introduces web servers, HTML/JSON responses, live file watching, and automatic browser reload.

### Added
- **Web Server Infrastructure**
  - `Create web server on port <port>` - Initialize HTTP server
  - `Add route "<path>" with method "<method>" to server:` - Define routes with handlers
  - `Start server` - Start server and listen for requests
  - Handler execution with isolated VM instances per request
  - Support for GET, POST, PUT, DELETE HTTP methods

- **HTTP Response Types**
  - `Write html response with <html>` - HTML responses with auto-injection
  - `Write json response with <json>` - JSON responses with proper headers
  - Automatic Content-Type headers
  - Custom status codes and headers support

- **Hot Reload System (Flutter-style)**
  - `--watch` CLI flag - Enable hot reload mode
  - `LiveReloadTracker` - File modification tracking
  - `/__reload_check` endpoint - Automatic polling endpoint
  - Auto-injection of livereload JavaScript into HTML
  - Sub-500ms reload time (2x per second polling)
  - No native dependencies (pure Rust polling)

- **Developer Experience**
  - Automatic livereload script injection before `</body>`
  - Browser console logging: `[LiveReload] Monitoring for changes...`
  - Friendly startup messages with emojis
  - Watch mode prints watched directories

### Changed
- HTTP server binds to `0.0.0.0` instead of `127.0.0.1` (Windows compatibility)
- Added `Value::LiveReloadTracker` and `Value::WebServer` to VM value system
- Added `vm.enable_hot_reload(paths)` public API

### Fixed
- Server blocking issue - now uses lock-free `start_server_from_arc()`
- Handler execution - each request gets isolated VM with cloned globals

### Examples
- `examples/poh/web_hello.poh` - Simple web server with hot reload
- `examples/TaskMaster/backend.poh` - Task management backend
- `examples/TaskMaster/public/*` - Complete frontend (HTML/CSS/JS)

### Performance
- Hot reload detection: <500ms
- Request handling: Multi-threaded with `thread::spawn`
- File polling: 500ms interval (2x per second)

## [0.5.4] - 2025-10-10 - Error Handling System (Phase 5)

### Overview
Major feature release adding comprehensive error handling with natural English syntax and messages. This release introduces try/catch/finally blocks, custom error types, type-specific error catching, and error location reporting.

### Added
- **Error Handling Infrastructure**
  - `PohError` type with 7 built-in error kinds: `RuntimeError`, `TypeError`, `MathError`, `FileError`, `JsonError`, `NetworkError`, `ValidationError`
  - Custom error types support with preserved casing
  - Error message and type extraction operations
  - Stack frame tracking for future enhancements

- **Try/Catch/Finally Syntax** (Natural English)
  - `try this:` / `end try` blocks
  - `if error` - catch all errors
  - `if error as variable` - catch with variable binding
  - `if error of type "ErrorType" as variable` - type-specific catching
  - `finally:` - always-executed cleanup blocks
  - Multiple catch handlers with first-match semantics

- **Error Creation and Throwing**
  - Create errors: `Set err to error of type "ValidationError" with message "Invalid input"`
  - Throw errors: `throw err` or `throw "Error message"`
  - Extract info: `error message of err`, `error type of err`

- **Natural Language Error Messages**
  - Consistent natural English format: `Error occurred: a file error - message`
  - Type marker system for accurate matching while showing natural text
  - Custom error names preserved (e.g., "DatabaseError occurred: message")

- **Error Location Reporting**
  - Filename tracking: Errors show `in file: filename.poh`
  - VM infrastructure for location context
  - Updated division by zero and throw statements with location info

### Changed
- Error messages converted from technical format to natural English
- Added `Value::Error` variant to value system
- Enhanced VM with error context tracking

### Examples
- `error_handling_demo.poh` - 5 comprehensive test scenarios
- `natural_errors.poh` - Natural language format validation
- `comprehensive_error_demo.poh` - Complete feature demonstration
- `test_error_location.poh`, `test_throw_location.poh` - Location reporting tests

### Documentation
- `PHASE_5_PLAN.md` - Complete design and architecture
- `PHASE_5_COMPLETE.md` - Implementation summary and examples
- `doc/ERROR_LOCATION_STATUS.md` - Technical details of location reporting

### Technical Details
- ~450 lines of new production code
- 10 unit tests in `stdlib/errors.rs`
- 11 new phrasal constants for error handling
- Complete parser support for error syntax
- VM execution with proper error propagation

### Known Limitations
- Line/column numbers not implemented (requires AST refactor, estimated 20+ hours)
- ~50 existing error sites could be updated to use location helper (can be done incrementally)

## [0.5.2] - 2025-10-05 - Symbolic Operators Support

### Overview
Feature release adding support for symbolic operators alongside existing phrasal operators. PohLang now supports both natural English-like phrasal syntax AND traditional symbolic operators, giving users flexibility in writing style.

### Added
- **Symbolic Arithmetic Operators**: Full support for traditional mathematical symbols
  - `+` (addition) works alongside `plus`
  - `-` (subtraction) works alongside `minus`
  - `*` (multiplication) works alongside `times`
  - `/` (division) works alongside `divided by`

- **Symbolic Comparison Operators**: Standard comparison symbols
  - `>` (greater than) works alongside `is greater than`
  - `<` (less than) works alongside `is less than`
  - `>=` (greater or equal) works alongside `is greater than or equal to`
  - `<=` (less or equal) works alongside `is less than or equal to`
  - `==` (equal) works alongside `is equal to`
  - `!=` (not equal) works alongside `is not equal to`

### Features
- **Both Forms Supported**: Phrasal and symbolic operators can be used interchangeably
- **Mixed Expressions**: Combine both styles in the same program
- **Proper Precedence**: Symbolic operators follow standard mathematical precedence
- **Backward Compatible**: All existing phrasal programs continue to work

### Examples

**Arithmetic - Both Forms:**
```pohlang
# Phrasal form (original style)
Set sum to a plus b
Set product to a times b

# Symbolic form (new style)
Set sum to a + b
Set product to a * b

# Mixed form (both in same program)
Set result to (a + b) times 2
```

**Comparisons - Both Forms:**
```pohlang
# Phrasal form (original style)
If age is greater than 18
    Write "Adult"
End If

# Symbolic form (new style)
If age > 18
    Write "Adult"
End If

# Mixed comparisons
If score >= 90
    Write "Excellent!"
Otherwise If score > 75
    Write "Good job!"
End If
```

### Testing
- ✅ All 50 tests passing (6 functions + 7 phrasals + 37 smoke tests)
- ✅ `test_symbolic_simple.poh` - Symbolic operators verified
- ✅ `test_both_forms.poh` - Both phrasal and symbolic forms working together
- ✅ No regressions from v0.5.1

### Technical Details
- Parser updated to recognize symbolic operators in expressions
- Operator precedence maintained: multiplication/division before addition/subtraction
- Comparison operators properly integrated with control flow statements
- Both operator forms compile to identical bytecode

### Migration Notes
- **No breaking changes** from v0.5.1
- All existing phrasal programs work without modification
- Symbolic operators are purely additive feature
- Choose your preferred style or mix both freely

### Design Philosophy
PohLang now offers **flexibility without compromising clarity**:
- **Beginners**: Use phrasal forms for maximum readability (`a plus b`)
- **Experienced programmers**: Use symbolic forms for familiarity (`a + b`)
- **Educational contexts**: Mix both to teach operator equivalence

---

## [0.5.1] - 2025-10-05 - Parser Fix for Multi-line If Statements

### Overview
Patch release fixing a critical parser issue with multi-line If statements that use phrasal comparison operators. This issue affected v0.5.0 and prevented proper parsing of natural language conditionals in block form.

### Fixed
- **Parser Issue**: Multi-line If statements with phrasal comparisons now work correctly
  - Added full support for " is greater than ", " is less than ", etc. in comparison expressions
  - Added " is greater than or equal to " and " is less than or equal to " patterns
  - Added " is equal to " and " is not equal to " explicit patterns
  - Fixed "End If" keyword recognition (previously only accepted "End")
  - Proper operator precedence: longer patterns matched first to avoid premature matching

**Examples that now work:**
```pohlang
# ✅ Works in v0.5.1 (failed in v0.5.0)
If temperature is greater than 20
    Write "It's warm!"
Otherwise
    Write "It's cool!"
End If

# ✅ Works in v0.5.1 (failed in v0.5.0)
If age is less than 18
    Write "Minor"
Otherwise
    Write "Adult"
End If

# ✅ All phrasal comparisons now supported in multi-line blocks:
# - is greater than
# - is less than
# - is greater than or equal to
# - is less than or equal to
# - is equal to
# - is not equal to
```

### Testing
- ✅ All 50 tests passing (6 functions + 7 phrasals + 37 smoke tests)
- ✅ `examples/poh/if_block_greeting.poh` - Now works
- ✅ `examples/poh/phrase_age_check.poh` - Now works
- ✅ All multi-line If variations tested and verified

### Notes
- **Backward Compatible**: All v0.5.0 programs that worked continue to work in v0.5.1
- **No Breaking Changes**: This is a pure bug fix release
- **Recommended Upgrade**: All v0.5.0 users should upgrade to v0.5.1

---

## [0.5.0] - 2025-10-05 - Phase 1 Complete: Production-Ready Release

### Overview
This release marks the **completion of Phase 1** of PohLang development. The language now features a mature, unambiguous grammar, comprehensive phrasal built-ins, and production-quality Rust runtime. All core features are stable and thoroughly tested.

**Milestone Achievement**: ✅ Phase 1 Complete
- **50 passing tests** (6 functions + 7 phrasals + 37 smoke tests)
- **20 phrasal built-in expressions** for natural programming
- **Mathematically proven unambiguous grammar**
- **Production-ready Rust runtime** with proper error handling

### Added

#### Phrasal Built-in Expressions (16 new expressions)
**Mathematical Operations:**
- `total of <list>` - Sum all numbers in a list
- `smallest in <list>` - Find minimum value
- `largest in <list>` - Find maximum value  
- `absolute value of <number>` - Get absolute value
- `round <number>` - Round to nearest integer
- `round down <number>` - Floor function
- `round up <number>` - Ceiling function

**String Operations:**
- `make uppercase <string>` - Convert to uppercase
- `make lowercase <string>` - Convert to lowercase
- `trim spaces from <string>` - Remove leading/trailing whitespace

**Collection Operations:**
- `first in <collection>` - Get first element of list/string
- `last in <collection>` - Get last element of list/string
- `reverse of <collection>` - Reverse list/string
- `count of <x>` - Size of list/string/dictionary
- `join <list> with <sep>` - Join items into text with separator
- `split <text> by <sep>` - Split text into list by separator
- `contains <item> in <collection>` - Check if item exists in list/string/dict
- `remove <item> from <list>` - Remove first occurrence from list
- `append <item> to <list>` - Add item to end of list
- `insert <item> at <index> in <list>` - Insert item at position

**Aliases for Readability:**
- `size of <x>` → `count of <x>`
- `separate <text> by <sep>` → `split <text> by <sep>`
- `reverse <collection>` → `reverse of <collection>`
- `clean spaces from <string>` → `trim spaces from <string>`

#### Grammar & Parser Improvements
- **Centralized phrase management**: Created `runtime/src/parser/phrases.rs` module
- **Consistent parsing**: All phrasal expressions use centralized constants
- **Proper operator precedence**: Verified unambiguous grammar
  - Multiplication/Division before Addition/Subtraction
  - Comparisons before logical AND
  - Logical AND before logical OR
  - Left-to-right associativity for same-precedence operators
- **Comprehensive grammar analysis**: New `doc/GRAMMAR_ANALYSIS.md` documentation

#### Documentation
- **Grammar Analysis**: Complete consistency and ambiguity analysis
- **CONTRIBUTING.md**: Developer guide for adding phrasal built-ins
- **Updated Vocabulary**: All new phrasal expressions documented
- **Updated Guide**: Examples for all new features
- **CI/CD**: GitHub Actions workflow for automated testing

#### Developer Experience
- **Modular architecture**: Phrases module for easy extension
- **Comprehensive tests**: 50 tests covering all features
- **Error handling**: Proper error messages for all built-ins
- **Type safety**: Value equality comparisons for contains/remove

### Changed
- Refactored parser to use centralized phrase constants (eliminates hardcoded strings)
- Improved test organization with dedicated `phrasals.rs` test suite
- Enhanced built-in function error messages
- Updated all documentation to reflect Phase 1 completion

### Fixed
- Proper handling of negative indices in `insert at` operation
- Consistent case-insensitive phrasal parsing across all expressions
- Value equality comparisons for collection operations

### Technical Details

**Test Coverage:**
```
✅ 6 function tests (closures, parameters, calls, errors)
✅ 7 phrasal tests (count, join, split, contains, remove, append, insert)
✅ 37 smoke tests (all core features, precedence, collections, control flow)
```

**Grammar Specification:**
- Formal precedence hierarchy documented in EBNF
- Recursive descent parser with proper operator precedence
- No ambiguities: Every valid program has exactly one parse tree
- Explicit block terminators eliminate dangling-else problems

**Performance:**
- Clean compilation in ~20 seconds
- Test suite runs in <2 seconds
- Efficient expression evaluation with proper precedence handling

### Language Example

```pohlang
Start Program

# Collection operations
Set numbers to [1, 2, 3, 4, 5]
Write "Numbers: " plus numbers
Write "Total: " plus total of numbers
Write "Largest: " plus largest in numbers

# String manipulation
Set text to "  Hello World  "
Set clean to trim spaces from text
Set upper to make uppercase clean
Write upper

# Checking membership
If contains 3 in numbers
    Write "Found 3!"
End If

# List manipulation
Set extended to append 6 to numbers
Set reduced to remove 3 from extended
Write "Modified: " plus reduced

# Advanced operations
Set words to split "apple,banana,cherry" by ","
Write "Words: " plus join words with " | "

End Program
```

### Migration Notes
- **No breaking changes** from v0.1.0
- All existing programs continue to work
- New phrasal expressions are additive features
- Aliases provide backward compatibility

### Performance Benchmarks
- Parser: ~1ms for typical programs
- Evaluation: Optimized for educational use (readability over speed)
- Memory: Efficient with proper garbage collection patterns

### Known Limitations
- Comments must be on separate lines (not end-of-line)
- "Make a list of" syntax has minor parsing quirks (use `[...]` for now)
- IDE integrations still in development (Phase 2)

### Future Roadmap (Phase 2)
- Standard library modules (collections, random, datetime, math, file, islamic)
- Module system with namespaces (`module::function`)
- Enhanced import syntax (`Import system "module" as alias`)
- Web-based playground
- VS Code extension with syntax highlighting
- Additional phrasal expressions based on community feedback

### Contributing
- **Repository**: https://github.com/AlhaqGH/PohLang
- **Issues**: Bug reports and feature requests welcome
- **Pull Requests**: See CONTRIBUTING.md for developer guide
- **Documentation**: Community contributions appreciated

### Credits
Special thanks to the PohLang community for feedback and testing during Phase 1 development.

### License
MIT License - Free for educational and commercial use

---

## [0.1.0] - 2025-09-21 - First Experimental Release

### Overview
This is the first experimental release of PohLang, marking the initial public availability of this beginner-focused, fully phrasal programming language.

**Release Components:**
- **PohLang Core**: v0.1.0 (First Experimental Release)
- **Python Interpreter**: v0.5.0 (Stable)

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

#### Rust Runtime (experimental)
- Standalone Rust VM (pohlang)
- CLI: run `.poh` scripts natively
- Early implementation with basic parsing and execution

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
- Additional execution/runtime targets

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

Note: Earlier experimental transpiler efforts have been retired.