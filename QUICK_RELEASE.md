# 🚀 RELEASE NOW - Quick Commands

## One-Command Release (Copy & Paste)

```pwsh
# Navigate and release PLHub
cd C:\Users\habib\POHLANG\PLHub; git add -A; git commit -m "chore: release v0.5.2 - fix test suite warnings"; git tag -a v0.5.2 -m "PLHub v0.5.2"; git push origin main v0.5.2

# Navigate and release Extension  
cd C:\Users\habib\POHLANG\PohLang-Hub-(VS_code_extention); git add -A; git commit -m "chore: release v0.1.1 - maintenance release"; git tag -a v0.1.1 -m "Extension v0.1.1"; git push origin main v0.1.1

# Navigate and push documentation
cd C:\Users\habib\POHLANG\PohLang; git add -A; git commit -m "docs: release v0.5.2 documentation and phase 2 kickoff"; git push origin main

Write-Host "`n✅ ALL RELEASES COMPLETE!" -ForegroundColor Green
```

---

## Start Phase 2 File I/O (Copy & Paste)

```pwsh
cd C:\Users\habib\POHLANG\PohLang

# Create module structure
mkdir -Force runtime\src\stdlib
'// File I/O module for PohLang standard library' | Out-File -FilePath runtime\src\stdlib\io.rs -Encoding UTF8
'pub mod io;' | Out-File -FilePath runtime\src\stdlib\mod.rs -Encoding UTF8

# Create test structure
mkdir -Force runtime\tests\stdlib
'// Tests for File I/O module' | Out-File -FilePath runtime\tests\stdlib\io_tests.rs -Encoding UTF8

# Create example files
@'
Start Program
    # File I/O Basics Example
    Write "Creating a file..."
    
    # Write to file
    Make content = "Hello, PohLang File I/O!"
    Write to file "test.txt" the content
    
    # Read from file
    Make data = Read file "test.txt"
    Write data
End Program
'@ | Out-File -FilePath examples\poh\file_io_basics.poh -Encoding UTF8

Write-Host "`n✅ PHASE 2 READY! Open runtime/src/stdlib/io.rs and start coding!" -ForegroundColor Green
```

---

## What Gets Released

**PLHub v0.5.2:**
- Fixed pytest test suite
- All tests passing (11/11)
- Zero warnings

**Extension v0.1.1:**
- Validated all features
- Runtime v0.5.2 bundled
- TypeScript compiled

**Documentation:**
- Verification report
- Release plan
- Phase 2 roadmap

---

## After Release

1. ✅ Check GitHub for tags
2. ✅ Create releases on GitHub (web UI or `gh release create`)
3. ✅ Start Phase 2 File I/O implementation

---

## Questions?

- See `COMPLETE_GUIDE.md` for full details
- See `PHASE_2_KICKOFF.md` for Phase 2 specs
- See `VERIFICATION_REPORT.md` for test results

---

**READY TO GO! 🚀**
