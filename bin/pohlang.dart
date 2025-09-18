import '../transpiler/src/transpiler.dart' as transpiler;

/// Root package executable entrypoint.
/// Allows running via: `dart run pohlang <file.poh>`
void main(List<String> args) => transpiler.main(args);