#!/usr/bin/env node

import {
	createConnection,
	TextDocuments,
	ProposedFeatures,
	InitializeParams,
	TextDocumentSyncKind,
	InitializeResult,
	CompletionItem,
	CompletionItemKind,
	TextDocumentPositionParams,
	Hover,
	MarkupKind,
	Definition,
	Location,
	DocumentSymbol,
	SymbolKind,
	Range,
	Diagnostic,
	DiagnosticSeverity,
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import { analyzeDocument, findSymbolAtPosition, getDocumentSymbols } from './analyzer';
import { pohLangKeywords, pohLangOperators } from './keywords';

// Create a connection for the server using Node's IPC as a transport
const connection = createConnection(ProposedFeatures.all);

// Create a text document manager
const documents = new TextDocuments(TextDocument);

// Server capabilities
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;

connection.onInitialize((params: InitializeParams) => {
	const capabilities = params.capabilities;

	hasConfigurationCapability = !!(
		capabilities.workspace && !!capabilities.workspace.configuration
	);
	hasWorkspaceFolderCapability = !!(
		capabilities.workspace && !!capabilities.workspace.workspaceFolders
	);

	const result: InitializeResult = {
		capabilities: {
			textDocumentSync: TextDocumentSyncKind.Incremental,
			completionProvider: {
				resolveProvider: true,
				triggerCharacters: [' ', '.', ':']
			},
			hoverProvider: true,
			definitionProvider: true,
			documentSymbolProvider: true,
			documentFormattingProvider: true,
		},
		serverInfo: {
			name: 'PohLang LSP Server',
			version: '0.1.0'
		}
	};

	if (hasWorkspaceFolderCapability) {
		result.capabilities.workspace = {
			workspaceFolders: {
				supported: true
			}
		};
	}

	return result;
});

connection.onInitialized(() => {
	connection.console.log('PohLang Language Server initialized');
});

// Document change handling
documents.onDidChangeContent(change => {
	validateTextDocument(change.document);
});

// Validate document and provide diagnostics
async function validateTextDocument(textDocument: TextDocument): Promise<void> {
	const text = textDocument.getText();
	const diagnostics: Diagnostic[] = [];

	const analysis = analyzeDocument(text);

	// Check for syntax errors
	for (const error of analysis.errors) {
		const diagnostic: Diagnostic = {
			severity: DiagnosticSeverity.Error,
			range: error.range,
			message: error.message,
			source: 'pohlang'
		};
		diagnostics.push(diagnostic);
	}

	// Check for warnings
	for (const warning of analysis.warnings) {
		const diagnostic: Diagnostic = {
			severity: DiagnosticSeverity.Warning,
			range: warning.range,
			message: warning.message,
			source: 'pohlang'
		};
		diagnostics.push(diagnostic);
	}

	connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// Completion provider
connection.onCompletion(
	(_textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
		const items: CompletionItem[] = [];

		// Add keywords
		pohLangKeywords.forEach((keyword, index) => {
			items.push({
				label: keyword.label,
				kind: CompletionItemKind.Keyword,
				detail: keyword.detail,
				documentation: keyword.documentation,
				insertText: keyword.insertText || keyword.label,
				data: index
			});
		});

		// Add operators
		pohLangOperators.forEach((operator, index) => {
			items.push({
				label: operator.label,
				kind: CompletionItemKind.Operator,
				detail: operator.detail,
				documentation: operator.documentation,
				insertText: operator.insertText || operator.label,
				data: 1000 + index
			});
		});

		return items;
	}
);

// Completion resolve (provides additional information)
connection.onCompletionResolve(
	(item: CompletionItem): CompletionItem => {
		return item;
	}
);

// Hover provider
connection.onHover(
	(params: TextDocumentPositionParams): Hover | null => {
		const document = documents.get(params.textDocument.uri);
		if (!document) {
			return null;
		}

		const text = document.getText();
		const offset = document.offsetAt(params.position);
		const symbol = findSymbolAtPosition(text, offset);

		if (symbol) {
			return {
				contents: {
					kind: MarkupKind.Markdown,
					value: [
						`**${symbol.name}** (${symbol.kind})`,
						'',
						symbol.documentation || 'PohLang symbol'
					].join('\n')
				}
			};
		}

		return null;
	}
);

// Definition provider (Go to Definition)
connection.onDefinition(
	(params: TextDocumentPositionParams): Definition | null => {
		const document = documents.get(params.textDocument.uri);
		if (!document) {
			return null;
		}

		const text = document.getText();
		const offset = document.offsetAt(params.position);
		const symbol = findSymbolAtPosition(text, offset);

		if (symbol && symbol.definitionRange) {
			return Location.create(
				params.textDocument.uri,
				symbol.definitionRange
			);
		}

		return null;
	}
);

// Document symbols provider (Outline)
connection.onDocumentSymbol(
	(params): DocumentSymbol[] => {
		const document = documents.get(params.textDocument.uri);
		if (!document) {
			return [];
		}

		const text = document.getText();
		return getDocumentSymbols(text);
	}
);

// Document formatting provider
connection.onDocumentFormatting(
	(params) => {
		const document = documents.get(params.textDocument.uri);
		if (!document) {
			return [];
		}

		// Basic formatting: ensure consistent indentation
		// This is a simple implementation - can be enhanced
		return [];
	}
);

// Make the text document manager listen on the connection
documents.listen(connection);

// Listen on the connection
connection.listen();

connection.console.log('PohLang Language Server is running...');
