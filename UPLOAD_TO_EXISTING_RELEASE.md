# Upload Additional Artifacts to v0.6.1 Release

## Current Situation

✅ **Release v0.6.1 exists** on GitHub: https://github.com/AlhaqGH/PohLang/releases/tag/v0.6.1

The "Simple Release" workflow automatically:
- ✅ Created the release
- ✅ Built binaries for Linux, Windows, macOS
- ✅ Uploaded platform-specific packages

## Missing Artifacts to Upload

You have these local artifacts that should be added to the release:

### 1. Windows Binary SHA256 Checksum
**File:** `C:\Users\habib\POHLANG\PohLang\pohlang-v0.6.1-windows.zip.sha256`  
**Size:** 94 bytes  
**Contains:** SHA256 hash for the Windows binary package

### 2. Additional Artifacts (If Desired)
You may also want to add:
- Language Extension VSIX: `pohlang-0.3.3.vsix` (can be uploaded here or to its own release)
- PLHub Extension VSIX: `plhub-0.2.4.vsix` (can be uploaded here or to its own release)

## How to Upload to Existing Release

### Method 1: Via GitHub Web Interface (Easiest)

1. **Navigate to the release:**
   ```
   https://github.com/AlhaqGH/PohLang/releases/tag/v0.6.1
   ```

2. **Click "Edit release"** (pencil icon at top right)

3. **Scroll to "Attach binaries"** section at bottom

4. **Drag and drop or click to upload:**
   - `C:\Users\habib\POHLANG\PohLang\pohlang-v0.6.1-windows.zip.sha256`

5. **Update release notes** (optional):
   - Add SHA256 hash in the description
   - Enhance with detailed notes from `RELEASE_PUBLICATION_GUIDE_v0.6.1.md`

6. **Click "Update release"**

### Method 2: Via GitHub CLI (gh)

If you have GitHub CLI installed:

```powershell
cd C:\Users\habib\POHLANG\PohLang

# Upload SHA256 file
gh release upload v0.6.1 pohlang-v0.6.1-windows.zip.sha256

# Verify upload
gh release view v0.6.1
```

### Method 3: Via GitHub API with PowerShell

```powershell
$token = "your_github_token"
$repo = "AlhaqGH/PohLang"
$tag = "v0.6.1"
$file = "C:\Users\habib\POHLANG\PohLang\pohlang-v0.6.1-windows.zip.sha256"

# Get release ID
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/tags/$tag" -Headers @{Authorization = "token $token"}

# Upload asset
$uploadUrl = $release.upload_url -replace '\{\?name,label\}', "?name=pohlang-v0.6.1-windows.zip.sha256"
Invoke-RestMethod -Uri $uploadUrl -Method Post -InFile $file -Headers @{
    Authorization = "token $token"
    "Content-Type" = "text/plain"
}
```

## Recommended Enhancement: Update Release Notes

While editing the release, consider enhancing the release notes with details from `RELEASE_PUBLICATION_GUIDE_v0.6.1.md`:

### Enhanced Release Notes Template

```markdown
## PohLang Runtime v0.6.1

### 🎯 Highlights
- Enhanced VM error messages with instruction pointer tracking
- Improved error diagnostics for runtime issues
- Stable foundation for language extension v0.3.3

### ✨ New Features
- **Enhanced VM Errors**: Instruction-pointer-aware error messages
  - Added `VMError::StackUnderflowAt(usize)`
  - Added `VMError::StackOverflowAt(usize)`
  - Added `VMError::TypeErrorAt(String, usize)`
  - Added `VMError::DivisionByZeroAt(usize)`
  - Improved error display with suggestions

### 🐛 Bug Fixes
- Fixed command registration issues in extensions
- Improved error reporting consistency
- Fixed code formatting for CI workflow

### 🧪 Testing
- All runtime tests passing
- Comprehensive VM error tests added
- Integration tests validated

### 📦 Downloads

| Platform | File | SHA256 |
|----------|------|--------|
| Windows x64 | `pohlang-v0.6.1-windows-x64.zip` | `111D0CDED63E00EED683369E601921515A24551E422C864D5B9EA92B2E739DDA` |
| Linux x64 | `pohlang-v0.6.1-linux-x64.tar.gz` | *(see attached file)* |
| macOS x64 | `pohlang-v0.6.1-macos-x64.tar.gz` | *(see attached file)* |

### 📚 Installation

**Windows:**
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/AlhaqGH/PohLang/releases/download/v0.6.1/pohlang-v0.6.1-windows-x64.zip" -OutFile "pohlang.zip"
Expand-Archive -Path "pohlang.zip" -DestinationPath "C:\Program Files\PohLang"

# Add to PATH
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\PohLang", [EnvironmentVariableTarget]::User)

# Verify
pohlang --version
```

**Linux/macOS:**
```bash
# Download and extract
curl -L https://github.com/AlhaqGH/PohLang/releases/download/v0.6.1/pohlang-v0.6.1-linux-x64.tar.gz | tar xz

# Move to PATH
sudo mv pohlang /usr/local/bin/

# Verify
pohlang --version
```

### 🔗 Related Releases
- Language Extension: [v0.3.3](https://github.com/AlhaqGH/PohLang-Hub/releases/tag/v0.3.3)
- PLHub Extension: [v0.2.4](https://github.com/AlhaqGH/PLHub/releases/tag/ext-v0.2.4)
- PLHub SDK: [v0.5.5](https://github.com/AlhaqGH/PLHub/releases/tag/sdk-v0.5.5)

### 💬 Feedback
Report issues: https://github.com/AlhaqGH/PohLang/issues
```

## Summary of Files in Release

After upload, the release should contain:

- ✅ `pohlang-v0.6.1-windows-x64.zip` (automated)
- ✅ `pohlang-v0.6.1-linux-x64.tar.gz` (automated)
- ✅ `pohlang-v0.6.1-macos-x64.tar.gz` (automated)
- ⬆️ `pohlang-v0.6.1-windows.zip.sha256` (manual upload)

## Next Steps

1. ✅ Upload SHA256 file to v0.6.1 release
2. ✅ Enhance release notes (optional)
3. ⏩ Create releases for extensions:
   - Language Extension v0.3.3
   - PLHub Extension v0.2.4
   - PLHub SDK v0.5.5
4. ⏩ Publish extensions to VS Code Marketplace

---

**Quick Action:** Visit https://github.com/AlhaqGH/PohLang/releases/tag/v0.6.1 and click "Edit release" to add the SHA256 file!
