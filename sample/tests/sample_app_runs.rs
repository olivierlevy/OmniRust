// sample/tests/sample_app_runs.rs

use std::process::Command;
use std::str;

#[test]
fn test_sample_app_runs_successfully() {
    // Ensure the sample app is built
    let build_status = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("omnirust_sample_app")
        .status()
        .expect("Failed to execute cargo build for sample app");

    assert!(build_status.success(), "Failed to build omnirust_sample_app");

    // Run the sample application
    let output = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("omnirust_sample_app")
        .output()
        .expect("Failed to execute sample application");

    // Check if the command executed successfully
    assert!(output.status.success(), "Sample application exited with an error. Stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Convert stdout to string
    let stdout_str = str::from_utf8(&output.stdout).expect("Stdout is not valid UTF-8");

    // Basic checks for output
    assert!(stdout_str.contains("OmniRust Sample Application - Starting"), "Sample app did not log starting message.");
    assert!(stdout_str.contains("Priority Queue Example:"), "Priority Queue showcase missing from output.");
    assert!(stdout_str.contains("OmniRust Sample Application - Finished"), "Sample app did not log finished message.");

    // Check for logger initialization message (this depends on the exact log format and level)
    // This is a bit brittle as log messages can change.
    // A more robust way might be to have the sample app write a specific marker to a temp file.
    assert!(stdout_str.contains("Logger initialized with level:"), "Logger initialization message not found.");
}