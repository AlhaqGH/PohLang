import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run_code(src: str):
    out = []
    interp = Interpreter(output_fn=lambda m: out.append(m))
    interp.run(src)
    return out


def test_begin_block_scoped_variable():
    src = """
Set x to 1
Begin
    Set y to 2
    Write x + y
End
Write x
""".strip()
    out = run_code(src)
    assert out == ['3','1']
    # y should be undefined now
    try:
        run_code(src + "\nWrite y")
    except RuntimeErrorPoh:
        pass
    else:
        raise AssertionError("Expected y to be undefined outside Begin block")


def test_begin_block_outer_mutation():
    src = """
Set total to 5
Begin
    Set total to total + 10
End
Write total
""".strip()
    out = run_code(src)
    assert out == ['15']


def test_nested_begin_blocks():
    src = """
Set a to 1
Begin
    Set a to a + 1
    Begin
        Set a to a + 2
        Write a
    End
    Write a
End
Write a
""".strip()
    out = run_code(src)
    # inner: a becomes 1+1+2=4, then after inner still 4, after outer End still 4
    assert out == ['4','4','4']


def test_begin_inside_if():
    src = """
Set flag to 1
If flag is 1
    Begin
        Set z to 9
        Write z
    End
End
Write flag
""".strip()
    out = run_code(src)
    assert out == ['9','1']


def test_begin_return_inside_function():
    src = """
Make foo with n
    Begin
        If n is greater than 0
            Return n + 1
        End
        Return 0
    End
End
Write foo(5)
Write foo(0)
""".strip()
    out = run_code(src)
    assert out == ['6','0']
