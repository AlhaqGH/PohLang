# PohLang Language Server Protocol (LSP) Implementation

A standalone Language Server Protocol implementation for PohLang, enabling IDE features across multiple editors.

## Overview

This is a **standalone LSP server** written in PohLang that provides:
- **Syntax completion** (IntelliSense)
- **Hover information** (documentation on hover)
- **Go to definition** (jump to symbol definitions)
- **Find references** (locate all symbol usages)
- **Document symbols** (outline view)
- **Diagnostics** (real-time error checking)
- **Code formatting** (auto-format documents)

## Architecture

```
┌─────────────────┐
│   Any Editor    │  (VS Code, Neovim, Emacs, Sublime, etc.)
│  (LSP Client)   │
└────────┬────────┘
         │ LSP Protocol (JSON-RPC over stdio)
         │
┌────────▼────────┐
│  PohLang LSP    │
│     Server      │  (This project - written in PohLang)
│  (standalone)   │
└─────────────────┘
```

## Features

### ✅ Implemented
- Basic LSP initialization
- Text document synchronization
- Completion provider (keywords, built-ins)
- Hover provider
- Definition provider
- Document symbol provider

### 🚧 Planned
- Semantic tokens (syntax highlighting)
- Code actions (quick fixes)
- Rename provider
- References provider
- Signature help
- Workspace symbols

## Usage

### Running the Server

```bash
# Direct execution
pohlang server/lsp-server.poh

# Or if installed
pohlang-lsp
```

The server communicates via **stdio** using the Language Server Protocol.

### Connecting from VS Code

Use the PohLang VS Code extension (separate project) which automatically starts and connects to this server.

### Connecting from Other Editors

#### Neovim (using nvim-lspconfig)
```lua
require'lspconfig'.configs.pohlang = {
  default_config = {
    cmd = {'pohlang', '/path/to/lsp-server.poh'},
    filetypes = {'pohlang'},
    root_dir = function() return vim.fn.getcwd() end,
  },
}

require'lspconfig'.pohlang.setup{}
```

#### Emacs (using lsp-mode)
```elisp
(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection '("pohlang" "/path/to/lsp-server.poh"))
                  :major-modes '(pohlang-mode)
                  :server-id 'pohlang-lsp))
```

#### Sublime Text (using LSP package)
```json
{
  "clients": {
    "pohlang-lsp": {
      "enabled": true,
      "command": ["pohlang", "/path/to/lsp-server.poh"],
      "selector": "source.pohlang"
    }
  }
}
```

## Protocol Compliance

This server implements LSP 3.17 specification:
- **Lifecycle**: initialize, initialized, shutdown, exit
- **Document**: didOpen, didChange, didClose, didSave
- **Language Features**: completion, hover, definition, documentSymbol, formatting

## Development

### Project Structure
```
server/
  ├── lsp-server.poh       # Main LSP server implementation
  ├── protocol.poh         # LSP protocol types and messages
  ├── analyzer.poh         # Code analysis and diagnostics
  ├── completions.poh      # Completion provider
  └── README.md            # This file
```

### Testing
```bash
# Test with sample input
echo 'Content-Length: 123\r\n\r\n{"jsonrpc":"2.0","id":1,"method":"initialize"}' | pohlang server/lsp-server.poh
```

## LSP Protocol Messages

### Example Initialize Request
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": null,
    "rootUri": "file:///path/to/project",
    "capabilities": {}
  }
}
```

### Example Completion Request
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "textDocument/completion",
  "params": {
    "textDocument": {"uri": "file:///path/to/file.poh"},
    "position": {"line": 5, "character": 10}
  }
}
```

## Benefits of Standalone Architecture

1. **Editor Agnostic**: Works with any editor that supports LSP
2. **Single Implementation**: Maintain one codebase for all IDEs
3. **Consistent Features**: Same functionality across all editors
4. **Independent Updates**: Update server without updating editor extensions
5. **Testing**: Easy to test using standard LSP testing tools

## License

Same as PohLang project
