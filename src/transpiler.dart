import 'dart:io';
import 'ast.dart';
import 'parser.dart';
import 'package:path/path.dart' as p;

void main(List<String> args) async {
  if (args.isEmpty) {
    stderr.writeln('Usage: dart run src/transpiler.dart <file.poh>');
    exit(64);
  }
  final file = File(args[0]);
  if (!await file.exists()) {
    stderr.writeln('File not found: ${args[0]}');
    exit(66);
  }
  final lines = await file.readAsLines();
  final parser = Parser(lines);
  late ParseResult result;
  try {
    result = parser.parse();
  } catch (e) {
    stderr.writeln('Parse error: $e');
    // stderr.writeln(st); // verbose option
    exit(65);
  }
  // Emit Dart alongside the input .poh file
  final inputAbs = p.normalize(p.absolute(args[0]));
  final inputDir = p.dirname(inputAbs);
  final inputBase = p.basenameWithoutExtension(inputAbs);
  final outPath = p.join(inputDir, '$inputBase.dart');
  final dartCode = _emitDart(result.program, outPath);
  await File(outPath).writeAsString(dartCode);
  stdout.writeln('Generated: $outPath');
  // Auto-run the generated program
  try {
    final proc = await Process.start('dart', ['run', outPath],
        mode: ProcessStartMode.inheritStdio);
    final code = await proc.exitCode;
    exit(code);
  } catch (e) {
    stderr.writeln('Failed to run generated program: $e');
    exit(70);
  }
}

String _emitDart(Program program, String outPath) {
  final b = StringBuffer();
  b.writeln("import 'dart:io';");
  // Compute relative import to runtime.dart from the output file directory
  final outDir = p.normalize(p.dirname(p.absolute(outPath)));
  final runtimeAbs =
      p.normalize(p.join(Directory.current.path, 'src', 'runtime.dart'));
  var runtimeRel = p.relative(runtimeAbs, from: outDir);
  runtimeRel = runtimeRel.replaceAll('\\', '/');
  b.writeln("import '$runtimeRel';");
  // Emit functions (from Make) before main
  final funcDefs = <FunctionDefStmt>[];
  for (final node in program.statements) {
    if (node is FunctionDefStmt) funcDefs.add(node);
  }
  for (final f in funcDefs) {
    b.writeln(_emitFunction(f));
  }
  b.writeln('Future<void> main() async {');
  final declared = <String>{};
  for (final node in program.statements) {
    final stmt = node as Statement;
    if (stmt is FunctionDefStmt) continue; // already emitted above
    final code = _emitStmt(stmt, declared);
    if (code.isNotEmpty) b.writeln(code);
  }
  b.writeln('}');
  return b.toString();
}

String _emitStmt(Statement s, Set<String> declared) {
  if (s is PrintStmt) {
    return '  print(${_emitExpr(s.expression)});';
  } else if (s is InputStmt) {
    final name = s.name;
    final decl = declared.contains(name) ? '' : 'var ';
    declared.add(name);
    return "  ${decl}${name} = PohRuntime.inputText('$name');";
  } else if (s is AssignStmt) {
    final decl = declared.contains(s.name) ? '' : 'var ';
    declared.add(s.name);
    return '  ${decl}${s.name} = ${_emitExpr(s.value)};';
  } else if (s is IfStmt) {
    final thenCode =
        s.thenBranch.map((Statement st) => _emitStmt(st, declared)).join('\n');
    final elseCode = (s.elseBranch != null)
        ? s.elseBranch!
            .map((Statement st) => _emitStmt(st, declared))
            .join('\n')
        : null;
    return '  if (${_emitExpr(s.condition)}) {\n$thenCode\n  }' +
        (elseCode != null ? ' else {\n$elseCode\n  }' : '');
  } else if (s is RepeatStmt) {
    final body =
        s.body.map((Statement st) => _emitStmt(st, declared)).join('\n');
    final countExpr = _emitExpr(s.count);
    final loopVar = '_i';
    return '  for (var $loopVar = 0; $loopVar < $countExpr; $loopVar++) {\n$body\n  }';
  } else if (s is WhileStmt) {
    final cond = _emitExpr(s.condition);
    final body =
        s.body.map((Statement st) => _emitStmt(st, declared)).join('\n');
    return '  while ($cond) {\n$body\n  }';
  } else if (s is FunctionDefStmt) {
    return '';
  } else if (s is CallStmt) {
    final args = s.args.map(_emitExpr).join(', ');
    return '  ${s.name}($args);';
  } else if (s is OpenFileStmt) {
    // Reads into a conventional variable name: fileContent
    const varName = 'fileContent';
    final decl = declared.contains(varName) ? '' : 'var ';
    declared.add(varName);
    return "  ${decl}${varName} = PohRuntime.readFile(${_emitExpr(s.path)});";
  } else if (s is WriteFileStmt) {
    final append = s.append ? 'true' : 'false';
    return "  PohRuntime.writeFile(${_emitExpr(s.path)}, ${_emitExpr(s.content)}, append: $append);";
  } else if (s is DeleteFileStmt) {
    return "  PohRuntime.deleteFile(${_emitExpr(s.path)});";
  } else if (s is ListFilesStmt) {
    const varName = 'fileList';
    final decl = declared.contains(varName) ? '' : 'var ';
    declared.add(varName);
    return "  ${decl}${varName} = PohRuntime.listFiles(${_emitExpr(s.directory)});";
  } else if (s is ChangeDirectoryStmt) {
    return "  PohRuntime.changeDirectory(${_emitExpr(s.path)});";
  } else if (s is CreateDirectoryStmt) {
    return "  PohRuntime.createDirectory(${_emitExpr(s.path)});";
  } else if (s is DeleteDirectoryStmt) {
    return "  PohRuntime.deleteDirectory(${_emitExpr(s.path)});";
  } else if (s is RunProgramStmt) {
    if (s.mode == 'wait') {
      return "  await PohRuntime.runProgramWait(${_emitExpr(s.command)});";
    } else if (s.mode == 'background') {
      return "  PohRuntime.runProgramBackground(${_emitExpr(s.command)});";
    } else {
      // plain: fire and forget without waiting
      return "  PohRuntime.runProgramBackground(${_emitExpr(s.command)});";
    }
  } else if (s is ReturnStmt) {
    return '  return ${s.value != null ? _emitExpr(s.value!) : ''};';
  }
  return '  // Unhandled statement ${s.runtimeType}';
}

String _emitFunction(FunctionDefStmt f) {
  // Inline functions are single-expression returns
  String body;
  if (f.body.isNotEmpty && f.body.first is ReturnStmt) {
    final ret = f.body.first as ReturnStmt;
    final value = ret.value != null ? _emitExpr(ret.value!) : '';
    body = 'return $value;';
  } else {
    body = 'return;';
  }
  final params = f.params.join(', ');
  return 'dynamic ${f.name}($params) { $body }';
}

String _emitExpr(Expression e) {
  if (e is LiteralExpr) {
    if (e.value is String) {
      return '"${e.value}"';
    }
    return e.value.toString();
  } else if (e is IdentifierExpr) {
    return e.name;
  } else if (e is BinaryExpr) {
    // If '+' is used and any side is or contains a string literal, coerce both to String
    if (e.op == '+') {
      final leftHasStr = _containsStringLiteral(e.left);
      final rightHasStr = _containsStringLiteral(e.right);
      if (leftHasStr || rightHasStr) {
        return '(${_emitForConcat(e.left)} + ${_emitForConcat(e.right)})';
      }
    }
    return '(${_emitExpr(e.left)} ${e.op} ${_emitExpr(e.right)})';
  } else if (e is UnaryExpr) {
    return '(${e.op}${_emitExpr(e.expr)})';
  } else if (e is CallExpr) {
    final args = e.args.map(_emitExpr).join(', ');
    return '${e.name}($args)';
  }
  return '/*expr*/';
}

// Detect whether an expression subtree contains any string literal.
bool _containsStringLiteral(Expression e) {
  if (e is LiteralExpr) return e.value is String;
  if (e is BinaryExpr)
    return _containsStringLiteral(e.left) || _containsStringLiteral(e.right);
  return false;
}

// Emit an expression ensured to be a String for concatenation context.
String _emitForConcat(Expression e) {
  if (e is LiteralExpr && e.value is String) {
    return '"${e.value}"';
  }
  // For everything else, rely on toString to avoid type errors.
  return '(${_emitExpr(e)}).toString()';
}
