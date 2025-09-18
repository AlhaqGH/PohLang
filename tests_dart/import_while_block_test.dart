// Moved from transpiler/test/import_while_block_test.dart
import 'package:test/test.dart';
import '../transpiler/src/parser.dart';
import '../transpiler/src/ast.dart';

void main() {
	group('While block parsing', () {
		test('while-body-end', () {
			final src = [
				'While x < 3',
				'Write "tick"',
				'End',
			];
			final p = Parser(src).parse();
			final w = p.program.statements.first as WhileStmt;
			expect(w.body.length, 1);
			expect(w.body.first, isA<PrintStmt>());
		});
	});

	group('Import parsing', () {
		test('import path quoted', () {
			final p = Parser(['Import "lib/util.poh"']).parse();
			expect(p.program.statements.first, isA<ImportStmt>());
			expect((p.program.statements.first as ImportStmt).path, 'lib/util.poh');
		});

		test('import requires quotes', () {
			try {
				Parser(['Import lib/util.poh']).parse();
				fail('Expected FormatException');
			} catch (e) {
				expect(e.toString(), contains('Import expects a quoted path'));
			}
		});
	});
}
