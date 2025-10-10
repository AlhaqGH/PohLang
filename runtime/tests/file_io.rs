// Integration tests for file I/O operations from PohLang syntax

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn run_in_dir(lines: &[&str], temp_dir: &TempDir) -> Command {
    let temp_path = temp_dir.path();
    let script_path = temp_path.join("test.poh");

    let mut content = String::from("Start Program\n");
    for line in lines {
        content.push_str(line);
        content.push('\n');
    }
    content.push_str("End Program\n");

    fs::write(&script_path, content).unwrap();

    let mut cmd = Command::cargo_bin("pohlang").unwrap();
    cmd.arg("--run").arg(&script_path);
    cmd.current_dir(temp_path);
    cmd
}

#[test]
fn test_write_and_read_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let test_file = temp_path.join("test_output.txt");
    let test_file_str = test_file.to_str().unwrap();

    // Write to file
    let mut cmd = run_in_dir(
        &[
            &format!(
                "Set r to write \"Hello PohLang!\" into file at \"{}\"",
                test_file_str
            ),
            "Write \"File written\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File written"));

    // Verify file exists and has correct content
    assert!(test_file.exists());
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "Hello PohLang!");

    // Read from file
    let mut cmd = run_in_dir(
        &[
            &format!("Set content to read file at \"{}\"", test_file_str),
            "Write content",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello PohLang!"));
}

#[test]
fn test_append_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let test_file = temp_path.join("append_test.txt");
    let test_file_str = test_file.to_str().unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!(
                "Set r1 to write \"Line 1\" into file at \"{}\"",
                test_file_str
            ),
            &format!(
                "Set r2 to append \"Line 2\" into file at \"{}\"",
                test_file_str
            ),
            &format!(
                "Set r3 to append \"Line 3\" into file at \"{}\"",
                test_file_str
            ),
            &format!("Set content to read file at \"{}\"", test_file_str),
            "Write content",
        ],
        &temp_dir,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Line 1Line 2Line 3"));
}

#[test]
fn test_file_exists() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let existing_file = temp_path.join("existing.txt");
    let existing_file_str = existing_file.to_str().unwrap();
    let missing_file_str = temp_path.join("missing.txt").to_str().unwrap().to_string();

    // Create a file
    fs::write(&existing_file, "test").unwrap();

    // Check existing file
    let mut cmd = run_in_dir(
        &[
            &format!("Set exists to file exists at \"{}\"", existing_file_str),
            "If exists is equal to True Write \"File exists!\" Otherwise Write \"File not found\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File exists!"));

    // Check missing file
    let mut cmd = run_in_dir(
        &[
            &format!("Set exists to file exists at \"{}\"", missing_file_str),
            "If exists is equal to True Write \"File exists!\" Otherwise Write \"File not found\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File not found"));
}

#[test]
fn test_delete_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let test_file = temp_path.join("to_delete.txt");
    let test_file_str = test_file.to_str().unwrap();

    // Create a file
    fs::write(&test_file, "temporary content").unwrap();
    assert!(test_file.exists());

    // Delete it
    let mut cmd = run_in_dir(
        &[
            &format!("Set r to delete file at \"{}\"", test_file_str),
            "Write \"File deleted\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File deleted"));

    // Verify it's gone
    assert!(!test_file.exists());
}

#[test]
fn test_create_directory() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let new_dir = temp_path.join("new_directory");
    let new_dir_str = new_dir.to_str().unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!("Set r to create directory at \"{}\"", new_dir_str),
            "Write \"Directory created\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Directory created"));

    assert!(new_dir.is_dir());
}

#[test]
fn test_list_directory() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();

    // Create some files
    fs::write(temp_path.join("file1.txt"), "test1").unwrap();
    fs::write(temp_path.join("file2.txt"), "test2").unwrap();
    fs::write(temp_path.join("file3.txt"), "test3").unwrap();

    let temp_path_str = temp_path.to_str().unwrap();
    let mut cmd = run_in_dir(
        &[
            &format!("Set files to list files in \"{}\"", temp_path_str),
            "Write \"Files found:\"",
            "Write files",
        ],
        &temp_dir,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Files found:"))
        .stdout(predicate::str::contains("file1.txt"))
        .stdout(predicate::str::contains("file2.txt"))
        .stdout(predicate::str::contains("file3.txt"));
}

#[test]
fn test_read_lines() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let test_file = temp_path.join("lines.txt");
    let test_file_str = test_file.to_str().unwrap();

    // Create a multi-line file
    fs::write(&test_file, "Line 1\nLine 2\nLine 3\n").unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!("Set lines to read lines from file at \"{}\"", test_file_str),
            "Write lines",
        ],
        &temp_dir,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Line 1"))
        .stdout(predicate::str::contains("Line 2"))
        .stdout(predicate::str::contains("Line 3"));
}

#[test]
fn test_copy_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let source_file = temp_path.join("source.txt");
    let dest_file = temp_path.join("destination.txt");
    let source_str = source_file.to_str().unwrap();
    let dest_str = dest_file.to_str().unwrap();

    // Create source file
    fs::write(&source_file, "Copy me!").unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!(
                "Set r to copy file from \"{}\" to \"{}\"",
                source_str, dest_str
            ),
            "Write \"File copied\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File copied"));

    // Verify both files exist with same content
    assert!(source_file.exists());
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert_eq!(content, "Copy me!");
}

#[test]
fn test_move_file() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let source_file = temp_path.join("move_source.txt");
    let dest_file = temp_path.join("move_dest.txt");
    let source_str = source_file.to_str().unwrap();
    let dest_str = dest_file.to_str().unwrap();

    // Create source file
    fs::write(&source_file, "Move me!").unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!(
                "Set r to move file from \"{}\" to \"{}\"",
                source_str, dest_str
            ),
            "Write \"File moved\"",
        ],
        &temp_dir,
    );
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File moved"));

    // Verify source is gone and dest exists
    assert!(!source_file.exists());
    assert!(dest_file.exists());
    let content = fs::read_to_string(&dest_file).unwrap();
    assert_eq!(content, "Move me!");
}

#[test]
fn test_file_not_found_error() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    let missing_file = temp_path.join("nonexistent_file_12345.txt");
    let missing_file_str = missing_file.to_str().unwrap();

    let mut cmd = run_in_dir(
        &[
            &format!("Set content to read file at \"{}\"", missing_file_str),
            "Write content",
        ],
        &temp_dir,
    );

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
}
