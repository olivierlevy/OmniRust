use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Writes data safely to a file, creating parent directories if needed.
pub fn write_to_file(path: &str, data: &[u8]) -> std::io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

/// Recursively searches for files matching a specific extension.
pub fn find_files_with_extension(dir: &str, extension: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            results.extend(find_files_with_extension(path.to_str().unwrap(), extension)?);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
            results.push(path);
        }
    }
    Ok(results)
}
