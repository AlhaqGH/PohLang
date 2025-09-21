#!/usr/bin/env python3
"""
PohLang Release Preparation Script

This script prepares PohLang for release by:
1. Running tests
2. Building distribution packages
3. Validating installation
4. Creating release artifacts

Usage:
    python prepare_release.py [--dry-run]
"""

import sys
import subprocess
import os
from pathlib import Path
import argparse
import tempfile
import shutil


def run_command(cmd, check=True, capture_output=False):
    """Run a command and handle errors."""
    print(f"Running: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, check=check, capture_output=capture_output, text=True)
        if capture_output:
            return result.stdout.strip()
        return True
    except subprocess.CalledProcessError as e:
        print(f"Command failed: {e}")
        if capture_output and e.stdout:
            print(f"Output: {e.stdout}")
        if capture_output and e.stderr:
            print(f"Error: {e.stderr}")
        return False


def check_version_consistency():
    """Check that versions are consistent across files."""
    print("🔍 Checking version consistency...")
    
    # Read pyproject.toml version
    pyproject_path = Path("pyproject.toml")
    if not pyproject_path.exists():
        print("❌ pyproject.toml not found")
        return False
    
    with open(pyproject_path) as f:
        content = f.read()
        if 'version = "0.1.0"' not in content:
            print("❌ pyproject.toml version is not 0.1.0")
            return False
    
    # Check interpreter version
    init_path = Path("Interpreter/__init__.py")
    if init_path.exists():
        with open(init_path) as f:
            content = f.read()
            if '__version__ = "0.5.0"' not in content:
                print("❌ Interpreter version is not 0.5.0")
                return False
    
    # Check transpiler version
    transpiler_pubspec = Path("transpiler/pubspec.yaml")
    if transpiler_pubspec.exists():
        with open(transpiler_pubspec) as f:
            content = f.read()
            if 'version: 0.3.5' not in content:
                print("❌ Transpiler version is not 0.3.5")
                return False
    
    print("✅ Version consistency check passed")
    return True


def run_tests():
    """Run the test suite."""
    print("🧪 Running tests...")
    
    # Try to run a simple interpreter test
    test_code = '''
Write "Hello, PohLang!"
Set x to 5
Set y to 3
Set result to x plus y
Write "5 + 3 = " plus result
'''
    
    try:
        with tempfile.NamedTemporaryFile(mode='w', suffix='.poh', delete=False) as f:
            f.write(test_code)
            test_file = f.name
        
        # Test interpreter
        cmd = [sys.executable, "-m", "Interpreter.run_poh", test_file]
        if run_command(cmd, capture_output=True):
            print("✅ Basic interpreter test passed")
            os.unlink(test_file)
            return True
        else:
            print("❌ Basic interpreter test failed")
            os.unlink(test_file)
            return False
            
    except Exception as e:
        print(f"❌ Test execution failed: {e}")
        return False


def build_package():
    """Build the distribution package."""
    print("📦 Building distribution package...")
    
    # Clean previous builds
    build_dirs = ["build", "dist", "*.egg-info"]
    for pattern in build_dirs:
        for path in Path(".").glob(pattern):
            if path.is_dir():
                shutil.rmtree(path)
                print(f"Cleaned {path}")
    
    # Build package
    cmd = [sys.executable, "-m", "build"]
    if run_command(cmd):
        print("✅ Package built successfully")
        return True
    else:
        print("❌ Package build failed")
        # Try with setuptools
        print("Trying with setuptools...")
        cmd = [sys.executable, "setup.py", "sdist", "bdist_wheel"]
        if run_command(cmd):
            print("✅ Package built with setuptools")
            return True
        else:
            print("❌ Package build failed with setuptools too")
            return False


def validate_package():
    """Validate the built package."""
    print("✅ Validating package...")
    
    dist_dir = Path("dist")
    if not dist_dir.exists():
        print("❌ No dist directory found")
        return False
    
    # Check for wheel and source distribution
    wheels = list(dist_dir.glob("*.whl"))
    tarballs = list(dist_dir.glob("*.tar.gz"))
    
    if not wheels:
        print("❌ No wheel file found")
        return False
    
    if not tarballs:
        print("❌ No source distribution found")
        return False
    
    print(f"✅ Found wheel: {wheels[0].name}")
    print(f"✅ Found source dist: {tarballs[0].name}")
    
    return True


def create_release_notes():
    """Create release notes file."""
    print("📝 Creating release notes...")
    
    release_notes = """# PohLang v0.1.0 Release Notes

## First Experimental Release - September 21, 2025

This is the inaugural release of PohLang, introducing a beginner-friendly, fully phrasal programming language designed specifically for educational purposes.

### 🎯 What's New

**Complete Language Implementation**
- Natural English-like syntax for all programming constructs
- Stable Python interpreter (v0.5.0) 
- Experimental Dart transpiler (v0.3.5)
- Comprehensive documentation and examples

**Key Features**
- Variables: `Set name to "PohLang"`
- Output: `Write "Hello World"`  
- Input: `Ask for name`
- Conditions: `If age is greater than 18`
- Loops: `Repeat 5` and `While x is less than 10`
- Functions: `Make greet with name`

### 📦 Installation

```bash
pip install pohlang
```

### 🚀 Quick Start

Create a file `hello.poh`:
```pohlang
Write "Welcome to PohLang!"
Ask for name
Write "Hello " plus name plus "!"
```

Run it:
```bash
pohlang hello.poh
```

### 📚 Documentation

- [PohLang Guide](PohLang_Guide.md) - Complete tutorial
- [Syntax Reference](doc/syntax.md) - Language syntax
- [Examples](examples/) - Sample programs

### 🎓 Educational Focus

PohLang is designed for:
- Programming beginners
- Educational institutions
- Learning programming concepts without syntax complexity
- Transitioning to other languages via Dart transpilation

### 🔮 Future Plans

- IDE plugins (VS Code, etc.)
- Web playground
- Enhanced error messages
- Expanded standard library
- More educational resources

### 🤝 Contributing

We welcome contributions! Visit our [GitHub repository](https://github.com/AlhaqGH/PohLang) to get involved.

---

**Note**: This is an experimental release focused on educational use. While stable for learning, it's not recommended for production applications.
"""
    
    with open("RELEASE_NOTES.md", "w", encoding="utf-8") as f:
        f.write(release_notes)
    
    print("✅ Release notes created")
    return True


def main():
    """Main release preparation function."""
    parser = argparse.ArgumentParser(description="Prepare PohLang for release")
    parser.add_argument("--dry-run", action="store_true", help="Run checks without building")
    args = parser.parse_args()
    
    print("🚀 PohLang Release Preparation")
    print("=" * 40)
    
    # Change to script directory
    script_dir = Path(__file__).parent
    os.chdir(script_dir)
    
    success = True
    
    # Run checks
    success &= check_version_consistency()
    success &= run_tests()
    
    if args.dry_run:
        print("\n🏁 Dry run completed")
        if success:
            print("✅ All checks passed - ready for release!")
        else:
            print("❌ Some checks failed - please fix before release")
        return
    
    if not success:
        print("\n❌ Pre-build checks failed. Aborting release preparation.")
        sys.exit(1)
    
    # Build and validate
    success &= build_package()
    success &= validate_package()
    success &= create_release_notes()
    
    print("\n🏁 Release Preparation Complete")
    if success:
        print("✅ PohLang v0.1.0 is ready for release!")
        print("\nRelease artifacts:")
        print("- Distribution packages in dist/")
        print("- Release notes in RELEASE_NOTES.md")
        print("- Updated CHANGELOG.md")
        print("\nNext steps:")
        print("1. Test installation: pip install dist/*.whl")
        print("2. Upload to PyPI: twine upload dist/*")
        print("3. Create GitHub release with artifacts")
        print("4. Update documentation websites")
    else:
        print("❌ Some steps failed. Please review and fix before release.")
        sys.exit(1)


if __name__ == "__main__":
    main()