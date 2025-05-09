// src/database/sql_connector.rs

use crate::database::traits::{DbConnection, DbConnectionPool, QueryResult};
use async_trait::async_trait;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions}; // Removed PgRow
use sqlx::{Pool, Postgres, Error as SqlxError, Connection as _, Executor as _}; // Renamed Connection to avoid conflict
use std::time::Duration;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout_seconds: Option<u64>,
    pub idle_timeout_seconds: Option<u64>,
    pub max_lifetime_seconds: Option<u64>,
}

#[derive(Debug)]
pub struct PostgresConnection {
    // For a single connection, sqlx uses sqlx::PgConnection
    // However, it's more common to work with a pool directly or connections from a pool.
    // This struct might represent a connection leased from a pool if not using the pool directly.
    // For simplicity, let's assume this will wrap a pooled connection or be used in contexts
    // where a single, short-lived connection is managed.
    // For now, we'll make it a wrapper around a Pool and get a connection from it for operations.
    // This is not ideal for a `DbConnection` trait that implies a single, stateful connection.
    // A better approach for `DbConnection` would be to wrap `sqlx::pool::PoolConnection<Postgres>`.
    // Let's adjust to use a single connection for the `DbConnection` trait implementation.
    conn: sqlx::PgConnection,
}

#[derive(Debug)]
pub struct PostgresPool {
    pool: Pool<Postgres>,
}

// Custom Error type for our Postgres connector
#[derive(Debug, thiserror::Error)]
pub enum PostgresConnectorError {
    #[error("SQLx error: {0}")]
    Sqlx(#[from] SqlxError),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Connection options parse error: {0}")]
    UrlParse(String), // Sqlx UrlParseError is not easily clonable or directly usable here
}


#[async_trait]
impl DbConnection for PostgresConnection {
    type Config = PgConnectOptions; // Using PgConnectOptions for a single connection
    type ConnectionError = PostgresConnectorError;

    async fn connect(options: Self::Config) -> Result<Self, Self::ConnectionError> {
        let conn = sqlx::PgConnection::connect_with(&options).await?;
        Ok(PostgresConnection { conn })
    }

    async fn execute_raw_query(&mut self, query: &str) -> QueryResult {
        // sqlx's simple query returns a stream of Either<PgQueryResult, PgRow>
        // For a generic QueryResult as String, we might try to fetch one row or summarize.
        // This is a simplification. A real implementation would need more robust result handling.
        // For now, let's try to execute and get rows affected or a simple message.
        match self.conn.execute(query).await {
            Ok(result) => Ok(format!("Query executed successfully. Rows affected: {}", result.rows_affected())),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    async fn close(mut self) -> Result<(), Self::ConnectionError> {
        self.conn.close().await?;
        Ok(())
    }
}


#[async_trait]
impl DbConnectionPool for PostgresPool {
    type Connection = PostgresConnection; // This needs to be a type that implements DbConnection
                                          // A direct PoolConnection<Postgres> would be better for DbConnection.
                                          // Let's refine this. DbConnectionPool should return a wrapper
                                          // around PoolConnection<Postgres> that implements DbConnection.

    type Config = PostgresConfig;
    type PoolError = PostgresConnectorError;

    async fn new_pool(config: Self::Config) -> Result<Self, Self::PoolError> {
        let connect_options = PgConnectOptions::from_str(&config.database_url)
            .map_err(|e| PostgresConnectorError::UrlParse(e.to_string()))?; // Convert sqlx::Error to our error

        // Note: PgConnectOptions itself doesn't have a direct connect_timeout method.
        // It's typically part of the DSN string or handled by the pool.
        // The pool's acquire_timeout is more relevant here.
        
        let mut pool_options = PgPoolOptions::new();
        if let Some(max_conn) = config.max_connections {
            pool_options = pool_options.max_connections(max_conn);
        }
        if let Some(min_conn) = config.min_connections {
            pool_options = pool_options.min_connections(min_conn);
        }
        if let Some(timeout) = config.connect_timeout_seconds { // Also set on pool options
            pool_options = pool_options.acquire_timeout(Duration::from_secs(timeout));
        }
        if let Some(idle_timeout) = config.idle_timeout_seconds {
            pool_options = pool_options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        if let Some(max_lifetime) = config.max_lifetime_seconds {
            pool_options = pool_options.max_lifetime(Duration::from_secs(max_lifetime));
        }
        
        let pool = pool_options.connect_with(connect_options).await?;
        Ok(PostgresPool { pool })
    }

    async fn get_connection(&self) -> Result<Self::Connection, Self::PoolError> {
        // This is where the design choice for PostgresConnection matters.
        // If PostgresConnection is to be a true single connection, we'd get a
        // PoolConnection<Postgres> and wrap it.
        // For now, this is a conceptual placeholder.
        // A proper implementation would be:
        // let conn = self.pool.acquire().await?;
        // Ok(PostgresPooledConnection { conn }) // Where PostgresPooledConnection implements DbConnection
        
        // Simplified: Re-parse options and connect. This is NOT how a pool.get_connection should work.
        // This part needs significant refinement to correctly implement the DbConnection/DbConnectionPool pattern.
        // The current PostgresConnection is for a standalone connection, not one from a pool.
        // For a quick placeholder:
        Err(PostgresConnectorError::Config("get_connection from pool not fully implemented for this PostgresConnection type".to_string()))
    }

    async fn close_pool(self) -> Result<(), Self::PoolError> {
        self.pool.close().await;
        Ok(())
    }
}


// Example of how one might use sqlx::query! (requires DB for prepare or sqlx-data.json)
// async fn example_macro_query(pool: &Pool<Postgres>) -> Result<(), SqlxError> {
//     struct MyData { val: String, count: i32 }
//     let _rows = sqlx::query_as!(MyData, "SELECT 'test' as val, 123 as count")
//         .fetch_all(pool)
//         .await?;
//     Ok(())
// }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::AppConfig; // Assuming AppConfig can provide DB URL

    // Note: These tests would ideally require a running PostgreSQL instance
    // or more sophisticated mocking if `sqlx::test` is not used.
    // For now, they are placeholders demonstrating structure.

    // Helper to get a test DB URL (e.g., from environment or a test config)
    // For CI, this often points to a Dockerized Postgres.
    fn get_test_db_url() -> String {
        // Load from AppConfig or environment variable for testing
        // For simplicity, hardcoding for this example, but NOT recommended for real tests.
        // std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| "postgres://user:pass@localhost:5432/testdb".to_string())
        
        // Let's try to load it from the project's config.toml if possible
        // This requires config.toml to be accessible and correctly formatted.
        if let Ok(app_config) = AppConfig::load() {
            app_config.database_url
        } else {
            // Fallback if config.toml can't be loaded (e.g., in a bare test environment)
            // Ensure your test environment has this variable set or a running DB at this default.
            eprintln!("Warning: Could not load AppConfig for test DB URL. Using default fallback.");
            "postgres://postgres:password@localhost:5432/omnirust_test".to_string()
        }
    }

    #[tokio::test]
    #[ignore] // Ignored because it requires a live database or sqlx-cli prepare
    async fn test_postgres_pool_creation() {
        let db_url = get_test_db_url();
        let config = PostgresConfig {
            database_url: db_url,
            max_connections: Some(5),
            min_connections: Some(1),
            connect_timeout_seconds: Some(5),
            idle_timeout_seconds: Some(300),
            max_lifetime_seconds: Some(1800),
        };
        let pool_result = PostgresPool::new_pool(config).await;
        assert!(pool_result.is_ok(), "Failed to create pool: {:?}", pool_result.err());
        if let Ok(pool) = pool_result {
            pool.close_pool().await.expect("Failed to close pool");
        }
    }
    
    #[tokio::test]
    #[ignore] // Requires live DB
    async fn test_postgres_single_connection_and_query() {
        let db_url = get_test_db_url();
        let connect_options = PgConnectOptions::from_str(&db_url)
            .expect("Failed to parse DB URL for connect options");
            
        let conn_result = PostgresConnection::connect(connect_options).await;
        assert!(conn_result.is_ok(), "Failed to connect: {:?}", conn_result.err());

        if let Ok(mut conn) = conn_result {
            // Example: Create a dummy table if it doesn't exist (not for production code in connector)
            let create_table_res = conn.execute_raw_query(
                "CREATE TABLE IF NOT EXISTS omnirust_test_table (id SERIAL PRIMARY KEY, name TEXT);"
            ).await;
            assert!(create_table_res.is_ok(), "Failed to create test table: {:?}", create_table_res.err());
            
            let query_res = conn.execute_raw_query("SELECT 1").await;
            assert!(query_res.is_ok());
            if let Ok(res_str) = query_res {
                // This check is very basic due to the generic QueryResult
                assert!(res_str.contains("Query executed successfully")); 
            }
            conn.close().await.expect("Failed to close connection");
        }
    }
}
