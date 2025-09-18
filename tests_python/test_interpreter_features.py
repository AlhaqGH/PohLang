import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import io
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run_code(src: str, inputs=None):
    inputs = inputs or []
    buf_in = list(inputs)
    out_lines = []
    def _in(prompt: str):
        return buf_in.pop(0) if buf_in else ''
    def _out(msg: str):
        out_lines.append(msg)
    interp = Interpreter(input_fn=_in, output_fn=_out)
    interp.run(src)
    return out_lines


def test_nested_blocks_and_scopes():
    src = """
Set x to 1
If x is 1
    Set y to 2
    If y is 2
        Set z to 3
        Write z
    End
    Write y
End
Write x
""".strip()
    out = run_code(src)
    assert out == ['3','2','1']


def test_function_recursion_factorial():
    src = """
Make fact with n
    If n is 0
        Return 1
    End
    Return n * fact(n - 1)
End
Write fact(5)
""".strip()
    out = run_code(src)
    assert out[-1] == '120'


def test_repeat_over_list_and_it():
    src = """
Set total to 0
Set nums to List contains 1,2,3,4
Repeat nums
    Set total to total plus it
End
Write total
""".strip()
    out = run_code(src)
    assert out[-1] == '10'


def test_dictionary_and_access():
    src = """
Set d to Dictionary contains "a": 1, "b": 2
Write d at "a" plus d at "b"
""".strip()
    out = run_code(src)
    assert out[-1] == '3'


def test_import_and_circular_prevention(tmp_path):
    a = tmp_path / 'a.poh'
    b = tmp_path / 'b.poh'
    a.write_text('Import "b.poh"\nWrite "A"\n', encoding='utf-8')
    b.write_text('Import "a.poh"\nWrite "B"\n', encoding='utf-8')
    # run a and expect circular import runtime error
    buf = []
    def _out(m): buf.append(m)
    interp = Interpreter(output_fn=_out)
    try:
        interp.run_file(str(a))
    except RuntimeErrorPoh as e:
        assert 'Circular import detected' in str(e)
    else:
        raise AssertionError('Expected circular import error')


def test_error_line_reporting():
    src = """
Write 1
Write 2
Set x to y plus 1
Write 3
""".strip()
    try:
        run_code(src)
    except RuntimeErrorPoh as e:
        assert 'y' in str(e)
    else:
        raise AssertionError('Expected undefined variable error')
