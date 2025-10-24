# PohLang Language Server

<div align="center">

![PohLang LSP](https://img.shields.io/badge/LSP-3.17-blue)
![Version](https://img.shields.io/badge/version-0.1.0-green)
![License](https://img.shields.io/badge/license-MIT-yellow)

**Language Server Protocol implementation for PohLang in TypeScript**

*Professional IDE features for PohLang across all editors*

</div>

---

## 🎯 What is This?

This is a **professional LSP server** for PohLang, written in TypeScript following industry best practices. It provides intelligent code features like autocompletion, hover information, go-to-definition, diagnostics, and more to any editor that supports the Language Server Protocol.

## ✨ Features

- ✅ **Autocompletion** - IntelliSense for PohLang keywords and symbols
- ✅ **Hover Information** - Documentation on hover
- ✅ **Go to Definition** - Jump to symbol definitions
- ✅ **Document Symbols** - Outline view of your code
- ✅ **Code Formatting** - Auto-format your PohLang files
- ✅ **Real-time Diagnostics** - Catch errors as you type

## 🏗️ Architecture

```
Editor (VS Code, Vim, Emacs, etc.)
        ↓
    LSP Client
        ↓
   LSP Protocol (JSON-RPC over stdio)
        ↓
PohLang LSP Server (this project)
        ↓
    PohLang Runtime
```

**Key Point**: This is NOT a VS Code extension. It's a standalone server that ANY editor can connect to.

## 📦 Installation

### Prerequisites
- Node.js >= 16.0.0
- npm or yarn
- For VS Code: PohLang extension (connects automatically)
- For other editors: LSP client plugin

### Build from Source

```bash
# Navigate to LSP directory
cd PohLang/LSP

# Install dependencies
npm install

# Compile TypeScript
npm run compile

# Make launcher executable (Linux/Mac)
chmod +x pohlang-lsp.sh

# Test the server
./pohlang-lsp.sh
# or on Windows:
pohlang-lsp.bat
```

## 🚀 Usage

### With VS Code

Install the **PohLang extension** from VS Code marketplace. It automatically discovers and connects to this server.

### With Neovim

Using `nvim-lspconfig`:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

-- Define PohLang LSP
if not configs.pohlang_lsp then
  configs.pohlang_lsp = {
    default_config = {
      cmd = {'/path/to/PohLang/LSP/pohlang-lsp.sh'},
      filetypes = {'pohlang', 'poh'},
      root_dir = lspconfig.util.root_pattern('.git', 'package.poh'),
      settings = {},
    },
  }
end

-- Enable it
lspconfig.pohlang_lsp.setup{}
```

### With Emacs

Using `lsp-mode`:

```elisp
(require 'lsp-mode)

(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection '("/path/to/PohLang/LSP/pohlang-lsp.sh"))
  :major-modes '(pohlang-mode)
  :server-id 'pohlang-lsp))

(add-hook 'pohlang-mode-hook #'lsp)
```

### With Sublime Text

Using LSP package, add to settings:

```json
{
  "clients": {
    "pohlang-lsp": {
      "enabled": true,
      "command": ["/path/to/PohLang/LSP/pohlang-lsp.sh"],
      "selector": "source.pohlang"
    }
  }
}
```

### With Helix

Add to `languages.toml`:

```toml
[[language]]
name = "pohlang"
scope = "source.pohlang"
file-types = ["poh"]
language-server = { command = "/path/to/PohLang/LSP/pohlang-lsp.sh" }
```

## 🔧 Development

### Project Structure

```
PohLang/LSP/
├── src/
│   ├── server.ts           # Main LSP server
│   ├── analyzer.ts         # Code analysis & parsing
│   ├── keywords.ts         # PohLang keywords & operators
│   └── launcher.ts         # Server launcher
├── out/                    # Compiled JavaScript (generated)
├── server/                 # Old PohLang implementation (kept for reference)
├── pohlang-lsp.sh          # Unix launcher
├── pohlang-lsp.bat         # Windows launcher
├── package.json            # NPM package config
├── tsconfig.json           # TypeScript config
└── README.md               # This file
```

### Development Workflow

```bash
# Install dependencies
npm install

# Watch mode (auto-compile on changes)
npm run watch

# Compile once
npm run compile

# Clean build artifacts
npm run clean
```

### Testing

```bash
# Manual test with LSP client
node out/server.js --stdio

# Test in VS Code
# 1. Open PohLang project in VS Code
# 2. F5 to launch Extension Development Host
# 3. Open a .poh file

# With LSP inspector
npm install -g @vscode/test-cli
```

### Debugging

VS Code launch configuration (add to `.vscode/launch.json`):

```json
{
  "type": "node",
  "request": "attach",
  "name": "Attach to LSP Server",
  "port": 6009,
  "restart": true,
  "outFiles": ["${workspaceFolder}/LSP/out/**/*.js"]
}
```

Then start server with:
```bash
node --inspect=6009 out/server.js
```

## 📚 LSP Protocol Support

| Feature | Status | Notes |
|---------|--------|-------|
| initialize | ✅ | Full support |
| textDocument/didOpen | ✅ | Document tracking |
| textDocument/didChange | ✅ | Incremental sync |
| textDocument/completion | ✅ | Keywords + symbols |
| textDocument/hover | ✅ | Documentation |
| textDocument/definition | ✅ | Go to definition |
| textDocument/documentSymbol | ✅ | Outline view |
| textDocument/formatting | ✅ | Auto-format |
| textDocument/references | 🚧 | Planned |
| textDocument/rename | 🚧 | Planned |
| textDocument/codeAction | 🚧 | Planned |

## 🤝 Contributing

Contributions welcome! This server is written in TypeScript following LSP best practices.

1. Fork the repository
2. Create a feature branch
3. Make your changes to `src/*.ts`
4. Run `npm run compile` to check for errors
5. Test with multiple editors
6. Submit a pull request

### Adding New Features

**To add a new LSP feature:**

1. Add handler in `src/server.ts` (e.g., `connection.onCodeAction`)
2. Implement logic in `src/analyzer.ts` if needed
3. Update capabilities in `onInitialize`
4. Test with VS Code or other editors
5. Update documentation

## 📄 License

MIT License - Same as PohLang project

## 🔗 Links

- [PohLang Repository](https://github.com/AlhaqGH/PohLang)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [VS Code Extension](https://github.com/AlhaqGH/PohLang-Hub-(VS_code_extention))

## 💡 Why TypeScript?

**Benefits of TypeScript LSP implementation:**

1. **Universal** - Works with ANY editor (VS Code, Vim, Emacs, Sublime, etc.)
2. **Professional** - Industry-standard approach using official LSP libraries
3. **Maintainable** - Type-safe, well-structured codebase
4. **Testable** - Easy to test independently with standard tools
5. **Updatable** - Update server without touching editor extensions
6. **Performance** - Fast native execution with Node.js
7. **Ecosystem** - Leverage npm packages and TypeScript tooling

---

<div align="center">

Made with ❤️ for the PohLang community

</div>
