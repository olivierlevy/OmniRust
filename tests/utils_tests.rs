use omnirust::utils::file_utils::{write_to_file, find_files_with_extension};
use std::fs;

#[test]
fn test_write_to_file() {
    let path = "tests/output/test_file.txt";
    let data = b"Hello, OmniRust!";
    write_to_file(path, data).unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "Hello, OmniRust!");
}

#[test]
fn test_find_files_with_extension() {
    let files = find_files_with_extension("tests", "toml").unwrap();
    assert!(files.iter().any(|path| path.ends_with("test_config.toml")));
}
