import re
import pytest
from Interpreter.poh_interpreter import Interpreter


def collect_output(src: str, debug: bool = False, filename: str = "trace.poh"):
    out_lines = []
    interp = Interpreter(output_fn=lambda s: out_lines.append(s), input_fn=lambda _: "")
    interp.debug_enabled = debug
    interp.run(src, filename=filename)
    return out_lines


def test_statement_and_assignment_tracing():
    src = """Set x to 1\nIncrease x by 2\nWrite x\n"""
    lines = collect_output(src, debug=True)
    # Expect execution lines and variable sets
    joined = "\n".join(lines)
    assert "Executing: SetStmt" in joined
    assert re.search(r"Set variable 'x' = 1", joined)
    assert re.search(r"Set variable 'x' = 3", joined)  # after increase
    assert any(l.endswith('3') for l in lines)  # final Write output


def test_function_call_and_return_tracing():
    src = """Make add with a, b\n\tReturn a + b\nEnd\nUse add with 2,3\n"""
    lines = collect_output(src, debug=True, filename="fn.poh")
    joined = "\n".join(lines)
    assert re.search(r"Enter function add\(a=2, b=3\)", joined)
    assert re.search(r"Return 5", joined)


def test_import_tracing(tmp_path):
    mod = tmp_path / "lib.poh"
    mod.write_text("Set y to 5\n", encoding="utf-8")
    main = f"Import \"{mod.name}\"\nWrite 1\n"
    # chdir to tmp_path for relative import resolution
    import os
    old = os.getcwd()
    os.chdir(tmp_path)
    try:
        lines = collect_output(main, debug=True, filename="main.poh")
    finally:
        os.chdir(old)
    joined = "\n".join(lines)
    assert re.search(r"\[import: lib.poh\]", joined)
    # normal output present
    assert any(l.endswith('1') for l in lines)


def test_debug_disabled_silent():
    src = "Set a to 1\nWrite a\n"
    lines = collect_output(src, debug=False)
    # Should only contain actual program output (the value 1)
    assert lines == ['1']
