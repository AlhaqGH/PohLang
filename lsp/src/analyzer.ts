import { Range, DocumentSymbol, SymbolKind } from 'vscode-languageserver/node';

export interface AnalysisError {
	range: Range;
	message: string;
}

export interface AnalysisWarning {
	range: Range;
	message: string;
}

export interface DocumentAnalysis {
	errors: AnalysisError[];
	warnings: AnalysisWarning[];
	symbols: DocumentSymbol[];
}

export interface Symbol {
	name: string;
	kind: string;
	documentation?: string;
	definitionRange?: Range;
}

// Keywords for validation
const keywords = [
	'Start Program', 'End Program', 'Write', 'Set', 'Make', 'End',
	'If', 'Else', 'While', 'For', 'Return', 'True', 'False',
	'Ask', 'Read', 'Break', 'Continue',
	'Plus', 'Minus', 'Times', 'Divided', 'Is', 'And', 'Or', 'Not',
	'To', 'Greater', 'Less', 'Than', 'With', 'Each', 'In'
];

/**
 * Analyze a PohLang document for errors, warnings, and symbols
 */
export function analyzeDocument(text: string): DocumentAnalysis {
	const errors: AnalysisError[] = [];
	const warnings: AnalysisWarning[] = [];
	const symbols: DocumentSymbol[] = [];

	const lines = text.split('\n');
	let hasStartProgram = false;
	let hasEndProgram = false;
	let blockStack: string[] = [];
	let currentFunction: { name: string, startLine: number } | null = null;

	lines.forEach((line, lineIndex) => {
		const trimmedLine = line.trim();

		// Check for Start Program
		if (trimmedLine === 'Start Program') {
			hasStartProgram = true;
			if (blockStack.length > 0) {
				errors.push({
					range: Range.create(lineIndex, 0, lineIndex, line.length),
					message: 'Start Program must be at the top level'
				});
			}
			blockStack.push('program');
		}

		// Check for End Program
		if (trimmedLine === 'End Program') {
			hasEndProgram = true;
			if (blockStack[blockStack.length - 1] !== 'program') {
				errors.push({
					range: Range.create(lineIndex, 0, lineIndex, line.length),
					message: 'End Program without matching Start Program'
				});
			} else {
				blockStack.pop();
			}
		}

		// Check for function definitions
		if (trimmedLine.startsWith('Make ') && trimmedLine.includes(':')) {
			const match = trimmedLine.match(/Make\s+(\w+)/);
			if (match) {
				const functionName = match[1];
				currentFunction = { name: functionName, startLine: lineIndex };
				blockStack.push('function');

				symbols.push(DocumentSymbol.create(
					functionName,
					undefined,
					SymbolKind.Function,
					Range.create(lineIndex, 0, lineIndex, line.length),
					Range.create(lineIndex, 0, lineIndex, line.length)
				));
			}
		}

		// Check for If statements
		if (trimmedLine.startsWith('If ')) {
			blockStack.push('if');
		}

		// Check for While loops
		if (trimmedLine.startsWith('While ')) {
			blockStack.push('while');
		}

		// Check for For loops
		if (trimmedLine.startsWith('For ')) {
			blockStack.push('for');
		}

		// Check for End statements
		if (trimmedLine === 'End') {
			if (blockStack.length === 0 || blockStack[blockStack.length - 1] === 'program') {
				errors.push({
					range: Range.create(lineIndex, 0, lineIndex, line.length),
					message: 'End without matching block statement'
				});
			} else {
				blockStack.pop();
			}
		}

		// Check for invalid function definitions (missing colon)
		if (trimmedLine.startsWith('Make ') && !trimmedLine.includes(':')) {
			errors.push({
				range: Range.create(lineIndex, 0, lineIndex, line.length),
				message: 'Invalid function definition. Use "Make functionName:" or "Make functionName with param:"'
			});
		}

		// Check for variable assignments
		if (trimmedLine.startsWith('Set ') && trimmedLine.includes(' to ')) {
			const match = trimmedLine.match(/Set\s+(\w+)\s+to/);
			if (match) {
				const varName = match[1];
				symbols.push(DocumentSymbol.create(
					varName,
					undefined,
					SymbolKind.Variable,
					Range.create(lineIndex, 0, lineIndex, line.length),
					Range.create(lineIndex, 0, lineIndex, line.length)
				));
			}
		}
	});

	// Check for required program structure
	if (!hasStartProgram) {
		warnings.push({
			range: Range.create(0, 0, 0, 0),
			message: 'Missing "Start Program" statement'
		});
	}

	if (!hasEndProgram) {
		warnings.push({
			range: Range.create(lines.length - 1, 0, lines.length - 1, 0),
			message: 'Missing "End Program" statement'
		});
	}

	// Check for unclosed blocks
	if (blockStack.length > 0) {
		warnings.push({
			range: Range.create(lines.length - 1, 0, lines.length - 1, 0),
			message: `Unclosed block: ${blockStack[blockStack.length - 1]}`
		});
	}

	return { errors, warnings, symbols };
}

/**
 * Find the symbol at a given position in the text
 */
export function findSymbolAtPosition(text: string, offset: number): Symbol | null {
	const lines = text.split('\n');
	let currentOffset = 0;
	let targetLine = 0;
	let targetColumn = 0;

	// Find the line and column for the offset
	for (let i = 0; i < lines.length; i++) {
		if (currentOffset + lines[i].length >= offset) {
			targetLine = i;
			targetColumn = offset - currentOffset;
			break;
		}
		currentOffset += lines[i].length + 1; // +1 for newline
	}

	const line = lines[targetLine];
	const trimmedLine = line.trim();

	// Check if on a function definition
	if (trimmedLine.startsWith('Make ')) {
		const match = trimmedLine.match(/Make\s+(\w+)/);
		if (match) {
			return {
				name: match[1],
				kind: 'function',
				documentation: 'User-defined function',
				definitionRange: Range.create(targetLine, 0, targetLine, line.length)
			};
		}
	}

	// Check if on a variable
	if (trimmedLine.startsWith('Set ')) {
		const match = trimmedLine.match(/Set\s+(\w+)/);
		if (match) {
			return {
				name: match[1],
				kind: 'variable',
				documentation: 'Variable',
				definitionRange: Range.create(targetLine, 0, targetLine, line.length)
			};
		}
	}

	// Check if on a keyword
	for (const keyword of keywords) {
		if (trimmedLine.includes(keyword)) {
			return {
				name: keyword,
				kind: 'keyword',
				documentation: `PohLang keyword: ${keyword}`
			};
		}
	}

	return null;
}

/**
 * Get all document symbols (for outline view)
 */
export function getDocumentSymbols(text: string): DocumentSymbol[] {
	const analysis = analyzeDocument(text);
	return analysis.symbols;
}
