# Release Automation Guide

## Overview

PohLang now has **fully automated release workflow** that builds and publishes both:
1. **SDK Packages** - Complete development kits with binary, docs, examples, test scripts
2. **Binary Packages** - Standalone executables for advanced users

Every release or update automatically generates packages for **Windows, Linux, and macOS**.

## 🚀 How to Trigger a Release

### Method 1: Git Tag (Recommended)

```bash
# Create and push a tag
git tag -a v0.6.0 -m "Release v0.6.0"
git push origin v0.6.0
```

This automatically triggers the release workflow and publishes to GitHub Releases.

### Method 2: Manual Trigger

1. Go to: https://github.com/AlhaqGH/PohLang/actions/workflows/release-v0.5.0.yml
2. Click **"Run workflow"**
3. Enter tag (e.g., `v0.6.0`)
4. Click **"Run workflow"**

## 📦 What Gets Built

### SDK Packages (Recommended for Users)

Each SDK package includes:

```
pohlang-sdk-{platform}-v{version}/
├── pohlang(.exe)              # Binary executable
├── QUICK_START.md             # 5-minute getting started
├── RELEASE_NOTES.md           # Version-specific release notes
├── LICENSE                    # MIT License
├── README.md                  # Project overview
├── CHANGELOG.md               # Complete history
├── test_examples.{bat|sh}     # Test script
├── doc/                       # Complete documentation
│   ├── PohLang_Guide.md
│   ├── GRAMMAR_ANALYSIS.md
│   ├── ROADMAP.md
│   └── ...
├── examples/                  # 20+ example programs
│   └── poh/
│       ├── hello.poh
│       ├── arithmetic.poh
│       └── ...
└── spec/                      # Language specification
    ├── Vocabulary.md
    └── Grammar.ebnf
```

**Filename Format:**
- Windows: `pohlang-sdk-windows-x64-v{version}.zip`
- Linux: `pohlang-sdk-linux-x64-v{version}.tar.gz`
- macOS: `pohlang-sdk-macos-x64-v{version}.tar.gz`

### Binary Packages (Advanced Users)

Just the executable:
- Windows: `pohlang-binary-windows-x64.zip`
- Linux: `pohlang-binary-linux-x64.tar.gz`
- macOS: `pohlang-binary-macos-x64.tar.gz`

## 🔧 Workflow Steps

The automated workflow performs these steps:

### 1. Test Suite (Ubuntu)
```yaml
- Checkout code
- Install Rust toolchain
- Run all 50 tests
- Verify 3+ test suites pass
```

### 2. Build Release Binaries (Multi-Platform)
```yaml
- Checkout code on Windows, Linux, macOS
- Install Rust with target platform
- Build optimized release binary
- Test binary (verify version)
- Upload binary artifact
```

### 3. Build SDK Packages (Multi-Platform)
```yaml
- Download binary artifact
- Create SDK directory structure
- Copy binary + docs + examples + spec
- Generate QUICK_START.md (platform-specific)
- Generate RELEASE_NOTES.md (version-specific)
- Generate test script (platform-specific)
- Create archive (.zip for Windows, .tar.gz for Unix)
- Upload SDK package artifact
```

### 4. Create GitHub Release
```yaml
- Download all artifacts (binaries + SDKs)
- Organize release packages
- Create standalone binary packages
- Generate release notes
- Create GitHub Release with all assets
- Mark as latest release
```

### 5. Verify Release
```yaml
- Confirm release created
- List all assets
```

## 📊 Release Timeline

Total time: **~15-20 minutes**

| Step | Time | Runs On |
|------|------|---------|
| Test Suite | ~2 min | Ubuntu |
| Build Windows Binary | ~3 min | Windows |
| Build Linux Binary | ~3 min | Ubuntu |
| Build macOS Binary | ~3 min | macOS |
| Build Windows SDK | ~1 min | Windows |
| Build Linux SDK | ~1 min | Ubuntu |
| Build macOS SDK | ~1 min | macOS |
| Create Release | ~1 min | Ubuntu |

Steps run in parallel where possible.

## 🎯 Version Management

### Version Sources

The version is automatically extracted from:
1. **Git Tag**: `v0.5.0` → version `0.5.0`
2. **Manual Input**: Tag field in workflow dispatch

### Version Embedding

Update version in these files before tagging:

```toml
# runtime/Cargo.toml
[package]
version = "0.6.0"
```

```rust
// runtime/src/main.rs (if using clap)
.version("0.6.0")
```

## 📝 Customizing Release Notes

The workflow generates complete release notes automatically. To customize:

1. **For specific release**: Edit the `Create release notes` step in `.github/workflows/release-v0.5.0.yml`

2. **For SDK packages**: Release notes are auto-generated in `build-sdk-packages` job

3. **For GitHub Release**: Main release notes come from `create-release` job

## 🔍 Verifying a Release

After workflow completes:

1. Visit: https://github.com/AlhaqGH/PohLang/releases
2. Verify latest release shows your version
3. Check assets:
   - ✅ 3 SDK packages (Windows, Linux, macOS)
   - ✅ 3 Binary packages (Windows, Linux, macOS)
   - ✅ Total: 6 downloadable files

4. Test downloads:
   ```bash
   # Download SDK
   # Extract
   # Run: ./pohlang --version
   # Run: ./pohlang --run examples/poh/hello.poh
   # Run: ./test_examples.sh (or .bat)
   ```

## 🛠️ Troubleshooting

### Release Not Created

**Check:**
1. GitHub Actions status: https://github.com/AlhaqGH/PohLang/actions
2. Test suite passed?
3. All platforms built successfully?
4. GITHUB_TOKEN permissions correct?

**Fix:**
- Re-run failed jobs in Actions tab
- Check logs for specific errors
- Verify Cargo.toml version updated

### Missing Assets

**Check:**
1. Artifact upload logs in workflow
2. File paths in artifacts

**Fix:**
- Verify directory structure in `build-sdk-packages` step
- Check archive creation step didn't fail
- Re-run workflow

### Wrong Version in Binary

**Fix:**
1. Update `runtime/Cargo.toml`
2. Update version in `main.rs` if hardcoded
3. Rebuild and re-tag

## 🚀 Future Enhancements

Planned automation improvements:

- [ ] Automatic changelog generation from commits
- [ ] Auto-increment version numbers
- [ ] Release candidate (RC) builds
- [ ] Nightly builds
- [ ] Download statistics tracking
- [ ] Homebrew formula auto-update
- [ ] Chocolatey package auto-publish
- [ ] Docker image builds

## 📚 Related Documentation

- [CONTRIBUTING.md](CONTRIBUTING.md) - Development workflow
- [AUTOMATION_IMPROVEMENTS.md](AUTOMATION_IMPROVEMENTS.md) - CI/CD changes log
- [PUBLICATION_GUIDE.md](PUBLICATION_GUIDE.md) - Manual publication steps (deprecated)
- [MANUAL_RELEASE_GUIDE.md](MANUAL_RELEASE_GUIDE.md) - Fallback for automation failure

---

## Quick Reference

### Create Release
```bash
git tag -a v0.6.0 -m "Release v0.6.0"
git push origin v0.6.0
```

### Check Status
https://github.com/AlhaqGH/PohLang/actions

### View Releases
https://github.com/AlhaqGH/PohLang/releases

### Workflow File
`.github/workflows/release-v0.5.0.yml`

---

**Automated releases save time and ensure consistency across all platforms!** 🎉
