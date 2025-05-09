//! # Application Configuration Management
//!
//! This module provides robust application configuration management. It leverages the
//! `config` crate to load settings from various sources, including configuration files
//! (TOML, YAML, JSON) and environment variables, allowing for flexible and
//! environment-aware application setup.
//!
//! ## Configuration Sources
//!
//! Configuration is loaded in the following order of precedence (lowest to highest):
//! 1.  `config.toml` (optional)
//! 2.  `config.yaml` (optional)
//! 3.  `config.json` (optional)
//! 4.  Environment variables prefixed with `APP_` (e.g., `APP_DATABASE_URL`).
//!
//! Settings from later sources override those from earlier ones. For example, an
//! environment variable `APP_LOG_LEVEL` will override the `log_level` defined in
//! any of the configuration files.
//!
//! ## Environment Variables
//!
//! Environment variables should be prefixed with `APP_`. For nested configuration
//! keys, use `_` as a separator. For example, a configuration like:
//!
//! ```toml
//! [database]
//! url = "..."
//! ```
//!
//! can be overridden by an environment variable `APP_DATABASE_URL`.

use config::{Config, File, Environment};
use crate::core::errors::OmniRustError;

/// Represents the application's configuration.
///
/// This struct holds all configurable parameters for the application,
/// such as database connection strings, logging levels, and other
/// service-specific settings.
#[derive(Debug)]
pub struct AppConfig {
    /// The URL for connecting to the primary database.
    /// Example: "postgres://user:password@localhost/mydatabase"
    pub database_url: String,
    /// The logging level for the application.
    /// Common values: "trace", "debug", "info", "warn", "error".
    pub log_level: String,
    // Add other configuration fields here as needed, e.g.:
    // pub server_port: u16,
    // pub external_api_key: String,
}

impl AppConfig {
    /// Loads the application configuration.
    ///
    /// Configuration is loaded from the following sources, in order of precedence (lowest to highest):
    /// 1. `config.toml` (optional)
    /// 2. `config.yaml` (optional)
    /// 3. `config.json` (optional)
    /// 4. Environment variables prefixed with `APP_` (e.g., `APP_DATABASE_URL`, `APP_LOG_LEVEL`).
    ///    Environment variables use `_` as a separator for nested keys (e.g., `APP_SERVER_PORT`).
    ///
    /// # Errors
    /// Returns an error if the `config.toml` file cannot be found or parsed,
    /// or if required configuration values are missing or have incorrect types.
    ///
    /// # Panics
    /// This function does not panic directly, but underlying `config` crate methods might
    /// if there are critical issues with its internal state, though this is rare.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming config.toml exists and is valid
    /// use omnirust::core::config::AppConfig; // Adjust path as needed
    ///
    /// match AppConfig::load() {
    ///     Ok(config) => {
    ///         println!("Loaded database URL: {}", config.database_url);
    ///         println!("Loaded log level: {}", config.log_level);
    ///     }
    ///     Err(e) => {
    ///         eprintln!("Failed to load application configuration: {}", e);
    ///     }
    /// }
    /// ```
    pub fn load() -> Result<Self, OmniRustError> {
        let settings = Config::builder()
            // Load configuration from `config.toml`, `config.yaml`, or `config.json` (optional).
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name("config.yaml").required(false))
            .add_source(File::with_name("config.json").required(false))
            // Allow overriding configuration with environment variables.
            // Variables should be prefixed with "APP_" (e.g., APP_DATABASE__URL for nested keys).
            // The separator `_` is used for nested keys (e.g., `database.url` becomes `APP_DATABASE_URL`).
            // Note: `config` crate uses `__` (double underscore) by default for nesting if `separator` is not specified.
            // Here, we explicitly use `_` as the separator for environment variables.
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build().map_err(|e| OmniRustError::ConfigError(e.to_string()))?;

        // Attempt to retrieve each configuration value, providing defaults if not found.
        Ok(AppConfig {
            database_url: settings.get_string("database.url")
                .map_err(|e| OmniRustError::ConfigError(e.to_string()))
                .unwrap_or_else(|_| "postgres://user:password@localhost/default_omnirust_db".to_string()),
            log_level: settings.get_string("log.level")
                .map_err(|e| OmniRustError::ConfigError(e.to_string()))
                .unwrap_or_else(|_| "info".to_string()),
            // Example for adding a new field with a default:
            // server_port: settings.get::<u16>("server.port").unwrap_or(8080),
        })
}
}
