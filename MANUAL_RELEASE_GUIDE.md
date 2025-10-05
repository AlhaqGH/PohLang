# Manual GitHub Release Creation Guide

## Status: Workflow Fixed ✅

The release workflow has been fixed and pushed. Here's how to create the release:

## Option 1: Wait for Automated Workflow (Recommended)

The workflow will automatically run when triggered by tag push. Since the tag `v0.5.0` already exists:

1. Delete and recreate the tag to trigger the workflow:
   ```powershell
   # Delete remote tag
   git push origin --delete v0.5.0
   
   # Delete local tag
   git tag -d v0.5.0
   
   # Create tag again
   git tag -a v0.5.0 -m "PohLang v0.5.0 - Phase 1 Complete"
   
   # Push tag (this will trigger the workflow)
   git push origin v0.5.0
   ```

2. Watch the workflow run at:
   https://github.com/AlhaqGH/PohLang/actions

3. Once complete, the release will be automatically created with binaries for:
   - Windows (pohlang-v0.5.0-windows-x64.zip)
   - Linux (pohlang-v0.5.0-linux-x64.tar.gz)
   - macOS (pohlang-v0.5.0-macos-x64.tar.gz)

## Option 2: Manual Release Creation (Quick)

1. Go to: https://github.com/AlhaqGH/PohLang/releases/new

2. Fill in the form:
   - **Choose a tag**: Select `v0.5.0` from dropdown
   - **Release title**: `PohLang v0.5.0 - Phase 1 Complete`
   - **Description**: Copy content from `RELEASE_NOTES_v0.5.0.md`

3. Upload files:
   - Click "Attach binaries by dropping them here or selecting them"
   - Upload: `pohlang-v0.5.0-windows-x64.zip` (from project root)

4. Options:
   - ✅ Set as the latest release
   - ❌ Set as a pre-release

5. Click **"Publish release"**

## What Was Fixed in Workflow

### Problem
- Artifacts were not being created in the correct location
- Archive creation was failing
- Download step couldn't find artifacts

### Solution
- Changed binary preparation to use `release-artifacts/` directory
- Renamed artifacts to `binary-{os}` for easier identification
- Fixed packaging step to properly create ZIP/tar.gz from downloaded artifacts
- Updated file paths in release upload

### Changes Made
```yaml
# Before:
- Create archives in nested build directory
- Upload with wildcard path patterns
- Hope artifacts end up in right place

# After:
- Copy binaries to known location (release-artifacts/)
- Upload with explicit artifact names (binary-{os})
- Download and package properly in create-release job
- Use explicit file patterns for upload
```

## Verification Steps

After release is created:

1. **Check Release Page**:
   - Go to: https://github.com/AlhaqGH/PohLang/releases/tag/v0.5.0
   - Verify release notes are displayed
   - Verify ZIP file is attached

2. **Test Download**:
   - Download the ZIP file
   - Extract it
   - Run `pohlang.exe --version`
   - Should show: `pohlang 0.5.0`

3. **Test Example**:
   ```powershell
   cd extracted-folder
   .\pohlang.exe --run examples\poh\hello.poh
   ```

## Files Ready for Upload

Located in project root:
- `pohlang-v0.5.0-windows-x64.zip` (0.59 MB)
  - Contains binary, docs, examples, quick start guide

Located in `release-v0.5.0/` directory:
- Complete release package with all files
- Can be zipped separately if needed

## Workflow Status

- ✅ Workflow file fixed and committed
- ✅ Changes pushed to GitHub
- ✅ Ready to trigger on next tag push
- ⏳ Waiting for manual trigger or re-push of tag

## Next Action

Choose one:

**A. Automated (Better)**: Delete and re-push tag to trigger workflow
```powershell
git push origin --delete v0.5.0
git tag -d v0.5.0
git tag -a v0.5.0 -m "PohLang v0.5.0 - Phase 1 Complete"
git push origin v0.5.0
```

**B. Manual (Faster)**: Create release manually on GitHub web interface
- Takes 2-3 minutes
- Upload pre-made ZIP
- Done!

## After Release is Published

1. Update PUBLICATION_SUCCESS.md with release URL
2. Post announcement in Discussions
3. Share on social media (optional)
4. Monitor Issues for bug reports

---

**Current Status**: Ready to create release with either method!
