import 'ast.dart';

// Lightweight line-based parser for phrasal PohLang v0.1 (symbol mode removed).

class ParseResult {
  final Program program;
  ParseResult(this.program);
}

class Parser {
  final List<String> lines;
  Parser(this.lines);

  ParseResult parse() {
    final filtered = <String>[];
    for (var l in lines) {
      final trimmed = l.trim();
      if (trimmed.isEmpty) continue;
      if (trimmed == 'Start Program' || trimmed == 'End Program') continue;
      filtered.add(trimmed);
    }
    final stmts = <Statement>[];
    var i = 0;
    while (i < filtered.length) {
      final line = filtered[i];
      if (line.startsWith('Repeat ')) {
        // Inline: Repeat N [times] Write X
        if (line.contains(' Say ')) {
          throw FormatException(
              'In \'Repeat\', use \'Write\', not \'Say\'. Example: Repeat 3 Write "Hi"');
        }
        int splitIdx = line.indexOf(' Write ');
        var tokenLen = ' Write '.length;
        var countStr = '';
        if (splitIdx != -1) {
          countStr = line.substring(7, splitIdx).trim();
          if (countStr.endsWith(' times')) {
            countStr =
                countStr.substring(0, countStr.length - ' times'.length).trim();
          }
          final bodyExprStr = line.substring(splitIdx + tokenLen).trim();
          final countExpr = _parseExpression(countStr);
          final bodyExpr = _parseExpression(bodyExprStr);
          stmts.add(RepeatStmt(countExpr, [PrintStmt(bodyExpr)]));
          i++;
          continue;
        }
        // Block: Repeat N [times] ... End
        var header = line.substring(7).trim();
        if (header.endsWith(' times')) {
          header = header.substring(0, header.length - ' times'.length).trim();
        }
        final countExpr = _parseExpression(header);
        final body = <Statement>[];
        i++;
        while (i < filtered.length && filtered[i] != 'End') {
          body.add(_parsePhraseLine(filtered[i]));
          i++;
        }
        if (i >= filtered.length || filtered[i] != 'End') {
          throw FormatException('Repeat block missing End');
        }
        // consume End
        i++;
        stmts.add(RepeatStmt(countExpr, body));
        continue;
      }
      // default single-line statement
      stmts.add(_parsePhraseLine(line));
      i++;
    }
    final desugared = _desugar(stmts);
    return ParseResult(Program(desugared));
  }

  Statement _parsePhraseLine(String line) {
    // Fast checks for forbidden synonyms inside the line
    if (line.contains(' Else ')) {
      throw FormatException(
          "Use 'Otherwise' instead of 'Else'. Example: If cond Write X Otherwise Write Y");
    }
    if (line.startsWith('Print ')) {
      throw FormatException(
          'Use \"Write\" instead of \"Print\". Example: Write "Hello"');
    }
    if (line.startsWith('Ask the user for ')) {
      throw FormatException(
          "Use 'Ask for <name>' instead of 'Ask the user for'. Example: Ask for name");
    }
    if (line.startsWith('Sya ') || line == 'Sya') {
      throw FormatException('Did you mean "Write"? Example: Write "Hi"');
    }
    if (line.startsWith('Say ')) {
      throw FormatException(
          'Say is not supported. Use "Write <expression>". Example: Write "Hello"');
    }

    // Write/Print
    if (line.startsWith('Write ') && !line.startsWith('Write file ')) {
      final expr = _parseExpression(line.substring('Write '.length));
      return PrintStmt(expr);
    }

    // Ask for
    if (line.startsWith('Ask for ')) {
      final name = line.substring(8).trim();
      return InputStmt(name);
    }

    // Set
    if (line.startsWith('Set ')) {
      // Supports: Set x to 5  | Set x 5
      final afterSet = line.substring(4).trim();
      final toIdx = afterSet.indexOf(' to ');
      late String name;
      late String valueStr;
      if (toIdx != -1) {
        name = afterSet.substring(0, toIdx).trim();
        valueStr = afterSet.substring(toIdx + 4).trim();
      } else {
        final firstSpace = afterSet.indexOf(' ');
        if (firstSpace == -1)
          throw FormatException('Invalid Set syntax: $line');
        name = afterSet.substring(0, firstSpace);
        valueStr = afterSet.substring(firstSpace + 1).trim();
      }
      return AssignStmt(name, _parseExpression(valueStr));
    }

    // Increase/Decrease (friendly)
    if (line.startsWith('Increase ')) {
      final after = line.substring('Increase '.length).trim();
      final byIdx = after.indexOf(' by ');
      final name = byIdx == -1 ? after : after.substring(0, byIdx).trim();
      final incExpr = byIdx == -1
          ? LiteralExpr(1)
          : _parseExpression(after.substring(byIdx + 4).trim());
      return AssignStmt(name, BinaryExpr('+', IdentifierExpr(name), incExpr));
    }
    if (line.startsWith('Decrease ')) {
      final after = line.substring('Decrease '.length).trim();
      final byIdx = after.indexOf(' by ');
      final name = byIdx == -1 ? after : after.substring(0, byIdx).trim();
      final decExpr = byIdx == -1
          ? LiteralExpr(1)
          : _parseExpression(after.substring(byIdx + 4).trim());
      return AssignStmt(name, BinaryExpr('-', IdentifierExpr(name), decExpr));
    }
    // Old terms with suggestions
    if (line.startsWith('Increment ')) {
      throw FormatException(
          "Use 'Increase <name> by <number>' (make number bigger). Example: Increase x by 1");
    }
    if (line.startsWith('Decrement ')) {
      throw FormatException(
          "Use 'Decrease <name> by <number>' (make number smaller). Example: Decrease x by 1");
    }

    // If
    if (line.startsWith('If ')) {
      if (line.contains(' Say ')) {
        throw FormatException(
            "In 'If', use 'Write', not 'Say'. Example: If cond Write X Otherwise Write Y");
      }
      final writeIdx = line.indexOf(' Write ', 3);
      if (writeIdx == -1) {
        throw FormatException(
            "Missing 'Write' in If. Example: If cond Write X Otherwise Write Y");
      }
      final elseIdx =
          line.indexOf(' Otherwise Write ', writeIdx + ' Write '.length);
      if (elseIdx != -1) {
        final condStr = line.substring(3, writeIdx).trim();
        final thenExprStr =
            line.substring(writeIdx + ' Write '.length, elseIdx).trim();
        final elseExprStr =
            line.substring(elseIdx + ' Otherwise Write '.length).trim();
        final cond = _parseBooleanExpression(condStr);
        return IfStmt(cond, [PrintStmt(_parseExpression(thenExprStr))],
            [PrintStmt(_parseExpression(elseExprStr))]);
      } else {
        final condStr = line.substring(3, writeIdx).trim();
        final thenExprStr = line.substring(writeIdx + ' Write '.length).trim();
        final cond = _parseBooleanExpression(condStr);
        return IfStmt(cond, [PrintStmt(_parseExpression(thenExprStr))]);
      }
    }

    // While (single-line)
    if (line.startsWith('While ')) {
      if (line.contains(' Say ')) {
        throw FormatException(
            "In 'While', use 'Write', not 'Say'. Example: While cond Write X");
      }
      final splitIdx = line.indexOf(' Write ');
      if (splitIdx == -1)
        throw FormatException(
            "Missing 'Write' in While. Example: While cond Write X");
      final condStr = line.substring(6, splitIdx).trim();
      final bodyExprStr = line.substring(splitIdx + ' Write '.length).trim();
      final cond = _parseBooleanExpression(condStr);
      final bodyExpr = _parseExpression(bodyExprStr);
      return WhileStmt(cond, [PrintStmt(bodyExpr)]);
    }

    // System/OS phrase commands
    if (line.startsWith('Open file ')) {
      final pathStr = line.substring('Open file '.length).trim();
      return OpenFileStmt(_parseExpression(pathStr));
    }
    if (line.startsWith('Write file ')) {
      final withIdx = line.indexOf(' with ');
      if (withIdx == -1) {
        throw FormatException(
            'Invalid Write file syntax, expected: Write file "name" with <expr>');
      }
      final pathStr = line.substring('Write file '.length, withIdx).trim();
      final contentStr = line.substring(withIdx + 6).trim();
      return WriteFileStmt(
          _parseExpression(pathStr), _parseExpression(contentStr),
          append: false);
    }
    if (line.startsWith('Append file ')) {
      final withIdx = line.indexOf(' with ');
      if (withIdx == -1) {
        throw FormatException(
            'Invalid Append file syntax, expected: Append file "name" with <expr>');
      }
      final pathStr = line.substring('Append file '.length, withIdx).trim();
      final contentStr = line.substring(withIdx + 6).trim();
      return WriteFileStmt(
          _parseExpression(pathStr), _parseExpression(contentStr),
          append: true);
    }
    if (line.startsWith('Delete file ')) {
      final pathStr = line.substring('Delete file '.length).trim();
      return DeleteFileStmt(_parseExpression(pathStr));
    }
    if (line.startsWith('List files in ')) {
      final dirStr = line.substring('List files in '.length).trim();
      return ListFilesStmt(_parseExpression(dirStr));
    }
    if (line.startsWith('Change directory to ')) {
      final pathStr = line.substring('Change directory to '.length).trim();
      return ChangeDirectoryStmt(_parseExpression(pathStr));
    }
    if (line.startsWith('Create directory ')) {
      final pathStr = line.substring('Create directory '.length).trim();
      return CreateDirectoryStmt(_parseExpression(pathStr));
    }
    if (line.startsWith('Delete directory ')) {
      final pathStr = line.substring('Delete directory '.length).trim();
      return DeleteDirectoryStmt(_parseExpression(pathStr));
    }
    if (line.startsWith('Run program ')) {
      // Run program "cmd" [and wait|in background]
      final base = line.substring('Run program '.length).trim();
      String mode = 'plain';
      String cmdStr = base;
      if (base.endsWith(' and wait')) {
        mode = 'wait';
        cmdStr = base.substring(0, base.length - ' and wait'.length).trim();
      } else if (base.endsWith(' in background')) {
        mode = 'background';
        cmdStr =
            base.substring(0, base.length - ' in background'.length).trim();
      }
      return RunProgramStmt(_parseExpression(cmdStr), mode: mode);
    }

    // Make/Use (functions)
    if (line.startsWith('Make ')) {
      if (line.contains(' Say ')) {
        throw FormatException(
            "In 'Make', use 'Write', not 'Say'. Example: Make f with x Write x + 1");
      }
      final withIndex = line.indexOf(' with ');
      final writeIndex = line.indexOf(' Write ');
      if (withIndex == -1 || writeIndex == -1) {
        throw FormatException(
            "Invalid Make syntax. Example: Make greet with who Write \"Hi \" + who");
      }
      final name = line.substring(5, withIndex).trim();
      final paramsStr = line.substring(withIndex + 6, writeIndex).trim();
      final exprStr = line.substring(writeIndex + ' Write '.length).trim();
      final params = paramsStr.isEmpty
          ? <String>[]
          : paramsStr.split(',').map((s) => s.trim()).toList();
      final bodyExpr = _parseExpression(exprStr);
      return FunctionDefStmt(name, params, [ReturnStmt(bodyExpr)]);
    }
    if (line.startsWith('Use ')) {
      final withIndex = line.indexOf(' with ');
      if (withIndex == -1) {
        final name = line.substring(4).trim();
        return CallStmt(name, []);
      }
      final name = line.substring(4, withIndex).trim();
      final argsStr = line.substring(withIndex + 6).trim();
      final args = argsStr.isEmpty
          ? <Expression>[]
          : argsStr.split(',').map((s) => _parseExpression(s.trim())).toList();
      return CallStmt(name, args);
    }
    // Old function terms with suggestions
    if (line.startsWith('Define ')) {
      throw FormatException(
          "Use 'Make <name> with <params> Write <expr>' (create a function).");
    }
    if (line.startsWith('Call ')) {
      throw FormatException("Use 'Use <name> with <args>' (run a function).");
    }

    throw FormatException('Unknown phrase statement: $line');
  }

  List<Statement> _desugar(List<Statement> input) {
    return input; // future expansion
  }

  Expression _parseExpression(String src) {
    src = src.trim();
    return _parseAddSub(src);
  }

  // Boolean grammar with simple precedence:
  // Or (lowest), And, Not (highest), then comparisons, then +,-,*, primary
  Expression _parseBooleanExpression(String src) {
    final norm = _normalizeComparators(src.trim());
    return _parseOr(norm);
  }

  Expression _parseOr(String src) {
    // split by ' Or ' at top level
    final parts = _splitTopLevel(src, ' Or ');
    if (parts.length == 1) return _parseAnd(src);
    Expression expr = _parseAnd(parts[0]);
    for (var i = 1; i < parts.length; i++) {
      expr = BinaryExpr('||', expr, _parseAnd(parts[i]));
    }
    return expr;
  }

  Expression _parseAnd(String src) {
    final parts = _splitTopLevel(src, ' And ');
    if (parts.length == 1) return _parseNot(src);
    Expression expr = _parseNot(parts[0]);
    for (var i = 1; i < parts.length; i++) {
      expr = BinaryExpr('&&', expr, _parseNot(parts[i]));
    }
    return expr;
  }

  Expression _parseNot(String src) {
    final trimmed = src.trim();
    if (trimmed.startsWith('Not ')) {
      return UnaryExpr('!', _parseNot(trimmed.substring(4)));
    }
    return _parseComparison(trimmed);
  }

  Expression _parseComparison(String src) {
    // Look for symbolic operators (normalized earlier): >=, <=, ==, !=, >, <
    final ops = ['>=', '<=', '==', '!=', '>', '<'];
    for (final op in ops) {
      final idx = src.indexOf(op);
      if (idx != -1) {
        final left = _parseAddSub(src.substring(0, idx));
        final right = _parseAddSub(src.substring(idx + op.length));
        return BinaryExpr(op, left, right);
      }
    }
    // Single '=' used as equality (avoid '==')
    int eqIdx = -1;
    for (int i = 0; i < src.length; i++) {
      if (src[i] == '=') {
        final isDouble = (i + 1 < src.length && src[i + 1] == '=');
        if (!isDouble) {
          eqIdx = i;
          break;
        }
      }
    }
    if (eqIdx != -1) {
      return BinaryExpr('==', _parseAddSub(src.substring(0, eqIdx)),
          _parseAddSub(src.substring(eqIdx + 1)));
    }
    return _parseAddSub(src);
  }

  Expression _parseAddSub(String src) {
    // Left-to-right split for + and - (no precedence with *)
    // We'll first split by + at top level
    final plusParts = _splitTopLevel(src, '+');
    if (plusParts.length > 1) {
      Expression expr = _parseMul(plusParts[0]);
      for (var i = 1; i < plusParts.length; i++) {
        expr = BinaryExpr('+', expr, _parseMul(plusParts[i]));
      }
      return expr;
    }
    // Then by - (but ignore leading minus for negative numbers)
    final minusParts = _splitTopLevelForMinus(src);
    if (minusParts.length > 1) {
      Expression expr = _parseMul(minusParts[0]);
      for (var i = 1; i < minusParts.length; i++) {
        expr = BinaryExpr('-', expr, _parseMul(minusParts[i]));
      }
      return expr;
    }
    return _parseMul(src);
  }

  Expression _parseMul(String src) {
    final parts = _splitTopLevel(src, '*');
    if (parts.length > 1) {
      Expression expr = _parsePrimary(parts[0]);
      for (var i = 1; i < parts.length; i++) {
        expr = BinaryExpr('*', expr, _parsePrimary(parts[i]));
      }
      return expr;
    }
    return _parsePrimary(src);
  }

  Expression _parsePrimary(String src) {
    final s = src.trim();
    // Verb arithmetic forms
    if (s.startsWith('Add ')) {
      // Add a and b
      final andIdx = s.indexOf(' and ');
      if (andIdx == -1) throw FormatException('Invalid Add syntax');
      final a = s.substring(4, andIdx).trim();
      final b = s.substring(andIdx + 5).trim();
      return BinaryExpr('+', _parseExpression(a), _parseExpression(b));
    }
    if (s.startsWith('Subtract ')) {
      // Subtract a from b  => b - a
      final fromIdx = s.indexOf(' from ');
      if (fromIdx == -1) throw FormatException('Invalid Subtract syntax');
      final a = s.substring(9, fromIdx).trim();
      final b = s.substring(fromIdx + 6).trim();
      return BinaryExpr('-', _parseExpression(b), _parseExpression(a));
    }
    if (s.startsWith('Multiply ')) {
      // Multiply a and b
      final andIdx = s.indexOf(' and ');
      if (andIdx == -1) throw FormatException('Invalid Multiply syntax');
      final a = s.substring(9, andIdx).trim();
      final b = s.substring(andIdx + 5).trim();
      return BinaryExpr('*', _parseExpression(a), _parseExpression(b));
    }
    if (s.startsWith('Divide ')) {
      // Divide a by b
      final byIdx = s.indexOf(' by ');
      if (byIdx == -1) throw FormatException('Invalid Divide syntax');
      final a = s.substring(7, byIdx).trim();
      final b = s.substring(byIdx + 4).trim();
      return BinaryExpr('/', _parseExpression(a), _parseExpression(b));
    }
    // future: parentheses
    // literal number
    final numVal = num.tryParse(s);
    if (numVal != null) return LiteralExpr(numVal);
    // boolean literals
    if (s == 'true' || s == 'false') return LiteralExpr(s == 'true');
    // string
    if (s.startsWith('"') && s.endsWith('"')) {
      return LiteralExpr(s.substring(1, s.length - 1));
    }
    return IdentifierExpr(s);
  }

  // Split by a delimiter string at top level (no parentheses parsing yet)
  List<String> _splitTopLevel(String src, String delimiter) {
    final parts = <String>[];
    int start = 0;
    int i = 0;
    while (i <= src.length - delimiter.length) {
      if (src.substring(i, i + delimiter.length) == delimiter) {
        parts.add(src.substring(start, i).trim());
        i += delimiter.length;
        start = i;
      } else {
        i++;
      }
    }
    if (start == 0) return [src.trim()];
    parts.add(src.substring(start).trim());
    return parts;
  }

  // Special handling for minus to avoid treating leading '-' as split
  List<String> _splitTopLevelForMinus(String src) {
    final parts = <String>[];
    int start = 0;
    for (int i = 0; i < src.length; i++) {
      if (src[i] == '-' && i > 0) {
        parts.add(src.substring(start, i).trim());
        start = i + 1;
      }
    }
    if (start == 0) return [src.trim()];
    parts.add(src.substring(start).trim());
    return parts;
  }

  String _normalizeComparators(String src) {
    // Order matters: longest phrases first
    final replacements = <String, String>{
      ' Greater Or Equal ': ' >= ',
      ' Less Or Equal ': ' <= ',
      ' Not Equals ': ' != ',
      ' Equals ': ' == ',
      ' Greater Than ': ' > ',
      ' Less Than ': ' < ',
      // legacy forms
      ' is not equal to ': ' != ',
      ' is equal to ': ' == ',
      ' is greater than ': ' > ',
      ' is less than ': ' < ',
    };
    var s = src;
    // Apply all replacements; repeat once more to be safe for overlapping spaces
    for (final entry in replacements.entries) {
      s = s.replaceAll(entry.key, entry.value);
    }
    return s;
  }
}
