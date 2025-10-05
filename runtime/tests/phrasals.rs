use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn run(lines: &[&str]) -> Command {
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
fn count_of_list_and_string() {
    let mut cmd = run(&[
        "Write count of Make a list of 1, 2 and 3",
        "Write count of \"abc\"",
    ]);
    cmd.assert().success().stdout(predicate::str::contains("3"));
}

#[test]
fn join_with_separator() {
    let mut cmd = run(&[
        "Set xs to Make a list of \"a\", \"b\", \"c\"",
        "Write join xs with \"-\"",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a-b-c"));
}

#[test]
fn split_by_separator() {
    let mut cmd = run(&[
        "Set s to \"a,b,c\"",
        "Set parts to split s by ','",
        "Write first in parts",
        "Write last in parts",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a"))
        .stdout(predicate::str::contains("c"));
}

#[test]
fn contains_in_list_and_string() {
    let mut cmd = run(&[
        "Set nums to [1, 2, 3, 4]",
        "Write contains 3 in nums",
        "Write contains 5 in nums",
        "Set text to \"hello world\"",
        "Write contains 'world' in text",
        "Write contains 'xyz' in text",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("True"))
        .stdout(predicate::str::contains("False"));
}

#[test]
fn remove_from_list() {
    let mut cmd = run(&[
        "Set nums to [1, 2, 3, 2, 4]",
        "Set result to remove 2 from nums",
        "Write count of result",
        "Write first in result",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4"))
        .stdout(predicate::str::contains("1"));
}

#[test]
fn append_to_list() {
    let mut cmd = run(&[
        "Set nums to [1, 2, 3]",
        "Set result to append 4 to nums",
        "Write count of result",
        "Write last in result",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4"))
        .stdout(predicate::str::contains("4"));
}

#[test]
fn insert_at_index_in_list() {
    let mut cmd = run(&[
        "Set nums to [1, 2, 4]",
        "Set result to insert 3 at 2 in nums",
        "Write count of result",
        "Write result[2]",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("4"))
        .stdout(predicate::str::contains("3"));
}
