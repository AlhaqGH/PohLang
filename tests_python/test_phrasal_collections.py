import unittest

from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run(src: str):
    out = []
    interp = Interpreter(output_fn=lambda s: out.append(str(s)))
    interp.run(src, filename="test_case.poh")
    return out


class TestPhrasalCollections(unittest.TestCase):

    def test_list_nth_set_remove_and_length(self):
        prog = """
        Make a mutable list of 10, 20, and 30
        Set xs to it
        Write Take the 2 item from xs
        Set the 2 item in xs to 99
        Write Take the 2 item from xs
        Remove the last item from xs
        Write length(xs)
        """.strip()
        out = run(prog)
        self.assertEqual(out, ["20", "99", "2"])

    def test_list_nth_out_of_range_error(self):
        prog = """
        Make a list of 1, 2
        Set xs to it
        Write Take the 3 item from xs
        """.strip()
        with pytest.raises(RuntimeErrorPoh) as ei:
            run(prog)
        msg = str(ei.value)
        assert "Index 3 is out of range for the list." in msg
        assert msg.startswith("[") and "]" in msg  # prefixed


    def test_set_nth_type_errors(self):
        prog = """
        Set xs to 123
        Set the 1 item in xs to 5
        """.strip()
        with pytest.raises(RuntimeErrorPoh) as ei:
            run(prog)
        assert "expects a list" in str(ei.value)

        prog2 = """
        Make a mutable list of 1,2,3
        Set xs to it
        Set the a item in xs to 5
        """.strip()
        with pytest.raises(RuntimeErrorPoh) as ei2:
            run(prog2)
        assert "Index must be a number" in str(ei2.value)


    def test_remove_last_errors_and_ok(self):
        prog = """
        Make a mutable list of
        Set xs to it
        Remove the last item from xs
        """.strip()
        # removing from empty list is no-op
        run(prog)

        prog2 = """
        Set xs to 5
        Remove the last item from xs
        """.strip()
        with pytest.raises(RuntimeErrorPoh) as ei:
            run(prog2)
        assert "expects a list" in str(ei.value)


    def test_dict_make_keys_values_contains_and_take_value(self):
        prog = """
        Make a dictionary with "a" as 1 and "b" as 2
        Set d to it
        Write keys of d
        Write values of d
        Write contains "a" in d
        Write Take the value of "b" from d
        """.strip()
        out = run(prog)

        # keys/values order for dict is insertion order in Python 3.7+
        assert out[0] == "['a', 'b']"
        assert out[1] == "[1, 2]"
        assert out[2] == "True"
        assert out[3] == "2"


if __name__ == '__main__':
    unittest.main()
