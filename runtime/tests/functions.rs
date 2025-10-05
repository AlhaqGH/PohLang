use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn run(lines: &[&str]) -> Command {
    // Create a persistent temp file path (avoid auto-delete on drop issues on Windows)
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("pohlang_test_{}.poh", ts));
    let mut contents = String::from("Start Program\n");
    for l in lines {
        contents.push_str(l);
        contents.push('\n');
    }
    contents.push_str("End Program\n");
    fs::write(&path, contents).unwrap();
    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd
}

#[test]
fn define_function_inline_and_call_with_and() {
    let mut cmd = run(&[
        "Define function add with parameters x, y as x plus y",
        "Write add with 2 and 3",
    ]);
    cmd.assert().success().stdout(predicate::str::contains("5"));
}

#[test]
fn default_param_with_defaulting_to() {
    let mut cmd = run(&[
        "Define function greet with parameter name defaulting to \"World\" as \"Hello \" plus name",
        "Write greet",
        "Write greet with \"Ada\"",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello World"))
        .stdout(predicate::str::contains("Hello Ada"));
}

#[test]
#[cfg_attr(target_os = "macos", ignore)]
fn closure_captures_variable_define_form() {
    let mut cmd = run(&[
        "Set base to 10",
        "Define function makeAdder with parameter y as base plus y",
        "Write makeAdder with 5",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("15"));
}

#[test]
fn call_alias_works() {
    let mut cmd = run(&[
        "Define function shout with parameter x as x plus x",
        "Call shout with \"Ha\"",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("HaHa"));
}

#[test]
fn wrong_arity_reports_error() {
    let mut cmd = run(&[
        "Define function pair with parameters a, b as a plus b",
        "Write pair with 1",
    ]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("expects"));
}

#[test]
fn unknown_function_reports_error() {
    let mut cmd = run(&["Write nope with 1"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Function 'nope' is not defined"));
}
