import pytest
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh
from Interpreter.poh_parser import ParseError, parse_program


def run_src(src: str):
    interp = Interpreter(input_fn=lambda _: "", output_fn=lambda _: None)
    interp.run(src, filename="<stdin>")
    return interp

def test_unknown_keyword_column():
    with pytest.raises(ParseError) as e:
        parse_program(["   Writ 5"], filename="file.poh")
    assert 'col 4' in str(e.value) or 'col 4' in repr(e.value)
    assert 'Writ' in str(e.value)


def test_function_arity_with_line():
    src = """Make add with a, b
End
Use add with 1
"""
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="test.poh")
    msg = str(e.value)
    assert 'expects 2 argument(s) but got 1' in msg
    assert 'defined at line 1' in msg


def test_type_mismatch_operator():
    src = "Write 5 - \"hi\""  # invalid '-' with string
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="arith.poh")
    assert 'Type mismatch' in str(e.value)


def test_circular_import_message(tmp_path):
    a = tmp_path / "a.poh"
    b = tmp_path / "b.poh"
    a.write_text("Import \"b.poh\"\n", encoding="utf-8")
    b.write_text("Import \"a.poh\"\n", encoding="utf-8")
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    # change cwd to tmp_path so relative imports work
    import os
    old = os.getcwd()
    os.chdir(tmp_path)
    try:
        with pytest.raises(RuntimeErrorPoh) as e:
            interp.run_file(str(a))
        assert 'Circular import detected' in str(e.value)
    finally:
        os.chdir(old)


def test_debug_trace_shows_statements(capsys):
    src = """Debug on
Write 1
Debug off
Write 2
"""
    interp = Interpreter(output_fn=lambda s: print(s), input_fn=lambda _: "")
    interp.run(src, filename="dbg.poh")
    captured = capsys.readouterr().out
    assert '[debug] Executing WriteStmt at line 2' in captured
    assert '1' in captured and '2' in captured
