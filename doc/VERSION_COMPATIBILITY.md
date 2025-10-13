# PohLang Version Compatibility Guide

**Last Updated**: October 12, 2025

## Current Versions

| Component | Version | Status | Release Date |
|-----------|---------|--------|--------------|
| **PohLang Runtime** | `0.6.0` | ✅ Stable | October 2025 |
| **PLHub SDK** | `0.5.4` | ✅ Stable | October 2025 |
| **PohLang Language Extension** | `0.3.1` | ✅ Stable | October 12, 2025 |
| **PLHub Extension** | `0.2.3` | ✅ Stable | October 12, 2025 |

---

## Extension Requirements

### PohLang Language Support Extension (v0.3.1)
**Required for**: Syntax highlighting, IntelliSense, code snippets

**Supports Runtime Versions**:
- ✅ v0.6.0 (Current - Full support)
- ✅ v0.5.4 (Phase 5 - Error Handling)
- ✅ v0.5.2 (Phase 4 - Collections)
- ⚠️ v0.4.x (Partial support - basic features only)

**Features**:
- Syntax highlighting (all PohLang constructs)
- Code snippets (30+ templates)
- IntelliSense (keyword completion)
- Hover documentation (30+ keywords)
- Signature help (function parameters)
- Real-time diagnostics (error detection)

**Installation**:
```bash
code --install-extension pohlang.pohlang
```

---

### PLHub Extension (v0.2.3)
**Required for**: Running code, project management, SDK updates

**Requires**:
- ✅ PohLang Language Support Extension (v0.3.0+)
- ✅ PohLang Runtime (v0.6.0+ recommended)

**Supports Runtime Versions**:
- ✅ v0.6.0 (Recommended - Phase 8 optimizations)
- ✅ v0.5.4 (Compatible - Phase 5 error handling)
- ⚠️ v0.5.2 (Compatible - limited error handling)

**Features**:
- Run PohLang files (Ctrl+F5)
- Create new projects
- Update runtime automatically
- Manage SDK versions
- GitHub integration for updates

**Installation**:
```bash
code --install-extension pohlang.plhub
```

---

## Version History

### Runtime v0.6.0 (Current)
**Release**: October 2025  
**Major Features**:
- ✅ Phase 8: Bytecode VM Optimizations (78% complete)
- ✅ Constant folding (compile-time optimization)
- ✅ Instruction fusion (combined operations)
- ✅ Peephole optimization (pattern-based improvements)
- ✅ Inline caching (property access optimization)
- ✅ Enhanced error messages (instruction pointers + suggestions)
- ⏳ VM statistics (planned)
- ⏳ Integration tests (planned)

**Performance**: 5-10x faster than v0.5.4 (target: 50-100x with all optimizations)

**Breaking Changes**: None (fully backward compatible)

**Extension Support**:
- Language Extension: v0.3.1+
- PLHub Extension: v0.2.3+

---

### Runtime v0.5.4
**Release**: October 2025  
**Major Features**:
- ✅ Phase 5: Error Handling System
- ✅ Try/Catch/Finally blocks
- ✅ 7 built-in error types
- ✅ Custom error types
- ✅ Natural English error messages
- ✅ File location tracking

**Breaking Changes**: None

**Extension Support**:
- Language Extension: v0.2.5 - v0.3.0
- PLHub Extension: v0.2.0 - v0.2.2

---

### Runtime v0.5.2
**Release**: September 2025  
**Major Features**:
- ✅ Phase 4: Collections & Modern Syntax
- ✅ Modern list syntax [1, 2, 3]
- ✅ Dictionary syntax
- ✅ Indexing operations
- ✅ Phrasal expressions

**Breaking Changes**: Old collection syntax deprecated

**Extension Support**:
- Language Extension: v0.2.0
- PLHub Extension: v0.1.x

---

## Compatibility Matrix

| Runtime | Language Ext | PLHub Ext | Features Supported |
|---------|-------------|-----------|-------------------|
| v0.6.0 | v0.3.1 | v0.2.3 | ✅ All (Current) |
| v0.6.0 | v0.3.0 | v0.2.2 | ⚠️ Most (missing separation fixes) |
| v0.5.4 | v0.3.0 | v0.2.0 | ✅ Error Handling |
| v0.5.4 | v0.2.5 | v0.2.0 | ✅ Error Handling |
| v0.5.2 | v0.2.0 | v0.1.x | ⚠️ Basic (no error handling) |
| v0.4.x | v0.1.x | v0.1.x | ❌ Legacy (not recommended) |

**Legend**:
- ✅ Fully supported and tested
- ⚠️ Works but missing features or fixes
- ❌ Not recommended (may have bugs)

---

## Recommended Configurations

### For Development (Recommended)
```json
{
  "runtime": "0.6.0",
  "languageExtension": "0.3.1",
  "plhubExtension": "0.2.3"
}
```
**Why**: Latest features, best performance, all optimizations

---

### For Stability
```json
{
  "runtime": "0.5.4",
  "languageExtension": "0.3.1",
  "plhubExtension": "0.2.3"
}
```
**Why**: Stable error handling, less experimental features

---

### For Learning (Beginners)
```json
{
  "runtime": "0.6.0",
  "languageExtension": "0.3.1",
  "plhubExtension": "0.2.3"
}
```
**Why**: Best error messages, fastest performance, latest features

---

## Upgrading Guide

### Upgrading Runtime (0.5.4 → 0.6.0)

**Automatic** (via PLHub Extension):
1. Open VS Code
2. Press Ctrl+Shift+P
3. Run: "PL-Hub: Update Language"
4. Extension downloads latest runtime automatically

**Manual**:
```bash
# Download from GitHub releases
https://github.com/AlhaqGH/PohLang/releases/tag/v0.6.0

# Or use PLHub CLI
plhub install --runtime latest
```

**Changes Required**: None (backward compatible)

---

### Upgrading Extensions

**Language Extension** (0.3.0 → 0.3.1):
```bash
code --install-extension pohlang.pohlang --force
```

**PLHub Extension** (0.2.0 → 0.2.3):
```bash
code --install-extension pohlang.plhub --force
```

**Important**: After upgrading to v0.2.3+, ensure you have BOTH extensions installed:
- Language Extension provides syntax highlighting
- PLHub Extension provides runtime execution

---

## Feature Support by Version

### Phase 8 Features (Runtime v0.6.0)

| Feature | Language Ext | PLHub Ext | Notes |
|---------|-------------|-----------|-------|
| Constant Folding | N/A | ✅ v0.2.3 | Compile-time optimization |
| Instruction Fusion | N/A | ✅ v0.2.3 | Combined operations |
| Peephole Optimization | N/A | ✅ v0.2.3 | Pattern-based improvements |
| Inline Caching | N/A | ✅ v0.2.3 | Property access speedup |
| Enhanced Errors | ✅ v0.3.1 | ✅ v0.2.3 | IP tracking + suggestions |

---

### Phase 5 Features (Runtime v0.5.4)

| Feature | Language Ext | PLHub Ext | Notes |
|---------|-------------|-----------|-------|
| Try/Catch/Finally | ✅ v0.2.5 | ✅ v0.2.0 | Full syntax support |
| Error Types | ✅ v0.2.5 | ✅ v0.2.0 | 7 built-in types |
| Custom Errors | ✅ v0.2.5 | ✅ v0.2.0 | User-defined types |
| Error Diagnostics | ✅ v0.2.5 | ✅ v0.2.0 | Real-time validation |

---

## Extension Separation (v0.2.3)

### Why Two Extensions?

Starting with v0.2.3, PohLang uses **two separate extensions**:

**Before (v0.2.0)**: Single extension
- ❌ Both provided language support
- ❌ Duplicate registrations caused crashes
- ❌ VS Code would deactivate extensions

**After (v0.2.3)**: Separate extensions
- ✅ Language Extension: Syntax + IntelliSense only
- ✅ PLHub Extension: Runtime + SDK management only
- ✅ No conflicts, clean architecture
- ✅ Better performance and stability

### Which Extension Do I Need?

**Just viewing code**: Language Extension only  
**Running code**: Both extensions required  
**Full development**: Both extensions required

---

## Troubleshooting Version Issues

### "Command not found: PL-Hub: Run File"
**Problem**: PLHub extension not installed  
**Solution**: Install PLHub extension (v0.2.3+)
```bash
code --install-extension pohlang.plhub
```

---

### "No syntax highlighting for .poh files"
**Problem**: Language extension not installed  
**Solution**: Install Language extension (v0.3.1+)
```bash
code --install-extension pohlang.pohlang
```

---

### "Extension conflict: Language already registered"
**Problem**: Old version with duplicate language support  
**Solution**: Update both extensions to latest versions
```bash
code --install-extension pohlang.pohlang --force
code --install-extension pohlang.plhub --force
```

---

### "Runtime version mismatch"
**Problem**: Extension expects newer runtime  
**Solution**: Update runtime via PLHub extension
1. Press Ctrl+Shift+P
2. Run: "PL-Hub: Update Language"
3. Or set `pohlangHub.autoUpdate: true` in settings

---

### "Errors not showing in Problems panel"
**Problem**: Using old runtime without enhanced errors  
**Solution**: Upgrade to runtime v0.6.0
```bash
plhub install --runtime v0.6.0
```

---

## Version Update Schedule

### Stable Releases (Monthly)
- **Runtime**: Major version every 2-3 months
- **Extensions**: Bug fixes + features monthly
- **SDK**: Synchronized with runtime releases

### Development Builds (Weekly)
- Available on GitHub releases
- Tagged as pre-release
- Not recommended for production

---

## Future Versions (Planned)

### Runtime v0.7.0 (November 2025)
**Phase 9**: Standard Library Modules
- Collections module (sort, filter, map)
- Random module (RNG, shuffle)
- Math module (sqrt, trig, logs)
- DateTime module (Hijri calendar)

### Runtime v0.8.0 (February 2026)
**Phase 10**: AOT Compilation
- Cranelift backend integration
- Static binary generation
- 10-50x performance improvement

### Runtime v1.0.0 (July 2026)
**Production Release**
- Package manager
- REPL, debugger, profiler
- Enhanced VS Code features
- Full documentation

---

## Getting Help

### Check Your Versions
```powershell
# In VS Code terminal
pohlang --version         # Runtime version
plhub --version          # SDK version

# In VS Code Extensions panel
# Search: "pohlang" to see installed versions
```

### Report Issues
- Runtime bugs: https://github.com/AlhaqGH/PohLang/issues
- Extension bugs: https://github.com/AlhaqGH/PLHub/issues
- Include version numbers in bug reports!

---

**Status**: All components updated to support runtime v0.6.0  
**Recommended Setup**: Runtime v0.6.0 + Both extensions v0.3.1/v0.2.3
