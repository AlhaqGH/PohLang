import sys, textwrap, os
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import pytest
from Interpreter.poh_interpreter import Interpreter, RuntimeErrorPoh


def run(src: str, base_dir: Path):
    out = []
    interp = Interpreter(output_fn=lambda m: out.append(m))
    # Write temp main file and run via run to simulate top-level (imports relative)
    main_path = base_dir / 'main_temp.poh'
    main_path.write_text(src, encoding='utf-8')
    interp.run_file(str(main_path))
    return out, interp


def test_simple_import(tmp_path: Path):
    lib = tmp_path / 'lib.poh'
    lib.write_text('Set libVar to 42\n', encoding='utf-8')
    main_src = 'Import "lib.poh"\nWrite 1'
    out, interp = run(main_src, tmp_path)
    assert out[-1] == '1'
    # libVar should not be in globals directly
    with pytest.raises(RuntimeErrorPoh):
        _ = interp.globals.get('libVar')
    assert 'lib' in interp.modules
    assert interp.modules['lib']['libVar'] == 42


def test_imported_function_access(tmp_path: Path):
    lib = tmp_path / 'mathlib.poh'
    lib.write_text(textwrap.dedent('''\
    Make add with a, b
        Return a + b
    End
    '''), encoding='utf-8')
    main_src = 'Import "mathlib.poh"\nWrite add(3,4)'
    out, _ = run(main_src, tmp_path)
    assert out[-1] == '7'


def test_relative_nested_import(tmp_path: Path):
    sub = tmp_path / 'sub'
    sub.mkdir()
    inner = sub / 'inner.poh'
    inner.write_text('Set innerVar to 5\n', encoding='utf-8')
    outer = tmp_path / 'outer.poh'
    # outer imports sub/inner.poh relatively
    outer.write_text('Import "sub/inner.poh"\nSet outerVar to innerVar + 1\nWrite outerVar\n', encoding='utf-8')
    main_src = 'Import "outer.poh"'
    out, interp = run(main_src, tmp_path)
    # innerVar not global; outerVar defined inside outer module only
    assert out[-1] == '6'
    with pytest.raises(RuntimeErrorPoh):
        _ = interp.globals.get('innerVar')
    with pytest.raises(RuntimeErrorPoh):
        _ = interp.globals.get('outerVar')
    assert 'outer' in interp.modules and 'sub' not in interp.modules  # sub module name is inner, not directory
    assert 'inner' in interp.modules
    assert interp.modules['inner']['innerVar'] == 5


def test_circular_import_detection(tmp_path: Path):
    a = tmp_path / 'a.poh'
    b = tmp_path / 'b.poh'
    a.write_text('Import "b.poh"\nWrite "A"\n', encoding='utf-8')
    b.write_text('Import "a.poh"\nWrite "B"\n', encoding='utf-8')
    main_src = 'Import "a.poh"'
    with pytest.raises(RuntimeErrorPoh) as e:
        run(main_src, tmp_path)
    assert 'Circular import detected' in str(e.value)


def test_idempotent_import(tmp_path: Path):
    lib = tmp_path / 'once.poh'
    lib.write_text('Write "Loaded"\n', encoding='utf-8')
    main_src = 'Import "once.poh"\nImport "once.poh"'
    out, _ = run(main_src, tmp_path)
    # Should only print Loaded once
    assert out.count('Loaded') == 1
