import unittest
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh
from Interpreter.poh_parser import ParseError, parse_program


def run_src(src: str):
    interp = Interpreter(input_fn=lambda _: "", output_fn=lambda _: None)
    interp.run(src, filename="<stdin>")
    return interp


class TestErrors(unittest.TestCase):
    
    def test_unknown_keyword_column(self):
        with self.assertRaises(ParseError) as e:
            parse_program(["   Writ 5"], filename="file.poh")
        msg = str(e.exception)
        # parse_program currently does not inject filename into ParseError prefix pipeline; default <stdin>
        # ParseError now uses capitalized 'Line'/'Col'
        self.assertTrue(msg.startswith('[<stdin>:Line 1:Col 4]'))
        self.assertIn('Writ', msg)


    def test_function_arity_with_line(self):
        src = """Make add with a, b
End
Use add with 1
"""
        interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
        with self.assertRaises(RuntimeErrorPoh) as e:
            interp.run(src, filename="test.poh")
        msg = str(e.exception)
        self.assertTrue(msg.startswith('[test.poh: Line 3'))  # call site line
        self.assertIn('expects 2 argument(s) but got 1', msg)
        self.assertIn('defined at line 1', msg)

    def test_type_mismatch_operator(self):
        src = "Write 5 - \"hi\""  # invalid '-' with string
        interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
        with self.assertRaises(RuntimeErrorPoh) as e:
            interp.run(src, filename="arith.poh")
        msg = str(e.exception)
        self.assertTrue(msg.startswith('[arith.poh: Line 1') and 'Type mismatch' in msg)

    def test_circular_import_message(self):
        import tempfile
        import os
        from pathlib import Path
        
        with tempfile.TemporaryDirectory() as tmp_dir:
            tmp_path = Path(tmp_dir)
            a = tmp_path / "a.poh"
            b = tmp_path / "b.poh"
            a.write_text("Import \"b.poh\"\n", encoding="utf-8")
            b.write_text("Import \"a.poh\"\n", encoding="utf-8")
            interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
            # change cwd to tmp_path so relative imports work
            old = os.getcwd()
            os.chdir(tmp_path)
            try:
                with self.assertRaises(RuntimeErrorPoh) as e:
                    interp.run_file(str(a))
                msg = str(e.exception)
                self.assertIn('Circular import detected', msg)
                self.assertTrue(msg.startswith('['))
            finally:
                os.chdir(old)

    def test_debug_trace_shows_statements(self):
        import io
        import sys
        
        src = """Debug on
Write 1
Debug off
Write 2
"""
        # Capture stdout
        captured_output = io.StringIO()
        sys.stdout = captured_output
        
        try:
            interp = Interpreter(output_fn=lambda s: print(s), input_fn=lambda _: "")
            interp.run(src, filename="dbg.poh")
            captured = captured_output.getvalue()
            # Updated trace format uses standardized prefix without [debug] tag for statement exec
            self.assertIn('[dbg.poh: Line 2: Col 1] Executing: WriteStmt', captured)
            self.assertIn('1', captured)
            self.assertIn('2', captured)
        finally:
            sys.stdout = sys.__stdout__


if __name__ == '__main__':
    unittest.main()

def test_divide_by_zero_prefix():
    src = "Write 5 / 0"
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="dz.poh")
    msg = str(e.value)
    assert msg.startswith('[dz.poh: Line 1') and 'divide by zero' in msg

def test_undefined_variable_prefix():
    src = "Write foo"
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="undef.poh")
    msg = str(e.value)
    assert msg.startswith('[undef.poh: Line 1') and "Undefined variable 'foo'" in msg

def test_list_index_out_of_range():
    # Build list then access out-of-range index
    src = "Set xs to List contains 1\nWrite xs at 5"
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="idx.poh")
    msg = str(e.value)
    assert msg.startswith('[idx.poh: Line 2') and 'out of range' in msg

def test_dict_key_not_found():
    src = 'Set d to Dictionary contains "a": 1\nWrite d at "b"'
    interp = Interpreter(output_fn=lambda _: None, input_fn=lambda _: "")
    with pytest.raises(RuntimeErrorPoh) as e:
        interp.run(src, filename="key.poh")
    msg = str(e.value)
    assert msg.startswith('[key.poh: Line 2') and 'was not found' in msg
