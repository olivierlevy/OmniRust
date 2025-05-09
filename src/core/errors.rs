//! # Custom Error Types for OmniRust
//!
//! This module defines the primary error enum `OmniRustError` used throughout the
//! OmniRust framework. It leverages the `thiserror` crate for convenient
//! error type derivation.

use thiserror::Error;

/// Represents all possible errors that can occur within the OmniRust framework.
///
/// Each variant corresponds to a specific category of error, allowing for
/// more granular error handling and reporting.
#[derive(Error, Debug)]
pub enum OmniRustError {
    /// Errors related to application configuration loading or parsing.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Errors originating from database operations.
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Errors related to network operations, such as HTTP requests or WebSocket communication.
    #[error("Networking error: {0}")]
    NetworkingError(String),

    /// Errors from standard input/output operations.
    /// This variant uses `#[from]` to allow easy conversion from `std::io::Error`.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// Errors during JSON serialization or deserialization.
    /// This variant uses `#[from]` to allow easy conversion from `serde_json::Error`.
    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),

    /// Errors during TOML serialization or deserialization.
    /// This variant uses `#[from]` to allow easy conversion from `toml::de::Error`.
    #[error("TOML error: {0}")]
    TOMLError(#[from] toml::de::Error),

    /// Errors during YAML serialization or deserialization.
    /// This variant uses `#[from]` to allow easy conversion from `serde_yaml::Error`.
    #[error("YAML error: {0}")]
    YAMLError(#[from] serde_yaml::Error),

    /// A catch-all for other types of errors not covered by specific variants.
    #[error("Other error: {0}")]
    Other(String),
}