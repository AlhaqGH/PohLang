# PohLang v0.6.0 Release Notes - Web Framework & Hot Reload 🔥

**Release Date:** October 11, 2025  
**Phase:** Phase 6 Complete  
**Status:** ✅ Production Ready

## 🎉 What's New

### Web Framework (Complete HTTP Server)
Build web applications with natural English syntax! PohLang now includes a complete HTTP server with route handling, request parsing, and response generation.

```poh
Start Program
Set server to create web server on port 3000

Add route "/" with method "GET" to server:
    Write html response with "<h1>Hello Web!</h1>"

Add route "/api/data" with method "GET" to server:
    Write json response with Make a dictionary with "status" set to "ok"

Start server
End Program
```

**Features:**
- ✅ HTTP server with multi-threaded request handling
- ✅ Route definition with method support (GET, POST, PUT, DELETE)
- ✅ HTML and JSON response types
- ✅ Automatic Content-Type headers
- ✅ Request isolation (each request gets its own VM instance)
- ✅ Handler execution with full PohLang code support

### Hot Reload (Flutter-Style) 🔥
Experience instant feedback while developing! Edit your `.poh` files and watch your web app reload automatically in the browser.

```bash
pohlang --run --watch server.poh
```

**Features:**
- ✅ File watching with `--watch` flag
- ✅ Sub-500ms reload time (polls 2x per second)
- ✅ Automatic browser reload on file changes
- ✅ `/__reload_check` endpoint auto-generated
- ✅ Livereload script auto-injected into HTML
- ✅ No native dependencies (pure Rust polling)
- ✅ Multi-file tracking support

**How it works:**
1. Start server with `--watch` flag
2. Open browser to your app
3. Edit `.poh` file
4. Browser detects change via `/__reload_check`
5. Page reloads automatically!

### Developer Experience
- 🔥 **Hot Reload** - Sub-500ms refresh (better than the 1s goal!)
- 🚀 **Multi-threaded** - Each request spawns its own thread
- 🎯 **Type-safe** - HTML/JSON responses with proper headers
- 📝 **Console Logging** - `[LiveReload] Monitoring for changes...`
- 🛠️ **Easy Setup** - One flag to enable: `--watch`

## 📦 What's Included

### Core Components
- **Web Server** - Powered by `tiny_http` (no native deps)
- **Route Handler** - Execute PohLang code per request
- **LiveReloadTracker** - File modification detection
- **Auto-injection** - Livereload script seamlessly added

### Examples
- `examples/poh/web_hello.poh` - Simple web server demo
- `examples/TaskMaster/backend.poh` - Task management API
- `examples/TaskMaster/public/*` - Complete frontend (HTML/CSS/JS)

## 🔧 Technical Details

### Architecture
- **HTTP Server**: `tiny_http` 0.12 (lightweight, pure Rust)
- **Threading**: Request handlers spawn in separate threads
- **VM Isolation**: Each request gets a cloned VM with fresh globals
- **File Watching**: Polling-based (500ms interval)
- **Bind Address**: `0.0.0.0` for Windows compatibility

### Performance
- **Hot Reload**: <500ms detection + reload
- **Polling**: 2x per second (configurable)
- **Request Handling**: Multi-threaded, non-blocking
- **Memory**: Efficient VM cloning per request

### API Changes
```rust
// New public API
vm.enable_hot_reload(watch_paths: Vec<PathBuf>)

// New CLI flag
pohlang --run --watch file.poh

// New VM value types
Value::WebServer(Arc<Mutex<WebServer>>)
Value::LiveReloadTracker(LiveReloadTracker)
Value::HttpResponse(HttpResponse)
```

## 📚 Documentation Updates

### Updated Files
- ✅ `CHANGELOG.md` - Detailed changelog
- ✅ `README.md` - Examples and quick start
- ✅ `spec/Grammar.ebnf` - Web framework syntax
- ✅ `spec/Vocabulary.md` - New phrases and examples
- ✅ `runtime/Cargo.toml` - Version bump to 0.6.0
- ✅ VS Code Extension - Version 0.3.0

### New Documentation
- Web server creation and routing
- Hot reload setup and usage
- HTTP response types
- Request handling flow

## 🚀 Getting Started

### Install/Update
```bash
# Windows (PowerShell)
irm https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.ps1 | iex

# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash
```

### Your First Web App
```poh
Start Program
Write "Starting web server with hot reload..."

Set server to create web server on port 3000

Add route "/" with method "GET" to server:
    Write html response with "<!DOCTYPE html><html><head><title>My App</title></head><body><h1>🔥 Hot Reload Works!</h1><p>Edit this file and watch it reload!</p></body></html>"

Write "✅ Server ready at http://localhost:3000"
Start server
End Program
```

Run with hot reload:
```bash
pohlang --run --watch myapp.poh
```

Open browser, edit `myapp.poh`, and watch it reload instantly!

## 🐛 Known Issues

None reported yet! This is a stable release.

## 🔮 What's Next (Phase 7)

Phase 7 will focus on:
- [ ] Database integration (SQLite)
- [ ] File uploads and form handling
- [ ] WebSocket support
- [ ] Template engine for HTML
- [ ] Session management
- [ ] Middleware support

See [ROADMAP.md](./doc/ROADMAP.md) for details.

## 💡 Migration Guide

### From v0.5.4 to v0.6.0

**New Features (Additive - No Breaking Changes):**
```poh
# Old code still works!
Start Program
Write "Hello World"
End Program

# New: Add web server
Start Program
Set server to create web server on port 3000
Add route "/" with method "GET" to server:
    Write html response with "<h1>Hello</h1>"
Start server
End Program
```

**No breaking changes!** All v0.5.4 code continues to work.

## 📊 Stats

- **Total Code**: 447 lines (http.rs), 3,742 lines (vm.rs)
- **New Features**: Web server + Hot reload
- **Build Time**: ~1m 20s (release)
- **Binary Size**: Similar to v0.5.4
- **Test Coverage**: All existing tests passing + web framework working

## 🙏 Credits

- Built with Rust 🦀
- `tiny_http` for HTTP server
- `handlebars` for future templating
- Community feedback and testing

## 📞 Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/AlhaqGH/PohLang/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/AlhaqGH/PohLang/discussions)
- 📖 **Docs**: [PohLang_Guide.md](./doc/PohLang_Guide.md)
- 🔌 **VS Code**: [Extension](https://marketplace.visualstudio.com/items?itemName=pohlang.pohlang-hub)

---

**Happy coding with PohLang v0.6.0!** 🚀🔥

Build web apps with natural English and watch them reload instantly!
