import os
import io
import sys
import textwrap

from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run(src: str):
    out = io.StringIO()
    interp = Interpreter(output_fn=lambda s: print(s, file=out))
    interp.run(src)
    return out.getvalue().strip()


def test_default_params_and_calls():
    src = textwrap.dedent(
        '''
        Make function greet with name set to "World"
            Write "Hello " + name
        End

        Call greet
        Call greet with "Alice"
        '''
    )
    out = run(src).splitlines()
    assert out[0] == 'Hello World'
    assert out[1] == 'Hello Alice'


def test_closure_and_return_function():
    src = textwrap.dedent(
        '''
        Make function make_adder with a set to 1
            Make function inner with b set to 0
                Return a + b
            End
            Set f to inner
            Return f
        End

        Set add2 to make_adder(2)
        Write add2(3)
        '''
    )
    out = run(src)
    assert out == '5'


def test_call_value_statement():
    src = textwrap.dedent(
        '''
        Make function say with x set to "hi"
            Write x
        End
        Set f to say
        Call f with "yo"
        '''
    )
    out = run(src)
    assert out == 'yo'


def test_hybrid_imports_system_and_local(tmp_path):
    # Local file
    local = tmp_path / 'm.poh'
    local.write_text(textwrap.dedent('''
        Make function twice with n set to 1
            Return n + n
        End
    '''))

    src = textwrap.dedent(f'''
        Import "{local}"
        Import system "collections"
        Write twice(3)
    ''')
    out = run(src)
    assert out == '6'


def test_errors():
    # Unknown function
    src = 'Use grete'
    try:
        run(src)
    except RuntimeErrorPoh as e:
        msg = str(e)
        assert 'Unknown function' in msg or "I can't find a function" in msg

    # Wrong arg count
    src = textwrap.dedent('''
        Make function add with a, b
            Return a + b
        End
        Use add with 1
    ''')
    try:
        run(src)
    except RuntimeErrorPoh as e:
        assert 'expects at least' in str(e) or 'expects' in str(e)
