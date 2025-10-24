# PohLang LSP Server - TypeScript Implementation

This directory contains the TypeScript-based Language Server Protocol implementation for PohLang.

## Quick Start

```bash
# Install dependencies
npm install

# Compile TypeScript to JavaScript
npm run compile

# Start the server (for testing)
node out/server.js --stdio
```

## Architecture

The LSP server is built using the official Microsoft LSP libraries:
- `vscode-languageserver` - LSP protocol implementation
- `vscode-languageserver-textdocument` - Text document utilities

### Components

1. **server.ts** - Main entry point
   - Connection setup
   - Capability registration
   - Request handlers

2. **analyzer.ts** - Code analysis
   - Document parsing
   - Symbol extraction
   - Error detection
   - Semantic analysis

3. **keywords.ts** - Language definitions
   - PohLang keywords
   - Operators
   - Documentation

4. **launcher.ts** - Server launcher utility

## Features Implemented

- ✅ Document synchronization (open, change, close)
- ✅ Completions (keywords, operators, symbols)
- ✅ Hover information
- ✅ Go to definition
- ✅ Document symbols (outline view)
- ✅ Diagnostics (errors and warnings)
- ✅ Document formatting

## Adding New Features

To add a new LSP capability:

1. Register the capability in `onInitialize` result
2. Add the handler (e.g., `connection.onReferences`)
3. Implement the logic (often in analyzer.ts)
4. Test with an LSP client

## Testing

### Manual Test
```bash
# Start server in stdio mode
node out/server.js --stdio

# In another terminal, send LSP messages:
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | node out/server.js --stdio
```

### With VS Code
The PohLang VS Code extension can connect to this server automatically.

## Debugging

Start with Node.js inspector:
```bash
node --inspect=6009 out/server.js --stdio
```

Then attach your debugger to port 6009.

## Build Scripts

- `npm run compile` - Compile TypeScript once
- `npm run watch` - Watch mode (auto-compile on save)
- `npm run clean` - Remove build artifacts
- `npm run start` - Start the server

## LSP Protocol

This server implements LSP 3.17 specification:
https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

## Performance Considerations

- Document caching for fast repeated analysis
- Incremental document sync (only changes sent)
- Lazy symbol resolution
- Debounced diagnostics
