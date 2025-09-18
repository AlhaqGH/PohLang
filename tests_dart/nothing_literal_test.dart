// Moved from transpiler/test/nothing_literal_test.dart
import 'package:test/test.dart';
import '../transpiler/src/parser.dart';
import '../transpiler/src/ast.dart';

void main() {
	test('nothing literal parses to null literal', () {
		final p = Parser(['Set x to nothing']).parse();
		final a = p.program.statements.first as AssignStmt;
		final lit = a.value as LiteralExpr;
		expect(lit.value, isNull);
	});
}
