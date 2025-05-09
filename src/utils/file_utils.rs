//! # Asynchronous File Utilities
//!
//! This module provides utility functions for performing common file operations
//! asynchronously using `tokio::fs`. All functions return `Result` with
//! `OmniRustError` to integrate with the framework's error handling.

use tokio::fs;
use tokio::io::{self, AsyncWriteExt};
use std::path::Path;
use crate::core::errors::OmniRustError;

/// Reads the entire content of a file into a string asynchronously.
///
/// # Arguments
///
/// * `path` - A type that can be converted into a `Path` reference, representing the file to read.
///
/// # Errors
///
/// Returns `OmniRustError::IOError` if the file cannot be read (e.g., it doesn't exist,
/// permissions are denied, or other I/O issues occur).
///
/// # Examples
///
/// ```no_run
/// use omnirust::utils::file_utils::read_to_string; // Adjust path as needed
/// use omnirust::core::errors::OmniRustError;
///
/// #[tokio::main]
/// async fn main() -> Result<(), OmniRustError> {
///     // Assuming "example.txt" exists and contains "Hello, world!"
///     let content = read_to_string("example.txt").await?;
///     assert_eq!(content, "Hello, world!");
///     Ok(())
/// }
/// ```
pub async fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, OmniRustError> {
    fs::read_to_string(path).await.map_err(OmniRustError::IOError)
}

/// Writes a string to a file asynchronously.
///
/// This function will create the file if it does not exist, and will truncate it if it does.
///
/// # Arguments
///
/// * `path` - A type that can be converted into a `Path` reference, representing the file to write to.
/// * `content` - The string content to write to the file.
///
/// # Errors
///
/// Returns `OmniRustError::IOError` if the file cannot be written (e.g., due to permissions
/// issues or other I/O errors).
///
/// # Examples
///
/// ```no_run
/// use omnirust::utils::file_utils::write_string_to_file; // Adjust path as needed
/// use omnirust::core::errors::OmniRustError;
///
/// #[tokio::main]
/// async fn main() -> Result<(), OmniRustError> {
///     write_string_to_file("output.txt", "Hello from OmniRust!").await?;
///     // "output.txt" will now contain "Hello from OmniRust!"
///     Ok(())
/// }
/// ```
pub async fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<(), OmniRustError> {
    let mut file = fs::File::create(path).await.map_err(OmniRustError::IOError)?;
    file.write_all(content.as_bytes()).await.map_err(OmniRustError::IOError)
}

/// Checks if a path exists asynchronously.
///
/// This function checks for the existence of a file or directory at the given path.
///
/// # Arguments
///
/// * `path` - A type that can be converted into a `Path` reference.
///
/// # Returns
///
/// Returns `true` if the path exists and metadata can be retrieved, `false` otherwise.
/// Note that this function swallows errors from `fs::metadata` for simplicity,
/// returning `false` if metadata retrieval fails for any reason (e.g. permissions).
///
/// # Examples
///
/// ```no_run
/// use omnirust::utils::file_utils::{path_exists, write_string_to_file}; // Adjust path as needed
/// use omnirust::core::errors::OmniRustError;
///
/// #[tokio::main]
/// async fn main() -> Result<(), OmniRustError> {
///     assert!(!path_exists("my_temp_file.txt").await);
///     write_string_to_file("my_temp_file.txt", "content").await?;
///     assert!(path_exists("my_temp_file.txt").await);
///     // Remember to clean up the file in a real scenario
///     tokio::fs::remove_file("my_temp_file.txt").await?;
///     Ok(())
/// }
/// ```
pub async fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).await.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_read_write_string_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");

        let content = "Hello, OmniRust File Utils!";
        write_string_to_file(&file_path, content).await.unwrap();

        let read_content = read_to_string(&file_path).await.unwrap();
        assert_eq!(read_content, content);
    }

    #[tokio::test]
    async fn test_path_exists() {
        let dir = tempdir().unwrap();
        let existing_file_path = dir.path().join("exists.txt");
        let non_existing_file_path = dir.path().join("not_exists.txt");

        fs::File::create(&existing_file_path).await.unwrap();

        assert!(path_exists(&existing_file_path).await);
        assert!(!path_exists(&non_existing_file_path).await);
        assert!(path_exists(dir.path()).await); // Check if directory exists
    }
}
