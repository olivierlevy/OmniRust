// src/lib.rs

// Core functionalities
pub mod core;

// Caching utilities
pub mod caching;

// Data structures
pub mod data_structures;

// GraphQL implementation
pub mod graphql;

// Logging utilities
pub mod logging;

// Data models (if any, currently seems to be just a mod.rs)
pub mod models;

// Networking utilities (HTTP, WebSockets, gRPC)
pub mod networking;

// General utilities (string, file, datetime, parsing)
pub mod utils;

// WebSocket implementation (might be part of networking or standalone)
pub mod websocket;

// You might also want to re-export key types or functions for easier access
// For example:
// pub use crate::core::config::AppConfig;
// pub use crate::data_structures::tree::Tree;
