import '../src/transpiler.dart' as transpiler;

/// CLI entrypoint for PohLang transpiler.
/// Usage: dart run pohlang <file.poh>
void main(List<String> args) {
  transpiler.main(args);
}
