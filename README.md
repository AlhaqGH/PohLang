# PohLang

A beginner-focused, fully phrasal (English-like) language that transpiles to Dart. There is no symbol-based mode: every program is written as readable commands.

## Goals
- Use plain-language statements: `Write "Hello"`, `Set count 5`, `Repeat count Write "Hi"`.
- Keep one clear form per concept (no synonyms like Print/Show). Use `Write`, not `Say/Print`.
- Provide a gentle path to programming concepts (variables, loops, conditionals, functions) without punctuation noise.
- Output readable Dart so learners can transition later.

## Example
```
Ask for name
Write "Hello " + name
Set count 3
Repeat count Write "Hi"
If count > 1 Write "Many" Otherwise Write "Few"
Make greet with who Write "Hi " + who
Use greet with "Poh"
```

## Running a Program
1. Ensure Dart SDK is installed.
2. Place a `.poh` file in the project root or `examples/`.
3. Transpile:
```
dart run src/transpiler.dart examples/phrase_repeat.poh
```
4. Run the generated Dart file (same basename with `.dart`).
```
dart run examples/phrase_repeat.dart
```

### CLI (bin/) usage

Once in this repo:

```
dart run bin/pohlang.dart examples/phrase_repeat.poh
```

Flags:
- `--no-run` — only transpile, don't execute the generated Dart
- `-o <file.dart>` — write output to a custom path

You can also run with tasks in VS Code: use the Command Palette → Run Task → "Transpile + Run (invoice example)".

### Package & publish

Current version: `0.0.2`.
To dry-run a publish:

```
dart pub publish --dry-run
```

To run analyzer and format:

```
dart analyze
dart format .
```

### Install from pub.dev

Once published, users can install globally:

```
dart pub global activate pohlang
```

Run a program:

```
pohlang path/to/program.poh
```

Notes:
- You can add comments with `#`. Everything after `#` on a line is ignored.
- Parser errors include line numbers, for example: `Line 2: After 'Otherwise' you must add 'Write <expression>'`.

## Current Features (v0.1)
- Write, Ask for, Set, Increase, Decrease
- Inline If with Otherwise (single-line)
- Inline Repeat loop
- Inline function definition + call (Make/Use)
- Expression support: identifiers, numbers, strings, `+` and `-`
- Desugaring for increase/decrease

## Roadmap
- Multi-line blocks with indentation (If / Repeat / Define)
- Functions emitted as real Dart functions (current inline stored only)
- Lists, dictionaries, predicates (is even, is greater than)
- Extended comparisons using verbal forms (`is greater than`)
- Random numbers, timers, date utilities
- Error diagnostics with suggestions & line numbers
- Optional extensions: Islamic-friendly helpers (prayer times, Hijri date, Quran recitation)
- Flutter integration examples

## Directory Structure
```
src/       Core transpiler sources
example/   Sample PohLang programs
doc/       Vocabulary & syntax references
lib/       Future runtime helpers / extensions
```

## Contributing
Open to experiments—keep syntax consistent and simple. Avoid adding multiple ways to express the same idea unless it aids learning progression.

## License
MIT © 2025 Habiburrahman Mukhlis — see LICENSE for details.
