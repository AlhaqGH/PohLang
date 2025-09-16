# Changelog

## 0.0.2 - 2025-09-16
- Fix: Generated Dart now imports runtime from the installed package (works from any folder)
- Move runtime to `lib/runtime.dart` and resolve via package URI with fallback for dev

## 0.0.1 - 2025-09-16
- Initial release
- CLI: `pohlang` to transpile and run `.poh` files
- Emits Dart next to input and auto-runs
- String+number concatenation handling
- Examples and documentation included
