# PohLang SDK Release Playbook

This playbook covers the complete release process for the PohLang SDK (the core language).

## Overview

PohLang SDK is the standalone language implementation that provides:
- Python interpreter (`pohlang` command)
- Dart transpiler (experimental)
- Core language libraries and runtime
- Language specification and documentation

Think: **PohLang SDK** is like **Dart SDK** - the core language tools.

## Release Types

### Automatic Release (Recommended)
Triggered by GitHub Actions when you push a `pohlang-v*` tag.

### Manual Release
For hotfixes or custom builds.

## Pre-Release Checklist

### 1. Version Management
- [ ] Update `Interpreter/__init__.py` with new `__version__`
- [ ] Update `pyproject.toml` with matching version
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Verify no version mismatches

### 2. Testing
- [ ] Run full test suite: `python -m pytest tests_python/`
- [ ] Run interpreter smoke test: `python -m Interpreter.run_poh examples/hello.poh`
- [ ] Test Dart transpiler: `dart run bin/pohlang.dart examples/hello.poh`
- [ ] Verify CLI works: `python -m pohlang examples/hello.poh`

### 3. Documentation
- [ ] Update README.md if needed
- [ ] Update language documentation in `doc/`
- [ ] Verify examples still work

## Release Process

### Option A: Automated Release (GitHub Actions)

1. **Create and push tag:**
   ```bash
   git tag pohlang-v0.5.0
   git push origin pohlang-v0.5.0
   ```

2. **Monitor GitHub Actions:**
   - Go to Actions tab in GitHub
   - Watch the "PohLang SDK Release" workflow
   - Verify all jobs pass (test, build-python-sdk, build-dart-sdk, release)

3. **Verify release artifacts:**
   - Check GitHub Releases page
   - Download and test Python wheel
   - Download and test Dart SDK bundle
   - Verify PyPI publication

### Option B: Manual Release

1. **Run release script:**
   ```bash
   python prepare_release.py
   ```

2. **Build distributions:**
   ```bash
   python -m build
   ```

3. **Test installation:**
   ```bash
   pip install dist/pohlang-*.whl
   pohlang examples/hello.poh
   ```

4. **Upload to PyPI:**
   ```bash
   python -m twine upload dist/*
   ```

5. **Create GitHub release:**
   - Go to GitHub Releases
   - Create new release with tag
   - Upload wheel and source dist
   - Upload Dart SDK bundle

## Post-Release Tasks

### 1. Verification
- [ ] Test PyPI installation: `pip install pohlang==X.Y.Z`
- [ ] Test GitHub release downloads
- [ ] Verify documentation is updated

### 2. Communication
- [ ] Update PohLang website/docs
- [ ] Announce on social media/forums
- [ ] Notify PL-Hub maintainers for integration

### 3. PL-Hub Integration
- [ ] File issue in PL-Hub repo to integrate new version
- [ ] Or trigger PL-Hub release with new PohLang version

## Version Numbering

PohLang uses semantic versioning:
- **Major**: Breaking language changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes only

Examples:
- `0.5.0` - First stable interpreter
- `0.5.1` - Bug fixes
- `0.6.0` - New language features
- `1.0.0` - First stable language release

## Troubleshooting

### Test Failures
```bash
# Run specific test
python -m pytest tests_python/test_interpreter.py -v

# Run with debug output
python -m pytest tests_python/ -v -s
```

### Build Failures
```bash
# Clean build artifacts
rm -rf build/ dist/ *.egg-info/

# Rebuild
python -m build
```

### Dart Transpiler Issues
```bash
# Check Dart SDK installation
dart --version

# Run transpiler tests
cd transpiler
dart test
```

### PyPI Upload Issues
```bash
# Check package
python -m twine check dist/*

# Test upload to TestPyPI first
python -m twine upload --repository testpypi dist/*
```

## Emergency Procedures

### Yanking a Release
If a critical bug is discovered:

1. **Yank from PyPI:**
   ```bash
   # Requires PyPI maintainer access
   python -m twine yank pohlang==X.Y.Z
   ```

2. **Mark GitHub release as pre-release**
3. **Release hotfix ASAP**

### Rollback
If automation fails:

1. Delete the problematic tag
2. Fix issues locally
3. Re-tag and re-release

## Files Modified During Release

- `Interpreter/__init__.py` - Version number
- `pyproject.toml` - Package version
- `CHANGELOG.md` - Release notes
- `RELEASE_NOTES.md` - Generated release notes
- Git tags - `pohlang-vX.Y.Z`

## Contact

For release issues, contact:
- GitHub Issues: Create issue with `release` label
- Maintainers: See AUTHORS.md