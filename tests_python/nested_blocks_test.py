import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh
from Interpreter.poh_parser import ParseError


def run_code(src: str):
    out = []
    interp = Interpreter(output_fn=lambda m: out.append(m))
    interp.run(src)
    return out


def test_deeply_nested_if_repeat_while():
    src = """
Set a to 1
If a is 1
    Set b to 2
    If b is 2
        Set c to 3
        Repeat 2
            Set d to 4
            While d is greater than 0
                Write a plus b plus c plus d
                Set d to d - 2
            End
        End
    End
End
""".strip()
    out = run_code(src)
    # Repeat 2 iterations * While loop prints twice per iteration (d=4 then d=2) => 4 lines
    # a+b+c+d = 1+2+3+4=10 then 1+2+3+2=8 repeating per repeat iteration
    assert out == ['10','8','10','8']


def test_variable_shadowing_blocks():
    src = """
Set x to 5
If x is 5
    Set x to 6
    Write x
    If x is 6
        Set x to 7
        Write x
    End
    Write x
End
Write x
""".strip()
    out = run_code(src)
    # Because assignment inside block should update same variable (no intentional shadowing keyword), all writes show progressive updates
    assert out == ['6','7','7','7']


def test_explicit_shadowing_via_inner_new_name():
    src = """
Set x to 1
If x is 1
    Set y to 2
    If y is 2
        Set y to 5
        Write y
    End
    Write y
End
""".strip()
    out = run_code(src)
    # two prints from inner and after inner
    assert out == ['5','5']
    # Accessing y now should fail
    try:
        run_code(src + "\nWrite y")
    except RuntimeErrorPoh:
        pass
    else:
        raise AssertionError("Expected undefined variable error for y after If")


def test_repeat_iteration_scope_it_does_not_leak():
    src = """
Set nums to List contains 1,2,3
Repeat nums
    Write it
End
Write it
""".strip()
    # first part prints 1 2 3 then final Write it should error
    out = []
    interp = Interpreter(output_fn=lambda m: out.append(m))
    try:
        interp.run(src)
    except RuntimeErrorPoh as e:
        # ensure three loop outputs captured
        assert out == ['1','2','3']
    else:
        raise AssertionError("Expected 'it' to be undefined outside loop")


def test_while_loop_variable_updates_outer():
    src = """
Set n to 0
While n is less than 3
    Set n to n + 1
End
Write n
""".strip()
    out = run_code(src)
    assert out == ['3']
