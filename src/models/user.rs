// src/models/user.rs

use serde::{Serialize, Deserialize}; // For potential API usage
use sqlx::FromRow; // For mapping from database rows

/// Represents a user entity in the database.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct DbUser {
    pub id: i32, // Assuming serial primary key
    pub name: String,
    pub email: Option<String>,
    // Add other fields as needed, e.g., created_at, updated_at
}
