//! # Tracing-based Logging Setup
//!
//! This module configures and initializes the logging system for the OmniRust
//! framework using the `tracing` and `tracing-subscriber` crates. It allows
//! for log level configuration via `AppConfig` and environment variables.
//!
//! ## Features
//!
//! - Log level controlled by `AppConfig.log_level` (e.g., "info", "debug", "trace").
//! - Environment variable override for log levels using `RUST_LOG` (standard `tracing-subscriber` behavior).
//! - Detailed log output including:
//!     - Timestamps (by default from `tracing_subscriber::fmt`)
//!     - Log level
//!     - Source file and line number
//!     - Thread ID
//! - No target information in logs by default (can be enabled if needed).
//!
//! ## Usage
//!
//! The `init_logger` function should be called early in the application's lifecycle,
//! typically in `main.rs` after loading the `AppConfig`.
//!
//! ```no_run
//! // In main.rs
//! use omnirust::core::config::AppConfig;
//! use omnirust::logging::logger::init_logger; // Adjust path as needed
//! use omnirust::core::errors::OmniRustError;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), OmniRustError> {
//!     let config = AppConfig::load()?;
//!     init_logger(&config)?;
//!
//!     tracing::info!("Application started successfully!");
//!     // ... rest of the application logic
//!     Ok(())
//! }
//! ```

use tracing::{info, Level};
use tracing_subscriber::{fmt, EnvFilter};
use crate::core::config::AppConfig;
use crate::core::errors::OmniRustError;

/// Initializes the global logger based on the provided application configuration.
///
/// This function sets up the `tracing` subscriber with a format that includes
/// timestamps, log levels, source file/line numbers, and thread IDs. The log
/// level is determined by `config.log_level` and can be further influenced by
/// the `RUST_LOG` environment variable.
///
/// # Arguments
///
/// * `config` - A reference to the `AppConfig` containing the desired `log_level`.
///
/// # Errors
///
/// Returns `OmniRustError::ConfigError` if:
/// - The `log_level` in the configuration is invalid and cannot be parsed.
/// - The `EnvFilter` cannot be built from the environment (e.g., invalid `RUST_LOG` syntax).
pub fn init_logger(config: &AppConfig) -> Result<(), OmniRustError> {
    // Parse the log level string from the configuration.
    // This will be the default level if RUST_LOG is not set or doesn't override it.
    let log_level = config.log_level.parse::<Level>()
        .map_err(|e| OmniRustError::ConfigError(format!("Invalid log level ('{}') in configuration: {}", config.log_level, e)))?;

    // Create an environment filter based on the log level
    let filter = EnvFilter::builder()
        .with_default_directive(log_level.into())
        .from_env()
        .map_err(|e| OmniRustError::ConfigError(format!("Failed to create log filter from environment: {}", e)))?;

    // Configure and initialize the tracing subscriber
    fmt()
        .with_target(false) // Disable target in logs
        .with_line_number(true) // Include line numbers
        .with_file(true) // Include file names
        .with_thread_ids(true) // Include thread IDs
        .with_env_filter(filter) // Apply the environment filter
        .init();

    info!("Logger initialized with level: {}", config.log_level);

    Ok(())
}

// The log_info and log_error functions are no longer needed as tracing macros are used directly.
// pub fn log_info(message: &str) {
//     info!("{}", message);
// }

// pub fn log_error(message: &str) {
//     error!("{}", message);
// }
