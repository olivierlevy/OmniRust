// src/database/traits.rs

use async_trait::async_trait;
use std::error::Error;

/// Represents a generic query result.
/// For simplicity, this might be a string representation or a more structured type.
pub type QueryResult = Result<String, Box<dyn Error + Send + Sync>>; // Example

/// A generic trait for database connections.
#[async_trait]
pub trait DbConnection: Send + Sync {
    /// The type of configuration needed to establish a connection.
    type Config: Send + Sync;
    /// The type of error that can occur during connection or query execution.
    type ConnectionError: Error + Send + Sync + 'static;

    /// Establishes a new database connection.
    async fn connect(config: Self::Config) -> Result<Self, Self::ConnectionError>
    where
        Self: Sized; // Sized is required for Self in return position

    /// Executes a raw query string.
    /// Returns a generic result or a specific error type.
    async fn execute_raw_query(&mut self, query: &str) -> QueryResult;

    // More specific methods could be added, e.g., for prepared statements,
    // transactions, or ORM-like operations if desired.
    // async fn query_typed<T: FromRow>(&mut self, query: &str, params: &[&dyn ToSql]) -> Result<Vec<T>, Self::ConnectionError>;
    // async fn begin_transaction(&mut self) -> Result<(), Self::ConnectionError>;
    // async fn commit_transaction(&mut self) -> Result<(), Self::ConnectionError>;
    // async fn rollback_transaction(&mut self) -> Result<(), Self::ConnectionError>;

    /// Closes the database connection.
    async fn close(self) -> Result<(), Self::ConnectionError>;
}

/// A generic trait for connection pools.
#[async_trait]
pub trait DbConnectionPool: Send + Sync {
    type Connection: DbConnection;
    type Config: Send + Sync;
    type PoolError: Error + Send + Sync + 'static;

    /// Creates a new connection pool.
    async fn new_pool(config: Self::Config) -> Result<Self, Self::PoolError>
    where
        Self: Sized;
    
    /// Gets a connection from the pool.
    async fn get_connection(&self) -> Result<Self::Connection, Self::PoolError>;

    /// Closes all connections in the pool and shuts down the pool.
    async fn close_pool(self) -> Result<(), Self::PoolError>;
}
