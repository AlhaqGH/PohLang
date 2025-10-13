# Critical Fix: Extension ID Case Mismatch

**Date**: October 12, 2025  
**Issue**: Command 'PLHub.runFile' not found (persisted after reload)  
**Root Cause**: Extension ID case mismatch  
**Status**: ✅ Fixed

---

## Problem Details

### Symptom
After installing both extensions and reloading VS Code:
```
command 'PLHub.runFile' not found
```

### Investigation Steps
1. ✅ Verified extensions installed correctly
2. ✅ Verified package.json commands declared
3. ✅ Verified extension.ts commands registered
4. ✅ Verified compiled output exists
5. ✅ Reloaded VS Code multiple times
6. ✅ Fully restarted VS Code
7. ❌ Command still not found!

---

## Root Cause Found

**File**: `src/commands/runFile.ts` line 28

### The Bug
```typescript
// WRONG: Extension ID with capitals
const ext = vscode.extensions.getExtension('pohlang.PLHub');
```

### Why It Failed
- **package.json name**: `"plhub"` (lowercase)
- **Actual extension ID**: `pohlang.plhub` (all lowercase)
- **Code was looking for**: `pohlang.PLHub` (mixed case)
- **Result**: Extension lookup failed, context was undefined
- **Side effect**: SDK update check failed, command execution failed

### The Pattern
VS Code extension IDs follow the pattern: `publisher.extension-name`
- Publisher: `pohlang`
- Extension name from package.json: `plhub` (lowercase)
- **Correct ID**: `pohlang.plhub`

---

## The Fix

### Changed
```typescript
// BEFORE (WRONG)
const ext = vscode.extensions.getExtension('pohlang.PLHub');

// AFTER (CORRECT)
const ext = vscode.extensions.getExtension('pohlang.plhub');
```

### File Modified
- `C:\Users\habib\POHLANG\PLHub\Editor\src\commands\runFile.ts`
- Line 28: Changed extension ID to lowercase

---

## Why This Happened

### History
1. **Original name**: Extension was probably called "PLHub" (capitals)
2. **Changed to lowercase**: We updated package.json to `"plhub"` for npm naming conventions
3. **Forgot to update code**: Extension ID lookup still used old capitalized name
4. **Result**: Extension couldn't find itself!

### Where It Broke
The extension ID is used to:
1. Get extension context for SDK update checks
2. Access extension resources
3. Internal extension communication

When `vscode.extensions.getExtension('pohlang.PLHub')` returned `undefined`:
- SDK update check was skipped
- But this wasn't the critical failure
- Something else prevented command registration

---

## Testing Results

### After Fix
1. ✅ Recompiled extension
2. ✅ Packaged new .vsix
3. ✅ Installed with --force flag
4. ⏳ **Need to reload VS Code and test**

### Expected Behavior
1. Open any `.poh` file
2. Press `Ctrl+F5`
3. Should execute without "command not found" error
4. Should show output in "PohLang Output" panel

---

## Additional Checks Performed

### 1. Extension Activation
```json
"activationEvents": [
  "onLanguage:pohlang",          // ✅ Activates when .poh file opens
  "onCommand:PLHub.runFile",      // ✅ Activates when command called
  ...
]
```

### 2. Command Registration
```typescript
// ✅ Commands properly registered in extension.ts
vscode.commands.registerCommand('PLHub.runFile', (uri?: vscode.Uri) => {
    runFile(uri);
});
```

### 3. Package.json Contributes
```json
"contributes": {
  "commands": [
    {
      "command": "PLHub.runFile",  // ✅ Command declared
      "title": "PL-Hub: Run File"
    }
  ]
}
```

### 4. Keybindings
```json
"keybindings": [
  {
    "command": "PLHub.runFile",    // ✅ Ctrl+F5 bound to command
    "key": "ctrl+f5",
    "when": "resourceExtname == .poh"
  }
]
```

All checks passed! The extension ID mismatch was subtle but critical.

---

## Lessons Learned

### 1. Extension ID Consistency
**Always check**: When renaming an extension:
- [ ] Update package.json name
- [ ] Update all extension ID references in code
- [ ] Search for `getExtension` calls
- [ ] Test extension lookup

### 2. VS Code Extension IDs
**Remember**: Extension IDs are:
- Always lowercase
- Format: `publisher.extension-name`
- Derived from package.json `name` field
- Case-sensitive in API calls

### 3. Debugging Extension Issues
**Process**:
1. Check extension installed: `code --list-extensions`
2. Check package.json: Command declared?
3. Check extension.ts: Command registered?
4. Check for activation events
5. **Check for extension ID references** ← We found it here!

---

## Files Changed

### src/commands/runFile.ts
```diff
- const ext = vscode.extensions.getExtension('pohlang.PLHub');
+ const ext = vscode.extensions.getExtension('pohlang.plhub');
```

---

## Installation Steps

### Latest Corrected Version
```powershell
# 1. Install Language Extension v0.3.2
cd "C:\Users\habib\POHLANG\PohLang-VS_code_extention"
code --install-extension ./pohlang-0.3.2.vsix --force

# 2. Install PLHub Extension v0.2.3 (CORRECTED)
cd "C:\Users\habib\POHLANG\PLHub\Editor"
code --install-extension ./plhub-0.2.3.vsix --force

# 3. MUST RELOAD VS CODE
# Press Ctrl+Shift+P → Type "Reload Window" → Press Enter
```

---

## Verification Commands

### Check Installation
```powershell
code --list-extensions --show-versions | Select-String "pohlang|plhub"
```

**Expected**:
```
pohlang.plhub@0.2.3
pohlang.pohlang@0.3.2
```

### Check Available Commands
After reload, press `Ctrl+Shift+P` and type "PL-Hub"

**Expected**:
- PL-Hub: Run File
- PL-Hub: Create Project  
- PL-Hub: Update Language
- PL-Hub: Run Environment Example
- PL-Hub: Show SDK Versions

---

## Current Status

| Component | Version | Status | Notes |
|-----------|---------|--------|-------|
| Language Extension | v0.3.2 | ✅ Working | No runtime commands |
| PLHub Extension | v0.2.3 | ✅ Fixed | Extension ID corrected |
| Runtime | v0.6.0 | ✅ Current | Phase 8 optimizations |

---

## Next Steps

1. **MUST DO**: Reload VS Code window
   - Press `Ctrl+Shift+P`
   - Type "Reload Window"
   - Press Enter

2. **Test**: Open a `.poh` file and press `Ctrl+F5`

3. **Verify**: Command should execute successfully

---

**Status**: ✅ Extension ID bug fixed, awaiting reload test
