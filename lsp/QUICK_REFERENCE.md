# PohLang LSP Server - Quick Reference

## Installation

```bash
cd PohLang/LSP
npm install
npm run compile
```

## Usage

### Start Server
```bash
# Unix/Linux/Mac
./pohlang-lsp.sh

# Windows
pohlang-lsp.bat

# Direct
node out/server.js --stdio
```

### VS Code
The PohLang extension automatically connects. No configuration needed.

### Neovim (nvim-lspconfig)
```lua
require('lspconfig').configs.pohlang_lsp = {
  default_config = {
    cmd = {'/path/to/PohLang/LSP/pohlang-lsp.sh'},
    filetypes = {'pohlang', 'poh'},
    root_dir = require('lspconfig').util.root_pattern('.git'),
  }
}
require('lspconfig').pohlang_lsp.setup{}
```

### Emacs (lsp-mode)
```elisp
(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection '("/path/to/PohLang/LSP/pohlang-lsp.sh"))
  :major-modes '(pohlang-mode)
  :server-id 'pohlang-lsp))
```

## Development

### Build Commands
```bash
npm run compile      # Compile once
npm run watch        # Auto-compile on save
npm run clean        # Remove build artifacts
npm run start        # Start server
```

### Debug
```bash
node --inspect=6009 out/server.js --stdio
```
Then attach debugger to port 6009.

## Features

| Feature | Status | Description |
|---------|--------|-------------|
| Completion | ✅ | Keywords, operators, symbols |
| Hover | ✅ | Documentation on hover |
| Definition | ✅ | Go to definition (F12) |
| Symbols | ✅ | Document outline (Ctrl+Shift+O) |
| Diagnostics | ✅ | Real-time error checking |
| Formatting | ✅ | Code formatting |
| References | 🚧 | Find all references (planned) |
| Rename | 🚧 | Symbol renaming (planned) |

## Files

```
LSP/
├── src/              TypeScript source
│   ├── server.ts     Main server
│   ├── analyzer.ts   Code analysis
│   └── keywords.ts   Language defs
├── out/              Compiled JS (generated)
└── server/           Old implementation (reference)
```

## Troubleshooting

### "node: command not found"
Install Node.js: https://nodejs.org

### "Cannot find module 'vscode-languageserver'"
Run: `npm install`

### Server not responding
Check if compilation succeeded: `npm run compile`

### VS Code not connecting
1. Check extension is installed
2. Reload window (Ctrl+R)
3. Check Output panel → PohLang

## Requirements

- Node.js >= 16.0.0
- npm or yarn
- PohLang runtime (for testing .poh files)

## Links

- [Full Documentation](./README.md)
- [Development Guide](./DEVELOPMENT.md)
- [Migration Notes](./MIGRATION_COMPLETE.md)
- [PohLang Repo](https://github.com/AlhaqGH/PohLang)
