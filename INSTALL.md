# PohLang Installation Guide

**✅ No Rust, Visual Studio, or Build Tools Required!**

PohLang is distributed as a standalone binary. Just download and run!

---

## Quick Install (Recommended)

### Windows

#### One-Line Install (PowerShell)
```powershell
irm https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.ps1 | iex
```

Or download manually:
1. Download [pohlang-v0.5.2-windows-x64.zip](https://github.com/AlhaqGH/PohLang/releases/latest)
2. Extract the zip file
3. Run `pohlang.exe`

### Linux

#### One-Line Install
```bash
curl -sSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash
```

Or install manually:
```bash
wget https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-v0.5.2-linux-x64.tar.gz
tar -xzf pohlang-v0.5.2-linux-x64.tar.gz
sudo mv pohlang /usr/local/bin/
```

### macOS

#### One-Line Install
```bash
curl -sSL https://raw.githubusercontent.com/AlhaqGH/PohLang/main/install/install.sh | bash
```

Or install manually:
```bash
curl -L https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-v0.5.2-macos-x64.tar.gz -o pohlang.tar.gz
tar -xzf pohlang.tar.gz
sudo mv pohlang /usr/local/bin/
```

---

## SDK Package (Includes Examples & Docs)

If you want a complete package with examples and documentation:

### Windows
```powershell
# Download SDK
Invoke-WebRequest -Uri "https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-sdk-v0.5.2-windows-x64.zip" -OutFile pohlang-sdk.zip
Expand-Archive -Path pohlang-sdk.zip -DestinationPath "C:\PohLang"
cd C:\PohLang\pohlang-sdk-*
```

### Linux
```bash
wget https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-sdk-v0.5.2-linux-x64.tar.gz
tar -xzf pohlang-sdk-v0.5.2-linux-x64.tar.gz
cd pohlang-sdk-*
./bin/pohlang --run examples/hello.poh
```

### macOS
```bash
curl -L https://github.com/AlhaqGH/PohLang/releases/download/v0.5.2/pohlang-sdk-v0.5.2-macos-x64.tar.gz -o sdk.tar.gz
tar -xzf sdk.tar.gz
cd pohlang-sdk-*
./bin/pohlang --run examples/hello.poh
```

---

## Verify Installation

After installation, test it:

```bash
# Check version
pohlang --version

# Run a quick test
echo 'Start Program
Write "Hello from PohLang!"
End Program' > test.poh

pohlang --run test.poh
```

---

## VS Code Extension (Optional)

For the best development experience, install the VS Code extension:

1. Install [Visual Studio Code](https://code.visualstudio.com/)
2. Search for "PohLang Hub" in the Extensions marketplace
3. Install the extension
4. Open any `.poh` file and press `Ctrl+F5` to run

Extension includes:
- ✅ Syntax highlighting
- ✅ IntelliSense & code completion
- ✅ 40+ code snippets
- ✅ Integrated runtime
- ✅ One-click execution

---

## Uninstallation

### Windows
```powershell
# If installed with script
Remove-Item "$env:LOCALAPPDATA\PohLang" -Recurse -Force
# Or if installed system-wide (as admin)
Remove-Item "C:\Program Files\PohLang" -Recurse -Force

# Remove from PATH manually in System Environment Variables
```

### Linux/macOS
```bash
sudo rm /usr/local/bin/pohlang
```

---

## Building from Source (For Developers)

If you want to contribute or modify PohLang:

1. Install [Rust](https://rustup.rs/)
2. Clone the repository:
   ```bash
   git clone https://github.com/AlhaqGH/PohLang.git
   cd PohLang
   ```
3. Build:
   ```bash
   cargo build --release --manifest-path runtime/Cargo.toml
   ```
4. Binary will be at `runtime/target/release/pohlang`

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

---

## Troubleshooting

### Windows: "pohlang is not recognized"
- Restart your terminal after installation
- Or add the install directory to PATH manually

### Linux/macOS: "Permission denied"
```bash
chmod +x pohlang
```

### macOS: "pohlang cannot be opened because it is from an unidentified developer"
```bash
xattr -d com.apple.quarantine pohlang
```

Or: System Preferences → Security & Privacy → Click "Open Anyway"

### Need More Help?
- [GitHub Issues](https://github.com/AlhaqGH/PohLang/issues)
- [Discussions](https://github.com/AlhaqGH/PohLang/discussions)

---

## What's Next?

1. **Learn the basics**: Read [PohLang_Guide.md](doc/PohLang_Guide.md)
2. **Try examples**: Explore the [examples/poh](examples/poh) folder
3. **Build something**: Create your first program!
4. **Join the community**: Share your projects on GitHub Discussions

**Happy coding with PohLang!** 🎉
