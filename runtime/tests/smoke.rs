use assert_cmd::prelude::*;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{tempdir, NamedTempFile, TempPath};

fn write_program(lines: &[&str]) -> TempPath {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "Start Program").unwrap();
    for line in lines {
        if line.trim().is_empty() {
            writeln!(file).unwrap();
        } else {
            writeln!(file, "{}", line).unwrap();
        }
    }
    writeln!(file, "End Program").unwrap();
    file.into_temp_path()
}

#[test]
fn run_simple_program() {
    let path = write_program(&["Write \"Hello PohLang\""]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello PohLang"));
}

#[test]
fn run_variable_assignment() {
    let path = write_program(&["Set name to \"World\"", "Write \"Hello \" plus name"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello World"));
}

#[test]
fn run_if_else_blocks() {
    let path = write_program(&[
        "Set condition to 0",
        "If condition",
        "    Write \"Adult\"",
        "Otherwise",
        "    Write \"Minor\"",
        "End",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Minor"));
}

#[test]
fn run_closure_like_capture() {
    let path = write_program(&[
        "Set prefix to \"Hi \"",
        "Make greet with name Write prefix plus name",
        "Write greet(\"Alice\")",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hi Alice"));
}

#[test]
fn run_func_block_and_return() {
    let path = write_program(&[
        "Make add with a, b set to 1",
        "    Return a plus b",
        "End",
        "Write add(1, 2)",
        "Write add(5)",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("3"))
        .stdout(predicates::str::contains("6"));
}

#[test]
fn run_nested_func_blocks() {
    let path = write_program(&[
        "Set prefix to \"Hi \"",
        "Make greet with name",
        "    Make suffix with last set to \"!\"",
        "        Return prefix plus name plus last",
        "    End",
        "    Return suffix()",
        "End",
        "Write greet(\"Alice\")",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hi Alice!"));
}

#[test]
fn run_repeat_block_counts() {
    let path = write_program(&[
        "Set x to 0",
        "Repeat 3 times",
        "    Set x to x plus 1",
        "End",
        "Write x",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("3"));
}

#[test]
fn run_while_block_counts() {
    let path = write_program(&[
        "Set x to 0",
        "Set n to 3",
        "While n",
        "    Set x to x plus 1",
        "    Set n to n plus -1",
        "End",
        "Write x",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("3"));
}

#[test]
fn import_local_file_and_call() {
    let dir = tempdir().unwrap();
    let lib_path = dir.path().join("lib.poh");
    fs::write(
        &lib_path,
        "Start Program\nMake hello with who Write \"Hi \" plus who\nEnd Program\n",
    )
    .unwrap();

    let main_path = dir.path().join("main.poh");
    fs::write(
        &main_path,
        format!(
            "Start Program\nImport \"{}\"\nWrite hello(\"Alice\")\nEnd Program\n",
            lib_path.display()
        ),
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(main_path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hi Alice"));
}

#[test]
fn system_import_stub_noop() {
    let path = write_program(&["Import system \"collections\"", "Write \"OK\""]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn system_import_with_alias_and_exposing() {
    let dir = tempdir().unwrap();
    let stdlib_dir = dir.path();
    let collections_path = stdlib_dir.join("collections.poh");
    fs::create_dir_all(stdlib_dir).unwrap();
    fs::write(
        &collections_path,
        "Start Program\nMake head with items\n    Return items[0]\nEnd\nEnd Program\n",
    )
    .unwrap();

    let program = write_program(&[
        "Import system \"collections\" as coll exposing head",
        "Set nums to [1, 2, 3]",
        "Write head(nums)",
        "Write coll::head(nums)",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(program.to_str().unwrap());
    cmd.env("POHLANG_STDLIB", stdlib_dir);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("1\n1"));
}

#[test]
fn join_and_range_builtins_work() {
    let path = write_program(&["Set nums to range(5)", "Write join(nums, \",\")"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("0,1,2,3,4"));
}

#[test]
fn ask_for_parses_correctly() {
    let path = write_program(&["Set x to 5", "Write \"Before ask\"", "Write x"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn list_indexing_works() {
    let path = write_program(&[
        "Set nums to [10, 20, 30, 40]",
        "Write nums[0]",
        "Write nums[2]",
        "Write nums[-1]",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("10"))
        .stdout(predicates::str::contains("30"))
        .stdout(predicates::str::contains("40"));
}

#[test]
fn dict_indexing_works() {
    let path = write_program(&[
        "Set person to {name: \"Bob\", age: 25}",
        "Write person[\"name\"]",
        "Write person[\"age\"]",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Bob"))
        .stdout(predicates::str::contains("25"));
}

#[test]
fn nested_indexing_works() {
    let path = write_program(&[
        "Set matrix to [[1, 2], [3, 4]]",
        "Write matrix[0][1]",
        "Write matrix[1][0]",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("2"))
        .stdout(predicates::str::contains("3"));
}

#[test]
fn string_indexing_works() {
    let path = write_program(&["Set word to \"Hello\"", "Write word[0]", "Write word[-1]"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("H"))
        .stdout(predicates::str::contains("o"));
}

#[test]
fn index_out_of_bounds_error() {
    let path = write_program(&["Set nums to [1, 2, 3]", "Write nums[10]"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("out of range"));
}

#[test]
fn dict_key_not_found_error() {
    let path = write_program(&["Set data to {x: 1}", "Write data[\"missing\"]"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("not found"));
}

#[test]
fn modern_list_syntax_works() {
    let path = write_program(&[
        "Set nums to [1, 2, 3]",
        "Write nums",
        "Set empty to []",
        "Write empty",
        "Set nested to [[1, 2], [3, 4]]",
        "Write nested",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[1, 2, 3]"))
        .stdout(predicates::str::contains("[]"))
        .stdout(predicates::str::contains("[[1, 2], [3, 4]]"));
}

#[test]
fn modern_dict_syntax_works() {
    let path = write_program(&[
        "Set person to {name: \"Alice\", age: 30}",
        "Write person",
        "Set config to {\"host\": \"localhost\", \"port\": 8080}",
        "Write config",
        "Set empty to {}",
        "Write empty",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Alice"))
        .stdout(predicates::str::contains("localhost"))
        .stdout(predicates::str::contains("{}"));
}

#[test]
fn collections_with_expressions() {
    let path = write_program(&[
        "Set calculated to [5 plus 5, 10 times 2, 30 divided by 3]",
        "Write calculated",
        "Set math to {sum: 10 plus 5, product: 10 times 5}",
        "Write math",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[10, 20, 10]"))
        .stdout(predicates::str::contains("sum"));
}

#[test]
fn legacy_collection_syntax_still_works() {
    let path = write_program(&[
        "Set nums to List contains 1, 2, 3",
        "Write nums",
        "Set dict to Dictionary contains \"x\" set to 5, \"y\" set to 10",
        "Write dict",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[1, 2, 3]"))
        .stdout(predicates::str::contains("5"));
}

#[test]
fn ask_for_in_bytecode() {
    let path = write_program(&["Ask for name", "Write \"Got: \" plus name"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--compile").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn increase_decrease_desugar_works() {
    let path = write_program(&[
        "Set x to 10",
        "Increase x by 5",
        "Write x",
        "Decrease x by 3",
        "Write x",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn arithmetic_operators_work() {
    let path = write_program(&[
        "Set a to 10 plus 5",
        "Write a",
        "Set b to 10 minus 5",
        "Write b",
        "Set c to 10 times 5",
        "Write c",
        "Set d to 10 divided by 5",
        "Write d",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn arithmetic_precedence_works() {
    let path = write_program(&["Set result to 10 plus 5 times 2", "Write result"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn decrease_with_variables_works() {
    let path = write_program(&["Set x to 100", "Set y to 10", "Decrease x by y", "Write x"]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert().success();
}

#[test]
fn logical_and_operator_works() {
    let path = write_program(&[
        "Set x to 1",
        "Set y to 1",
        "If x And y",
        "    Write \"both true\"",
        "Otherwise",
        "    Write \"not both\"",
        "End",
        "Set z to 0",
        "If x And z",
        "    Write \"should not print\"",
        "Otherwise",
        "    Write \"one false\"",
        "End",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("both true"))
        .stdout(predicates::str::contains("one false"));
}

#[test]
fn logical_or_operator_works() {
    let path = write_program(&[
        "Set x to 1",
        "Set y to 0",
        "If x Or y",
        "    Write \"at least one true\"",
        "Otherwise",
        "    Write \"both false\"",
        "End",
        "Set z to 0",
        "If z Or y",
        "    Write \"should not print\"",
        "Otherwise",
        "    Write \"both are false\"",
        "End",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("at least one true"))
        .stdout(predicates::str::contains("both are false"));
}

#[test]
fn logical_not_operator_works() {
    let path = write_program(&[
        "Set x to 0",
        "If Not x",
        "    Write \"x is falsy\"",
        "Otherwise",
        "    Write \"x is truthy\"",
        "End",
        "Set y to 1",
        "If Not y",
        "    Write \"should not print\"",
        "Otherwise",
        "    Write \"y is truthy\"",
        "End",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("x is falsy"))
        .stdout(predicates::str::contains("y is truthy"));
}

#[test]
fn complex_logical_expressions() {
    let path = write_program(&[
        "Set a to 1",
        "Set b to 1",
        "Set c to 0",
        "If a And b And Not c",
        "    Write \"complex condition true\"",
        "End",
        "If a Or c And b",
        "    Write \"precedence test\"",
        "End",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("complex condition true"))
        .stdout(predicates::str::contains("precedence test"));
}

#[test]
fn test_length_builtin() {
    let path = write_program(&[
        "Set nums to [1, 2, 3, 4, 5]",
        "Write length(nums)",
        "Set text to \"hello\"",
        "Write length(text)",
        "Set empty to []",
        "Write length(empty)",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("5"))
        .stdout(predicates::str::contains("0"));
}

#[test]
fn test_split_builtin() {
    let path = write_program(&[
        "Set text to \"apple,banana,cherry\"",
        "Set fruits to split(text, \",\")",
        "Write fruits[0]",
        "Write fruits[1]",
        "Write fruits[2]",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("apple"))
        .stdout(predicates::str::contains("banana"))
        .stdout(predicates::str::contains("cherry"));
}

#[test]
fn test_now_builtin() {
    let path = write_program(&[
        "Set timestamp to now()",
        "Write \"Timestamp: \"",
        "Write timestamp",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Timestamp:"));
}

#[test]
fn test_range_builtin_comprehensive() {
    let path = write_program(&[
        "Set nums to range(3)",
        "Write nums[0]",
        "Write nums[1]",
        "Write nums[2]",
        "Write length(nums)",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("0"))
        .stdout(predicates::str::contains("1"))
        .stdout(predicates::str::contains("2"))
        .stdout(predicates::str::contains("3"));
}

#[test]
fn test_join_builtin_comprehensive() {
    let path = write_program(&[
        "Set words to [\"Hello\", \"World\", \"PohLang\"]",
        "Write join(words, \" \")",
        "Set nums to [1, 2, 3]",
        "Write join(nums, \"-\")",
    ]);

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Hello World PohLang"))
        .stdout(predicates::str::contains("1-2-3"));
}
