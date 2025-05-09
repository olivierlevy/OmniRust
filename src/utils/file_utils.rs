// src/utils/file_utils.rs

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Reads the entire content of a file into a string.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Writes a string to a file, creating the file if it doesn't exist,
/// and overwriting it if it does.
pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}

/// Checks if a path exists.
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use tempfile::tempdir;

    #[test]
    fn test_read_write_string_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");

        let content = "Hello, OmniRust File Utils!";
        write_string_to_file(&file_path, content).unwrap();

        let read_content = read_to_string(&file_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_path_exists() {
        let dir = tempdir().unwrap();
        let existing_file_path = dir.path().join("exists.txt");
        let non_existing_file_path = dir.path().join("not_exists.txt");

        File::create(&existing_file_path).unwrap();

        assert!(path_exists(&existing_file_path));
        assert!(!path_exists(&non_existing_file_path));
        assert!(path_exists(dir.path())); // Check if directory exists
    }
}
