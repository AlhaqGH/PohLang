import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import pytest
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run_code(src: str):
    out_lines = []
    interp = Interpreter(output_fn=lambda m: out_lines.append(m))
    interp.run(src)
    return out_lines, interp


def test_inline_function_single_return():
    src = """
Make greet with name Write "Hello, " plus name
Write greet("World")
""".strip()
    out, _ = run_code(src)
    assert out[-1] == 'Hello, World'


def test_block_function_multiple_params_and_return():
    src = """
Make add with a, b
    Set temp to a plus b
    Return temp
End
Write add(3,4)
""".strip()
    out, _ = run_code(src)
    assert out[-1] == '7'


def test_recursion_factorial_with_shadowing():
    src = """
Set n to 3
Make fact with n
    If n is 0
        Return 1
    End
    Return n * fact(n - 1)
End
Write fact(5)
Write n  # ensure outer n not changed by recursion
""".strip()
    out, _ = run_code(src)
    assert out[-2] == '120'
    assert out[-1] == '3'


def test_local_variable_isolation_and_shadow():
    src = """
Set x to 10
Make demo with x
    Set x to x plus 5
    Return x
End
Write demo(2)
Write x
""".strip()
    out, _ = run_code(src)
    assert out[-2] == '7'  # local x = 2 + 5
    assert out[-1] == '10'  # global x unchanged


def test_use_stmt_invocation_and_side_effect():
    src = """
Set counter to 0
Make bump with amount
    Set counter to counter plus amount
    Return nothing
End
Use bump with 5
Use bump with 3
Write counter
""".strip()
    out, _ = run_code(src)
    assert out[-1] == '8'


def test_arity_mismatch_errors():
    src = """
Make foo with a, b
    Return a plus b
End
Write foo(1)
""".strip()
    with pytest.raises(RuntimeErrorPoh) as e:
        run_code(src)
    assert 'expects 2 argument(s) but got 1' in str(e.value)


def test_inline_function_arity_mismatch():
    src = """
Make greet with name, title Write "Hi " + title + " " + name
Write greet("OnlyOne")
""".strip()
    with pytest.raises(RuntimeErrorPoh) as e:
        run_code(src)
    assert "expects 2 argument(s) but got 1" in str(e.value)

