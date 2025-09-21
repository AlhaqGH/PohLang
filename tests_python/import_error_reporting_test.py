import pytest, textwrap, os
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def test_error_location_inside_import(tmp_path):
    bad = tmp_path / 'bad.poh'
    # Reference undefined variable inside imported file
    bad.write_text('Write missingVar\n', encoding='utf-8')
    main_src = 'Import "bad.poh"\n'
    interp = Interpreter(output_fn=lambda s: None, input_fn=lambda _: "")
    cwd = os.getcwd()
    os.chdir(tmp_path)
    try:
        with pytest.raises(RuntimeErrorPoh) as e:
            interp.run(main_src, filename='main.poh')
        msg = str(e.value)
        # Should reference bad.poh in prefix
        assert msg.startswith('[%s' % str(bad)) or ('[bad.poh:' in msg)
        assert "Undefined variable 'missingVar'" in msg
    finally:
        os.chdir(cwd)
