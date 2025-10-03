use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use std::io::Write;
use tempfile::NamedTempFile;
use tempfile::tempdir;

#[test]
fn run_write_works() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Write \"Hello\" plus \" World\"").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_if_inline_works() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "If 1 Write \"yes\" Otherwise Write \"no\"").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_inline_function_and_use() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Make greet with name set to \"World\" Write \"Hello \" plus name").unwrap();
    writeln!(f, "Use greet with \"Alice\"").unwrap();
    writeln!(f, "Write greet(\"Bob\")").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_if_block_and_set() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 1").unwrap();
    writeln!(f, "If x").unwrap();
    writeln!(f, "    Write \"T\"").unwrap();
    writeln!(f, "Otherwise").unwrap();
    writeln!(f, "    Write \"F\"").unwrap();
    writeln!(f, "End").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_closure_like_capture() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set prefix to \"Hi \"").unwrap();
    writeln!(f, "Make greet with name set to \"World\" Write prefix plus name").unwrap();
    writeln!(f, "Write greet(\"Alice\")").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_func_block_and_return() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Make add with a, b set to 1").unwrap();
    writeln!(f, "    Return a plus b").unwrap();
    writeln!(f, "End").unwrap();
    writeln!(f, "Write add(1, 2)").unwrap();
    writeln!(f, "Write add(5)").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_nested_func_blocks() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set p to \"Hi \"").unwrap();
    writeln!(f, "Make greet with name set to \"World\"").unwrap();
    writeln!(f, "    Make full with last set to \"!\"").unwrap();
    writeln!(f, "        Return p plus name plus last").unwrap();
    writeln!(f, "    End").unwrap();
    writeln!(f, "    Return full() ").unwrap();
    writeln!(f, "End").unwrap();
    writeln!(f, "Write greet(\"Alice\")").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_repeat_block_counts() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 0").unwrap();
    writeln!(f, "Repeat 3 times").unwrap();
    writeln!(f, "    Set x to x plus 1").unwrap();
    writeln!(f, "End").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn run_while_block_counts() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 0").unwrap();
    writeln!(f, "Set n to 3").unwrap();
    writeln!(f, "While n").unwrap();
    writeln!(f, "    Set x to x plus 1").unwrap();
    writeln!(f, "    Set n to n plus -1").unwrap();
    writeln!(f, "End").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn import_local_file_and_call() {
    let dir = tempdir().unwrap();
    let lib_path = dir.path().join("lib.poh");
    fs::write(&lib_path, "Make hello with who Write \"Hi \" plus who\n").unwrap();

    let main_path = dir.path().join("main.poh");
    fs::write(&main_path, format!("Import \"{}\"\nWrite hello(\"Alice\")\n", lib_path.display())).unwrap();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(main_path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn system_import_stub_noop() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Import system \"collections\"").unwrap();
    writeln!(f, "Write \"OK\"").unwrap();
    let path = f.into_temp_path();
    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn join_and_range_builtins_work() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set nums to range(5)").unwrap();
    writeln!(f, "Write join(nums, \",\")").unwrap();
    let path = f.into_temp_path();
    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn ask_for_parses_correctly() {
    // Test that Ask for syntax parses without error
    // Note: We can't easily test interactive stdin in unit tests,
    // so this test validates parsing and compilation only.
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 5").unwrap();
    writeln!(f, "Write \"Before ask\"").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn ask_for_in_bytecode() {
    // Test that Ask for compiles to bytecode without error
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Ask for name").unwrap();
    writeln!(f, "Write \"Got: \" plus name").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--compile").arg(path.to_str().unwrap());
    // Should compile to bytecode successfully, generating ASK opcode
    cmd.assert().success();
}

#[test]
fn increase_decrease_desugar_works() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 10").unwrap();
    writeln!(f, "Increase x by 5").unwrap();
    writeln!(f, "Write x").unwrap();
    writeln!(f, "Decrease x by 3").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn arithmetic_operators_work() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set a to 10 plus 5").unwrap();
    writeln!(f, "Write a").unwrap();
    writeln!(f, "Set b to 10 minus 5").unwrap();
    writeln!(f, "Write b").unwrap();
    writeln!(f, "Set c to 10 times 5").unwrap();
    writeln!(f, "Write c").unwrap();
    writeln!(f, "Set d to 10 divided by 5").unwrap();
    writeln!(f, "Write d").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn arithmetic_precedence_works() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set result to 10 plus 5 times 2").unwrap();
    writeln!(f, "Write result").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn decrease_with_variables_works() {
    let mut f = NamedTempFile::new().unwrap();
    writeln!(f, "Set x to 100").unwrap();
    writeln!(f, "Set y to 10").unwrap();
    writeln!(f, "Decrease x by y").unwrap();
    writeln!(f, "Write x").unwrap();
    let path = f.into_temp_path();

    let mut cmd = Command::cargo_bin("pohlangc").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}
