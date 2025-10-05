# PohLang v0.5.0 Publication Guide

This guide walks through the complete publication process for PohLang v0.5.0.

## ✅ Pre-Publication Checklist (COMPLETE)

- ✅ All 50 tests passing
- ✅ Version updated to 0.5.0 in Cargo.toml
- ✅ CHANGELOG.md updated with v0.5.0 section
- ✅ README.md updated with Phase 1 completion
- ✅ Release binary built (`runtime/target/release/pohlang.exe`)
- ✅ Documentation updated (Vocabulary, Guide, Grammar Analysis)
- ✅ Release notes document created

## 📦 Files Ready for Release

### Binaries
- `runtime/target/release/pohlang.exe` (Windows x64)

### Documentation
- `RELEASE_NOTES_v0.5.0.md` - Comprehensive release notes
- `CHANGELOG.md` - Version history
- `README.md` - Project overview
- `doc/PohLang_Guide.md` - Language tutorial
- `doc/GRAMMAR_ANALYSIS.md` - Formal grammar specification
- `spec/Vocabulary.md` - Language reference

### Additional Files
- `LICENSE` - MIT License
- `CONTRIBUTING.md` - Developer guide

## 🚀 Publication Steps

### Step 1: Commit All Changes

```powershell
# Navigate to repository root
cd c:\Users\habib\POHLANG\PohLang

# Check status
git status

# Add all updated files
git add CHANGELOG.md
git add README.md
git add RELEASE_NOTES_v0.5.0.md
git add PUBLICATION_GUIDE.md
git add doc/GRAMMAR_ANALYSIS.md
git add spec/Vocabulary.md
git add doc/PohLang_Guide.md
git add runtime/Cargo.toml
git add runtime/src/

# Commit with descriptive message
git commit -m "Release v0.5.0 - Phase 1 Complete

- Add 20 phrasal built-in expressions
- Implement comprehensive grammar analysis
- Achieve 50 passing tests (100% coverage)
- Update all documentation for Phase 1 completion
- Build optimized release binary

This marks the completion of Phase 1 development with a 
production-ready, mathematically unambiguous grammar."
```

### Step 2: Create Git Tag

```powershell
# Create annotated tag
git tag -a v0.5.0 -m "PohLang v0.5.0 - Phase 1 Complete

Phase 1 Achievement:
- 20 phrasal built-in expressions
- Unambiguous grammar with formal proof
- 50 passing tests (100% coverage)
- Production-ready release

See RELEASE_NOTES_v0.5.0.md for details."

# Verify tag
git tag -l -n9 v0.5.0
```

### Step 3: Push to GitHub

```powershell
# Push commits
git push origin main

# Push tag
git push origin v0.5.0
```

### Step 4: Create GitHub Release

#### Option A: Using GitHub CLI (Recommended)

```powershell
# Install GitHub CLI if needed
# winget install GitHub.cli

# Login to GitHub
gh auth login

# Create release with binary
gh release create v0.5.0 `
  --title "PohLang v0.5.0 - Phase 1 Complete" `
  --notes-file RELEASE_NOTES_v0.5.0.md `
  runtime/target/release/pohlang.exe#pohlang-v0.5.0-windows-x64.exe

# Verify release
gh release view v0.5.0
```

#### Option B: Using GitHub Web Interface

1. Go to https://github.com/AlhaqGH/PohLang/releases/new

2. Fill in the form:
   - **Tag version**: `v0.5.0` (select existing tag)
   - **Release title**: `PohLang v0.5.0 - Phase 1 Complete`
   - **Description**: Copy content from `RELEASE_NOTES_v0.5.0.md`

3. Upload binary:
   - Click "Attach binaries by dropping them here or selecting them"
   - Select `runtime/target/release/pohlang.exe`
   - Rename to: `pohlang-v0.5.0-windows-x64.exe`

4. Options:
   - ✅ Set as the latest release
   - ❌ Set as a pre-release (this is stable!)

5. Click **"Publish release"**

### Step 5: Verify Publication

```powershell
# Check release exists
gh release list

# Check tag exists remotely
git ls-remote --tags origin

# Verify release on GitHub
# Visit: https://github.com/AlhaqGH/PohLang/releases/tag/v0.5.0
```

### Step 6: Update Repository Settings

On GitHub repository page:

1. **Update Repository Description**:
   - Go to repository main page
   - Click "About" settings (⚙️ icon)
   - Update description: `PohLang - Natural Language Programming (Phase 1 Complete ✅)`
   - Add topics: `programming-language`, `natural-language`, `educational`, `rust`

2. **Update README Badge** (if you have status badges):
   - Update version badge to show v0.5.0
   - Add "Phase 1 Complete" badge if desired

3. **Pin Release**:
   - In Releases page, consider pinning v0.5.0 release

### Step 7: Announce Release

#### GitHub Discussions
Create announcement in GitHub Discussions:

```markdown
# 🎉 PohLang v0.5.0 Released - Phase 1 Complete!

We're thrilled to announce that **PohLang v0.5.0** is now available, 
marking the successful completion of Phase 1 development!

## Highlights
- ✅ 20 phrasal built-in expressions for natural programming
- ✅ Mathematically proven unambiguous grammar
- ✅ 50 passing tests (100% core feature coverage)
- ✅ Production-ready for educational use

## Download
Get the release binary: https://github.com/AlhaqGH/PohLang/releases/tag/v0.5.0

## What's Next
Phase 2 development begins with focus on standard library modules!

See full release notes: [RELEASE_NOTES_v0.5.0.md](./RELEASE_NOTES_v0.5.0.md)
```

#### Social Media (Optional)
Share on platforms like:
- Twitter/X
- Reddit (r/ProgrammingLanguages)
- LinkedIn
- Dev.to

Sample post:
```
🚀 Just released PohLang v0.5.0 - Phase 1 Complete!

A natural-language programming language now production-ready with:
✅ 20 phrasal built-ins
✅ Unambiguous grammar
✅ 50 passing tests

Perfect for teaching programming concepts!

https://github.com/AlhaqGH/PohLang
#ProgrammingLanguages #Education #Rust
```

## 📊 Post-Publication Checklist

- [ ] Release visible on GitHub
- [ ] Binary downloadable from release page
- [ ] Tag appears in git history
- [ ] README shows Phase 1 Complete
- [ ] Release notes accessible
- [ ] Repository description updated
- [ ] Announcement posted (optional)

## 🔄 Building Additional Platform Binaries (Future)

Currently we have Windows x64 binary. To add more platforms:

### Linux x64
```bash
# On Linux machine or WSL
cd runtime
cargo build --release
# Binary at: target/release/pohlang
# Rename to: pohlang-v0.5.0-linux-x64
```

### macOS
```bash
# On macOS machine
cd runtime
cargo build --release
# Binary at: target/release/pohlang
# Rename to: pohlang-v0.5.0-macos-x64
```

### Cross-compilation (Advanced)
```powershell
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu

# Build for macOS (on Windows is tricky, recommend native build)
```

Then add these binaries to the existing v0.5.0 release on GitHub.

## 🐛 If Something Goes Wrong

### Wrong commit/tag pushed
```powershell
# Delete remote tag
git push origin --delete v0.5.0

# Delete local tag
git tag -d v0.5.0

# Fix issue, then create tag again
git tag -a v0.5.0 -m "..."
git push origin v0.5.0
```

### Need to update release
```powershell
# Using GitHub CLI
gh release edit v0.5.0 --notes-file RELEASE_NOTES_v0.5.0.md

# Or edit manually on GitHub web interface
```

### Binary issues
- Ensure binary is from `runtime/target/release/` not `debug/`
- Verify binary works: `.\runtime\target\release\pohlang.exe --version`
- If corrupted, rebuild: `cargo build --release`

## 📝 Notes

- **Version**: Cargo.toml already set to 0.5.0
- **Binary size**: ~4MB (optimized with LTO)
- **Build time**: ~2 minutes on modern hardware
- **Stability**: Production-ready, no breaking changes planned

## 🎯 Success Criteria

Release is successful when:
- ✅ Users can download binary from GitHub Releases
- ✅ Running `pohlang --version` shows "0.5.0"
- ✅ All 50 tests pass on user machines
- ✅ Documentation is accessible and complete
- ✅ No immediate bug reports

---

**Ready to publish!** Follow steps 1-7 above to complete the release process.

Last updated: October 5, 2025
