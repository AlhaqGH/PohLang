// Integration tests for JSON operations in PohLang
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_parse_json_object() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_string to "{\"name\":\"Alice\",\"age\":30}"
Set parsed to parse json from json_string
Write "Parsed JSON successfully"
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Parsed JSON successfully"));
}

#[test]
fn test_json_get() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_string to "{\"name\":\"Bob\",\"age\":25}"
Set parsed to parse json from json_string
Set name to get "name" from json parsed
Write name
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn test_json_set() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_string to "{\"name\":\"Carol\"}"
Set parsed to parse json from json_string
Set updated to set "age" in json parsed to 35
Set json_str to convert to json updated
Write json_str
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("age"))
        .stdout(predicate::str::contains("35"));
}

#[test]
fn test_new_json_object() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set obj to new json object
Set obj2 to set "status" in json obj to "ok"
Set json_str to convert to json obj2
Write json_str
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("ok"));
}

#[test]
fn test_new_json_array() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set arr to new json array
Set arr2 to push 1 to json arr
Set arr3 to push 2 to json arr2
Set arr4 to push 3 to json arr3
Set json_str to convert to json arr4
Write json_str
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_json_length_array() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_str to "[1,2,3,4,5]"
Set arr to parse json from json_str
Set len to json length of arr
Write len
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn test_json_length_object() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_str to "{\"a\":1,\"b\":2,\"c\":3}"
Set obj to parse json from json_str
Set len to json length of obj
Write len
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_json_pretty() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set json_str to "{\"name\":\"Dave\",\"age\":40}"
Set parsed to parse json from json_str
Set pretty to convert to pretty json parsed
Write pretty
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("name"))
        .stdout(predicate::str::contains("Dave"));
}

#[test]
fn test_json_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    
    fs::write(
        &test_file,
        r#"
Set original to "{\"items\":[1,2,3],\"status\":\"ok\"}"
Set parsed to parse json from original
Set stringified to convert to json parsed
Set reparsed to parse json from stringified
Write "Roundtrip successful"
"#,
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Roundtrip successful"));
}

#[test]
fn test_json_with_file() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.poh");
    let json_file = temp_dir.path().join("data.json");
    
    fs::write(
        &test_file,
        format!(
            r#"
Set obj to new json object
Set obj2 to set "message" in json obj to "Hello from PohLang"
Set obj3 to set "version" in json obj2 to 1
Set json_str to convert to pretty json obj3
Write json_str into file at "{}"
Set content to read file at "{}"
Write content
"#,
            json_file.to_str().unwrap().replace("\\", "\\\\"),
            json_file.to_str().unwrap().replace("\\", "\\\\")
        ),
    )
    .unwrap();

    Command::cargo_bin("pohlang")
        .unwrap()
        .arg(test_file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("message"))
        .stdout(predicate::str::contains("Hello from PohLang"));
    
    // Verify the file was actually created
    assert!(json_file.exists());
}
