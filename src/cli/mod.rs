// src/cli/mod.rs

pub mod arg_parser;
pub use clap::Parser; // Re-export clap::Parser
pub mod tui_components;
// pub mod cli_logging;    // Placeholder for CLI-specific logging/config if different from core
