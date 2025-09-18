import subprocess
import tempfile
import shutil
import os
from pathlib import Path

# Project root (two levels up from this file)
ROOT = Path(__file__).resolve().parents[1]

# Locate Dart executable robustly (works in constrained test environments).
DART_EXECUTABLE = shutil.which("dart") or os.environ.get("DART_PATH")
DART_AVAILABLE = DART_EXECUTABLE is not None


def run_cmd(cmd, cwd=None, input_text=None):
    proc = subprocess.Popen(
        cmd,
        cwd=cwd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    out, err = proc.communicate(input=input_text, timeout=30)
    return proc.returncode, out, err


def transpile_and_run(poh_source: str, input_text: str = ""):
    """Transpile a PohLang source snippet and run produced Dart.

    Uses the package executable (dart run bin/pohlang.dart ...) instead of a
    now-removed root/src path.
    """
    if not DART_AVAILABLE:
        # Return special marker so tests can treat as skip.
        return "__SKIP_NO_DART__"

    with tempfile.TemporaryDirectory() as td:
        td_path = Path(td)
        poh = td_path / "main.poh"
        dart_out = td_path / "main.dart"
        poh.write_text(poh_source, encoding="utf-8")
        # Transpile only
        code, out, err = run_cmd([
            DART_EXECUTABLE,
            "run",
            "bin/pohlang.dart",
            str(poh),
            "--no-run",
            "-o",
            str(dart_out),
        ], cwd=str(ROOT))
        assert code == 0, f"Transpile failed: {err}"
        # Run generated Dart
        code, out, err = run_cmd([DART_EXECUTABLE, "run", str(dart_out)], cwd=str(ROOT), input_text=input_text)
        assert code == 0, f"Run failed: {err}"
        return out


def test_write_and_arithmetic():
    src = """
Write 1 plus 2 times 3
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "7"


def test_if_else_block():
    src = """
Set age to 20
If age is greater than 18
    Write "adult"
Otherwise
    Write "minor"
End
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "adult"


def test_while_and_repeat():
    src = """
Set x to 0
Repeat 3
    Set x to x plus 2
End
While x is greater than 0
    Set x to x minus 1
End
Write x
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "0"


def test_functions_and_use():
    src = """
Make add2 with a, b
    Return a plus b
End
Use add2 with 3, 4
""".strip()
    out = transpile_and_run(src)
    # Nothing printed (Use doesn't print); extend: call via Write
    src2 = """
Make add2 with a, b
    Return a plus b
End
Set r to add2(3, 4)
Write r
""".strip()
    out = transpile_and_run(src2)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "7"


def test_input_number_and_decimal():
    src = """
Ask for n number
Ask for d decimal
Write n plus d
""".strip()
    out = transpile_and_run(src, input_text="5\n2.5\n")
    if out == "__SKIP_NO_DART__":
        return
    lines = [ln.strip() for ln in out.splitlines() if ln.strip()]
    # Prompts are inline (Enter n:, Enter d:) so numeric result is last token
    last_token = lines[-1].split()[-1]
    assert last_token == "7.5", f"Expected 7.5, got {last_token!r}. Full output: {out!r}"


def test_comparisons_and_logic():
    src = """
Set a to 5
Set b to 10
If a is less than b And b is at least 10
    Write "ok"
Otherwise
    Write "no"
End
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "ok"


def test_increase_decrease_and_inline_if():
    src = """
Set x to 1
Increase x by 4
Decrease x by 2
If x is 3 Write "three" Otherwise Write "no"
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "three"


def test_import_file():
    # Create a temp imported file that sets and writes a value
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        td_path = Path(td)
        imported = td_path / "lib.poh"
        imported.write_text("Write \"from import\"\n", encoding="utf-8")
        src = f"""
Import "{imported.name}"
""".strip()
        (td_path / "main.poh").write_text(src, encoding="utf-8")

        if not DART_AVAILABLE:
            return

        def transpile_custom():
            code, out, err = run_cmd([
                DART_EXECUTABLE,
                "run",
                "bin/pohlang.dart",
                str(td_path / "main.poh"),
            ], cwd=str(ROOT))
            assert code == 0, f"Transpile+run failed: {err}"
            return out

        out = transpile_custom()
        assert "from import" in out


def test_booleans_true_false():
    src = """
Set b to true
If b Write "yes" Otherwise Write "no"
""".strip()
    out = transpile_and_run(src)
    if out == "__SKIP_NO_DART__":
        return
    assert out.strip().splitlines()[-1] == "yes"


if __name__ == "__main__":
    # Simple manual runner to avoid requiring pytest dependency.
    passed = 0
    failed = 0
    skipped = 0
    if not DART_AVAILABLE:
        print("[INFO] Dart executable not found; E2E tests skipped. Set DART_PATH or add dart to PATH to run them.")
    for name, fn in sorted(globals().items()):
        if name.startswith("test_") and callable(fn):
            try:
                fn()
                # Detect skip by absence of dart
                if not DART_AVAILABLE:
                    print(f"[SKIP] {name}")
                    skipped += 1
                else:
                    print(f"[PASS] {name}")
                    passed += 1
            except Exception as e:  # noqa: BLE001
                failed += 1
                print(f"[FAIL] {name}: {e}")
    if DART_AVAILABLE:
        print(f"Python E2E tests: {passed} passed, {failed} failed")
    else:
        print(f"Python E2E tests: {skipped} skipped (dart not available)")
    if failed and DART_AVAILABLE:
        raise SystemExit(1)
