import io
import sys
import pytest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def capture_output(path: str):
    buf = []
    def _out(msg: str):
        buf.append(msg)
    interp = Interpreter(output_fn=_out)
    interp.run_file(path)
    return "\n".join(buf)


def test_builtins_range_join_split_now(tmp_path):
    src = tmp_path / "prog.poh"
    src.write_text("""
Set nums to List contains 1,2,3
Write length(range(5))
Write join(nums, "-")
Write split("a,b,c", ",")
Write now()
""".strip())
    cap = capture_output(str(src))
    lines = cap.strip().splitlines()
    assert lines[0] == '5'
    assert lines[1] == '1-2-3'
    # third line prints Python list repr; ensure elements present
    assert '[' in lines[2] and 'a' in lines[2] and 'c' in lines[2]
    assert 'T' in lines[3]  # ISO timestamp contains 'T'


def test_import_suggestion(tmp_path):
    # Create an actual file
    real = tmp_path / "helpers.poh"
    real.write_text("Write \"ok\"")
    # Program that imports a misspelled file
    mainf = tmp_path / "main.poh"
    mainf.write_text("Import \"helper.poh\"")
    with pytest.raises(RuntimeErrorPoh) as ei:
        capture_output(str(mainf))
    assert 'Did you mean' in str(ei.value)