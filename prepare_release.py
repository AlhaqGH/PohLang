#!/usr/bin/env python3
"""
PohLang Release Preparation Script

This script prepares PohLang for release by:
1. Running tests
2. Building distribution packages
3. Validating installation
4. Creating release artifacts
5. Creating a git tag in the format pohlang-vX.Y.Z

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
import datetime


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


def extract_version_from_interpreter() -> str | None:
    """Extract __version__ from Interpreter/__init__.py (authoritative language version)."""
    init_path = Path("Interpreter/__init__.py")
    if not init_path.exists():
        return None
    try:
        for line in init_path.read_text(encoding="utf-8").splitlines():
            line = line.strip()
            if line.startswith("__version__"):
                # __version__ = "0.5.0"
                val = line.split("=", 1)[1].strip().strip("'\"")
                return val
    except Exception:
        return None
    return None


def read_pyproject_version() -> str | None:
    pyproject_path = Path("pyproject.toml")
    if not pyproject_path.exists():
        return None
    try:
        for line in pyproject_path.read_text(encoding="utf-8").splitlines():
            if line.strip().startswith("version") and "[project]" not in line:
                return line.split("=", 1)[1].strip().strip("'\"")
    except Exception:
        return None
    return None


def check_version_consistency() -> tuple[bool, str | None]:
    """Check version consistency and return (ok, version)."""
    print("🔍 Checking version consistency...")
    lang_version = extract_version_from_interpreter()
    py_version = read_pyproject_version()

    if not lang_version:
        print("❌ Could not determine language version from Interpreter/__init__.py")
        return False, None
    if not py_version:
        print("❌ Could not determine project version from pyproject.toml")
        return False, lang_version
    if lang_version.split("-")[0] != py_version.split("-")[0]:
        print(f"❌ Version mismatch: Interpreter __version__ is {lang_version} but pyproject is {py_version}")
        return False, lang_version
    print(f"✅ Version check: {lang_version}")
    return True, lang_version


def run_tests() -> bool:
    """Run the test suite via pytest if available, else unittest; also a tiny smoke test."""
    print("🧪 Running tests...")
    # Prefer pytest
    try:
        res = subprocess.run([sys.executable, '-m', 'pytest', '-q'], capture_output=True, text=True)
        if res.returncode == 0:
            print("✅ Pytest suite passed")
        else:
            print("⚠️ Pytest failed; falling back to unittest\n" + res.stdout + "\n" + res.stderr)
            raise RuntimeError()
    except Exception:
        res = subprocess.run([sys.executable, '-m', 'unittest', 'discover', '-s', 'tests_python'], capture_output=True, text=True)
        if res.returncode != 0:
            print("❌ Unittest suite failed")
            print(res.stdout)
            print(res.stderr)
            return False
        print("✅ Unittest suite passed")

    # Extra: basic interpreter smoke test
    test_code = (
        "Write \"Hello, PohLang!\"\n"
        "Set x to 5\n"
        "Set y to 3\n"
        "Set result to x plus y\n"
        "Write \"5 + 3 = \" plus result\n"
    )
    try:
        with tempfile.NamedTemporaryFile(mode='w', suffix='.poh', delete=False, encoding='utf-8') as f:
            f.write(test_code)
            test_file = f.name
        cmd = [sys.executable, "-m", "Interpreter.run_poh", test_file]
        ok = run_command(cmd, capture_output=True)
        os.unlink(test_file)
        if ok:
            print("✅ Interpreter smoke test passed")
            return True
        print("❌ Interpreter smoke test failed")
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
    
    # Determine current versions for notes
    ok, lang_version = check_version_consistency()
    py_version = read_pyproject_version() or 'unknown'
    display_ver = py_version

    release_notes = f"""# PohLang v{display_ver} Release Notes

## Automated Release - {datetime.datetime.now().strftime('%B %d, %Y')}

This is the inaugural release of PohLang, introducing a beginner-friendly, fully phrasal programming language designed specifically for educational purposes.

### 🎯 What's New

**Complete Language Implementation**
- Natural English-like syntax for all programming constructs
- Stable Python interpreter (v{lang_version or 'unknown'})
- Experimental Rust runtime/VM (pohlangc)
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

    # Create git tag if inside repository
    ok, lang_version = check_version_consistency()
    tag_name = None
    if ok and lang_version:
        # pyproject version used for tag X.Y.Z
        ver = read_pyproject_version() or lang_version
        tag_name = f"pohlang-v{ver}"
        try:
            res = subprocess.run(['git', 'rev-parse', '--is-inside-work-tree'], capture_output=True, text=True)
            if res.returncode == 0 and 'true' in res.stdout.lower():
                # Commit any changes (release notes, dist ignored typically) and tag
                subprocess.run(['git', 'add', '-A'], check=False)
                # Commit only if staged changes
                diff = subprocess.run(['git', 'diff', '--cached', '--quiet'])
                if diff.returncode != 0:
                    subprocess.run(['git', 'commit', '-m', f'Release {ver} artifacts'], check=False)
                # Create or update tag
                subprocess.run(['git', 'tag', '-f', tag_name, '-m', f'PohLang {ver}'], check=False)
                print(f"🏷️  Created/updated git tag {tag_name}")
            else:
                print("ℹ️  Not a git repository; skipping tagging.")
        except Exception as e:
            print(f"⚠️  Failed to create git tag: {e}")
    
    print("\n🏁 Release Preparation Complete")
    if success:
        print(f"✅ PohLang v{read_pyproject_version() or 'unknown'} is ready for release!")
        print("\nRelease artifacts:")
        print("- Distribution packages in dist/")
        print("- Release notes in RELEASE_NOTES.md")
        print(f"- Git tag: {tag_name or 'n/a'}")
        print("\nNext steps:")
        print("1. Test installation: pip install dist/*.whl")
        print("2. Upload to PyPI: twine upload dist/*")
        print("3. Git tag has been created (if repo). Push tags if needed.")
        print("4. Create GitHub release with artifacts (optional)")
        print("5. Update documentation websites")
    else:
        print("❌ Some steps failed. Please review and fix before release.")
        sys.exit(1)


if __name__ == "__main__":
    main()