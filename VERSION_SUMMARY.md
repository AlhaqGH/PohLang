# PohLang Ecosystem Version Summary

**Current Release:** Phase 8 Complete - v0.6.6

## Component Versions (as of Phase 8)

| Component | Version | Repository | Status |
|-----------|---------|------------|--------|
| **PohLang Runtime** | v0.6.6 | [PohLang](https://github.com/AlhaqGH/PohLang) | ✅ Released |
| **PLHub SDK** | v0.6.0 | [PLHub](https://github.com/AlhaqGH/PLHub) | ✅ Released |
| **PLHub VS Code Extension** | v0.2.5 | [PLHub/Editor](https://github.com/AlhaqGH/PLHub/tree/main/Editor) | ✅ Ready |
| **PohLang Language Extension** | v0.6.6 | [PohLang_VSCE](https://github.com/AlhaqGH/PohLang_VSCE) | ✅ Ready |

## Versioning Strategy

### PohLang Runtime (v0.6.6)
- Primary version number follows semantic versioning
- Increments with major language features and breaking changes
- Phase 8 introduces: inline caching, VM statistics, enhanced errors

### PLHub SDK (v0.6.0)
- Uses **independent versioning** from PohLang runtime
- Reflects SDK capabilities and tooling features
- v0.6.0 adds: Phase 8 runtime integration, enhanced CLI, better error handling

### PLHub Extension (v0.2.5)
- Tracks PLHub Editor extension features
- Minor version updates for new SDK management capabilities
- v0.2.5 adds: SDK version display, enhanced environment integration

### PohLang Language Extension (v0.6.6)
- Matches PohLang runtime version for clarity
- Provides syntax highlighting, IntelliSense, and diagnostics
- v0.6.6 adds: Phase 8 error format support

## Version Compatibility Matrix

| PLHub SDK | Compatible PohLang Versions | Notes |
|-----------|----------------------------|-------|
| v0.6.0 | v0.6.6 | Phase 8 features supported |
| v0.5.4 | v0.5.4, v0.5.3 | Phase 7 compatibility |
| v0.5.1-0.5.3 | v0.5.0-v0.5.3 | Baseline functionality |

## Phase 8 Features (v0.6.6)

### Runtime Enhancements
- **Inline Caching**: 256-slot cache for global/local lookups
- **VM Statistics**: Detailed performance metrics
- **Enhanced Error Messages**: Line numbers and source context
- **Optimizations**: Constant folding, instruction fusion, dead code elimination

### PLHub Improvements
- Integrated PohLang v0.6.6 runtime
- Enhanced error reporting with source context
- Improved CLI ergonomics
- Better SDK version management

### Extension Updates
- Support for Phase 8 error formats
- Enhanced diagnostics integration
- Improved IntelliSense with VM statistics awareness

## Download Links

### PohLang Runtime v0.6.6
- Source: https://github.com/AlhaqGH/PohLang/releases/tag/v0.6.6
- Windows x64: `pohlang-v0.6.6-windows-x64.zip`
- Linux x64: `pohlang-v0.6.6-linux-x64.tar.gz`
- macOS x64: `pohlang-v0.6.6-macos-x64.tar.gz`

### PLHub SDK v0.6.0
- Source: https://github.com/AlhaqGH/PLHub/releases/tag/plhub-v0.6.0
- SDK Bundle: `plhub-sdk-0.6.0.tar.gz` / `plhub-sdk-0.6.0.zip`
- PyPI: `pip install plhub==0.6.0`

### VS Code Extensions
- PLHub Extension: Available in PLHub/Editor directory (v0.2.5)
- Language Extension: https://github.com/AlhaqGH/PohLang_VSCE (v0.6.6)

## Changelog Highlights

### PohLang v0.6.6
- Inline caching system with 256-slot cache
- VM statistics tracking and reporting
- Enhanced error messages with line numbers
- Performance optimizations (constant folding, DCE)

### PLHub v0.6.0
- Embedded PohLang v0.6.6 runtime
- Enhanced error reporting
- Improved CLI and SDK management
- Phase 8 feature documentation

### PLHub Extension v0.2.5
- Updated for PLHub v0.6.0 / PohLang v0.6.6
- Enhanced SDK version display
- Better environment integration

### Language Extension v0.6.6
- Phase 8 error format support
- Enhanced diagnostics
- Improved IntelliSense

## Installation Guide

### Quick Start with PLHub (Recommended)
```bash
# Install PLHub SDK
pip install plhub==0.6.0

# Verify installation
plhub --version
# Output: PL-Hub SDK v0.6.0 (PohLang v0.6.6)

# Create new project
plhub create my-app --template console

# Run your program
plhub run src/main.poh
```

### Standalone PohLang Runtime
1. Download binary for your platform from releases
2. Extract to desired location
3. Add to PATH
4. Run: `pohlang --version`

### VS Code Extensions
1. **PLHub Extension**: 
   - Navigate to `PLHub/Editor`
   - Run: `code --install-extension plhub-extension-0.2.5.vsix`

2. **Language Extension**:
   - Download from PohLang_VSCE releases
   - Run: `code --install-extension pohlang-0.6.6.vsix`

## Support and Documentation

- **Main Documentation**: [PohLang/doc/PohLang_Guide.md](./doc/PohLang_Guide.md)
- **PLHub Guide**: [PLHub/README.md](https://github.com/AlhaqGH/PLHub/blob/main/README.md)
- **Roadmap**: [PohLang/doc/ROADMAP.md](./doc/ROADMAP.md)
- **Phase 8 Details**: [PohLang/PHASE_8_COMPLETE.md](./PHASE_8_COMPLETE.md)

## Future Versions

### Planned for v0.7.0
- Class support with inheritance
- Module system improvements
- Standard library expansion
- Performance profiling tools

### Planned for PLHub v0.7.0
- Package repository integration
- Enhanced project templates
- Build system improvements
- Testing framework integration

---

**Last Updated**: October 2024  
**Status**: All Phase 8 components released and operational
