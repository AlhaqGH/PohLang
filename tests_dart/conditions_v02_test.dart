// Moved from transpiler/test/conditions_v02_test.dart
import 'package:test/test.dart';
import '../transpiler/src/parser.dart';
import '../transpiler/src/ast.dart';

void main() {
	group('v0.2 conditions', () {
		test('is / is not', () {
			final p = Parser([
				'If x is 5 Write "ok" Otherwise Write "no"',
				'If x is not 5 Write "ok" Otherwise Write "no"',
			]).parse();
			final if1 = p.program.statements[0] as IfStmt;
			expect((if1.condition as BinaryExpr).op, '==');
			final if2 = p.program.statements[1] as IfStmt;
			expect((if2.condition as BinaryExpr).op, '!=');
		});

		test('is at least / at most', () {
			final p = Parser([
				'If x is at least 10 Write "ok" Otherwise Write "no"',
				'If x is at most 3 Write "ok" Otherwise Write "no"',
			]).parse();
			final if1 = p.program.statements[0] as IfStmt;
			expect((if1.condition as BinaryExpr).op, ">=");
			final if2 = p.program.statements[1] as IfStmt;
			expect((if2.condition as BinaryExpr).op, "<=");
		});

		test('lowercase connectors and Not', () {
			final p = Parser([
				'If not hasPaid and age is greater than 18 Write "ok" Otherwise Write "no"',
			]).parse();
			final if1 = p.program.statements[0] as IfStmt;
			final and = if1.condition as BinaryExpr; // (!hasPaid) && (age > 18)
			expect(and.op, '&&');
			final left = and.left as UnaryExpr;
			expect(left.op, '!');
			final right = and.right as BinaryExpr;
			expect(right.op, '>');
		});
	});
}
