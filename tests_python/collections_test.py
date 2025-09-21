import pytest

from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run(src: str):
    out = []
    interp = Interpreter(output_fn=lambda s: out.append(str(s)))
    interp.run(src, filename="collections_test.poh")
    return out


def test_immutable_list_mutation_raises():
    prog = """
    Make a list of 1, 2, and 3
    Set xs to it
    Add 4 to xs
    """.strip()
    with pytest.raises(RuntimeErrorPoh) as ei:
        run(prog)
    msg = str(ei.value)
    assert "Cannot modify immutable list" in msg
    assert msg.startswith("[") and "]" in msg


def test_mutable_list_mutation_ok():
    prog = """
    Make a mutable list of 1, 2, and 3
    Set xs to it
    Add 4 to xs
    Write length(xs)
    """.strip()
    out = run(prog)
    assert out[-1] == "4"


def test_immutable_dict_add_key_raises():
    prog = """
    Make a dictionary with "a" as 1 and "b" as 2
    Set d to it
    Add "c": 3 to d
    """.strip()
    with pytest.raises(RuntimeErrorPoh) as ei:
        run(prog)
    msg = str(ei.value)
    assert "Cannot modify immutable dictionary" in msg


def test_mutable_dict_add_key_ok():
    prog = """
    Make a mutable dictionary with "a" as 1 and "b" as 2
    Set d to it
    Add "c": 3 to d
    Write keys of d
    """.strip()
    out = run(prog)
    assert out[-1] == "['a', 'b', 'c']"


def test_deprecation_warning_for_legacy_list_literal():
    prog = """
    Set xs to List contains 1, 2
    Add 3 to xs
    """.strip()
    out = []
    interp = Interpreter(output_fn=lambda s: out.append(str(s)))
    interp.run(prog, filename="warn_test.poh")
    # Expect a warning printed
    joined = "\n".join(out)
    assert "Warning:" in joined and "Implicit mutable list is deprecated" in joined


def test_deprecation_warning_for_legacy_dict_literal():
    prog = """
    Set d to Dictionary contains "a": 1
    Add "b": 2 to d
    """.strip()
    out = []
    interp = Interpreter(output_fn=lambda s: out.append(str(s)))
    interp.run(prog, filename="warn_test.poh")
    joined = "\n".join(out)
    assert "Warning:" in joined and "Implicit mutable dictionary is deprecated" in joined
