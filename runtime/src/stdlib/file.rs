// File I/O Module for PohLang
// Provides phrasal expression-based file operations

use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Read the contents of a file as a string
/// Usage: "Read file at [path]"
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Write content to a file (overwrites existing content)
/// Usage: "Write [content] to file at [path]"
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Append content to a file
/// Usage: "Append [content] to file at [path]"
pub fn append_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}

/// Check if a file exists
/// Usage: "File exists at [path]"
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Delete a file
/// Usage: "Delete file at [path]"
pub fn delete_file(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

/// Create a directory
/// Usage: "Create directory at [path]"
pub fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// List files in a directory
/// Usage: "List files in directory at [path]"
pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            files.push(filename.to_string());
        }
    }
    
    Ok(files)
}

/// Read file as lines
/// Usage: "Read lines from file at [path]"
pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// Copy a file
/// Usage: "Copy file from [source] to [destination]"
pub fn copy_file(source: &str, destination: &str) -> io::Result<u64> {
    fs::copy(source, destination)
}

/// Move/rename a file
/// Usage: "Move file from [source] to [destination]"
pub fn move_file(source: &str, destination: &str) -> io::Result<()> {
    fs::rename(source, destination)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn test_file_path(temp_dir: &TempDir, filename: &str) -> PathBuf {
        temp_dir.path().join(filename)
    }

    #[test]
    fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = test_file_path(&temp_dir, "test.txt");
        let path_str = path.to_str().unwrap();

        // Write content
        write_file(path_str, "Hello, PohLang!").unwrap();

        // Read content
        let content = read_file(path_str).unwrap();
        assert_eq!(content, "Hello, PohLang!");
    }

    #[test]
    fn test_append_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = test_file_path(&temp_dir, "append.txt");
        let path_str = path.to_str().unwrap();

        // Write initial content
        write_file(path_str, "Line 1\n").unwrap();

        // Append more content
        append_file(path_str, "Line 2\n").unwrap();
        append_file(path_str, "Line 3\n").unwrap();

        // Read and verify
        let content = read_file(path_str).unwrap();
        assert_eq!(content, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let path = test_file_path(&temp_dir, "exists.txt");
        let path_str = path.to_str().unwrap();

        // File doesn't exist yet
        assert!(!file_exists(path_str));

        // Create file
        write_file(path_str, "test").unwrap();

        // Now it exists
        assert!(file_exists(path_str));
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = test_file_path(&temp_dir, "delete.txt");
        let path_str = path.to_str().unwrap();

        // Create and delete
        write_file(path_str, "temporary").unwrap();
        assert!(file_exists(path_str));

        delete_file(path_str).unwrap();
        assert!(!file_exists(path_str));
    }

    #[test]
    fn test_create_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = test_file_path(&temp_dir, "test_dir");
        let dir_str = dir_path.to_str().unwrap();

        create_directory(dir_str).unwrap();
        assert!(Path::new(dir_str).is_dir());
    }

    #[test]
    fn test_list_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create some test files
        write_file(test_file_path(&temp_dir, "file1.txt").to_str().unwrap(), "test").unwrap();
        write_file(test_file_path(&temp_dir, "file2.txt").to_str().unwrap(), "test").unwrap();
        write_file(test_file_path(&temp_dir, "file3.txt").to_str().unwrap(), "test").unwrap();

        // List directory
        let files = list_directory(temp_dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 3);
        assert!(files.contains(&"file1.txt".to_string()));
        assert!(files.contains(&"file2.txt".to_string()));
        assert!(files.contains(&"file3.txt".to_string()));
    }

    #[test]
    fn test_read_lines() {
        let temp_dir = TempDir::new().unwrap();
        let path = test_file_path(&temp_dir, "lines.txt");
        let path_str = path.to_str().unwrap();

        // Write multi-line content
        write_file(path_str, "Line 1\nLine 2\nLine 3").unwrap();

        // Read as lines
        let lines = read_lines(path_str).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
    }

    #[test]
    fn test_copy_file() {
        let temp_dir = TempDir::new().unwrap();
        let source = test_file_path(&temp_dir, "source.txt");
        let dest = test_file_path(&temp_dir, "dest.txt");

        // Create source file
        write_file(source.to_str().unwrap(), "Copy me!").unwrap();

        // Copy file
        copy_file(source.to_str().unwrap(), dest.to_str().unwrap()).unwrap();

        // Verify both exist with same content
        assert!(file_exists(source.to_str().unwrap()));
        assert!(file_exists(dest.to_str().unwrap()));
        assert_eq!(read_file(dest.to_str().unwrap()).unwrap(), "Copy me!");
    }

    #[test]
    fn test_move_file() {
        let temp_dir = TempDir::new().unwrap();
        let source = test_file_path(&temp_dir, "move_source.txt");
        let dest = test_file_path(&temp_dir, "move_dest.txt");

        // Create source file
        write_file(source.to_str().unwrap(), "Move me!").unwrap();

        // Move file
        move_file(source.to_str().unwrap(), dest.to_str().unwrap()).unwrap();

        // Verify source is gone and dest exists
        assert!(!file_exists(source.to_str().unwrap()));
        assert!(file_exists(dest.to_str().unwrap()));
        assert_eq!(read_file(dest.to_str().unwrap()).unwrap(), "Move me!");
    }
}
