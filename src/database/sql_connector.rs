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

/// Represents a connection acquired from a `PostgresPool`.
/// Implements the `DbConnection` trait.
#[derive(Debug)]
pub struct PostgresPooledConnection {
    // sqlx::pool::PoolConnection<Postgres> automatically returns the connection to the pool when dropped.
    conn: sqlx::pool::PoolConnection<Postgres>,
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
        match self.conn.execute(query).await {
            Ok(result) => Ok(format!("Query executed successfully. Rows affected: {}", result.rows_affected())),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    async fn query_typed<T>(&mut self, query: &str) -> Result<Vec<T>, Self::ConnectionError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&mut self.conn)
            .await
            .map_err(PostgresConnectorError::Sqlx)
    }

    async fn begin_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        self.conn.execute("BEGIN").await?;
        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        self.conn.execute("COMMIT").await?;
        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        self.conn.execute("ROLLBACK").await?;
        Ok(())
    }

    async fn close(mut self) -> Result<(), Self::ConnectionError> {
        self.conn.close().await?; // Closes the standalone connection
        Ok(())
    }
}

#[async_trait]
impl DbConnection for PostgresPooledConnection {
    type Config = PgConnectOptions; // Not used for pooled connections directly via this trait
    type ConnectionError = PostgresConnectorError;

    async fn connect(_config: Self::Config) -> Result<Self, Self::ConnectionError> {
        // This method should not be called directly on a PooledConnection.
        // Connections are obtained from the pool.
        Err(PostgresConnectorError::Config(
            "Cannot call connect directly on a PooledConnection. Get it from a pool.".to_string()
        ))
    }

    async fn execute_raw_query(&mut self, query: &str) -> QueryResult {
        match self.conn.execute(query).await {
            Ok(result) => Ok(format!("Query executed successfully. Rows affected: {}", result.rows_affected())),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
        }
    }

    async fn query_typed<T>(&mut self, query: &str) -> Result<Vec<T>, Self::ConnectionError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&mut *self.conn) // Deref to get &mut PgConnection from &mut PoolConnection
            .await
            .map_err(PostgresConnectorError::Sqlx)
    }

    async fn begin_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        // sqlx::Transaction can be started from a &mut PoolConnection
        // However, to keep the DbConnection trait simple (not returning a Transaction guard),
        // we'll use raw SQL here too. This means the transaction is managed by the DB session,
        // not by an explicit Transaction object in Rust.
        self.conn.execute("BEGIN").await?;
        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        self.conn.execute("COMMIT").await?;
        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), Self::ConnectionError> {
        self.conn.execute("ROLLBACK").await?;
        Ok(())
    }

    async fn close(self) -> Result<(), Self::ConnectionError> {
        // For a pooled connection, `close` means returning it to the pool.
        // `PoolConnection` does this on Drop. So, this can be a no-op or explicit drop.
        // Explicitly dropping is fine.
        drop(self.conn);
        Ok(())
    }
}


#[async_trait]
impl DbConnectionPool for PostgresPool {
    type Connection = PostgresPooledConnection; // Changed to PostgresPooledConnection
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
        let conn = self.pool.acquire().await?;
        Ok(PostgresPooledConnection { conn })
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
    use crate::models::user::DbUser; // Import DbUser for typed queries

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

    #[tokio::test]
    #[ignore] // Requires live DB and assumes table 'db_users' can be created/dropped
    async fn test_postgres_query_typed() {
        let db_url = get_test_db_url();
        let connect_options = PgConnectOptions::from_str(&db_url)
            .expect("Failed to parse DB URL for connect options");
            
        let conn_result = PostgresConnection::connect(connect_options).await;
        assert!(conn_result.is_ok(), "Failed to connect for query_typed test: {:?}", conn_result.err());

        if let Ok(mut conn) = conn_result {
            // Setup: Ensure table exists and is empty
            let _ = conn.execute_raw_query("DROP TABLE IF EXISTS db_users;").await; // Drop if exists
            let create_res = conn.execute_raw_query(
                "CREATE TABLE db_users (id SERIAL PRIMARY KEY, name TEXT NOT NULL, email TEXT);"
            ).await;
            assert!(create_res.is_ok(), "Failed to create db_users table: {:?}", create_res.err());

            // Insert test data
            let insert1_res = conn.execute_raw_query("INSERT INTO db_users (name, email) VALUES ('Alice', 'alice@example.com');").await;
            assert!(insert1_res.is_ok());
            let insert2_res = conn.execute_raw_query("INSERT INTO db_users (name) VALUES ('Bob');").await;
            assert!(insert2_res.is_ok());

            // Call query_typed
            let users_result = conn.query_typed::<DbUser>("SELECT id, name, email FROM db_users ORDER BY name ASC;").await;
            assert!(users_result.is_ok(), "query_typed failed: {:?}", users_result.err());

            if let Ok(users) = users_result {
                assert_eq!(users.len(), 2);
                
                assert_eq!(users[0].name, "Alice");
                assert_eq!(users[0].email, Some("alice@example.com".to_string()));
                
                assert_eq!(users[1].name, "Bob");
                assert_eq!(users[1].email, None);
            }

            // Teardown (optional, if tests run in isolated DBs/schemas)
            // let _ = conn.execute_raw_query("DROP TABLE db_users;").await;

            conn.close().await.expect("Failed to close connection");
        }
    }

    #[tokio::test]
    #[ignore] // Requires live DB
    async fn test_postgres_transaction_commit_and_rollback() {
        let db_url = get_test_db_url();
        let connect_options = PgConnectOptions::from_str(&db_url)
            .expect("Failed to parse DB URL for connect options");

        // Test Commit
        {
            let mut conn = PostgresConnection::connect(connect_options.clone()).await.expect("Failed to connect for commit test");
            
            // Setup table
            let _ = conn.execute_raw_query("DROP TABLE IF EXISTS transaction_test_table;").await;
            let create_res = conn.execute_raw_query("CREATE TABLE transaction_test_table (id INT PRIMARY KEY, name TEXT);").await;
            assert!(create_res.is_ok(), "Failed to create transaction_test_table: {:?}", create_res.err());

            conn.begin_transaction().await.expect("Failed to begin transaction for commit");
            let insert_res = conn.execute_raw_query("INSERT INTO transaction_test_table (id, name) VALUES (1, 'Commit Test');").await;
            assert!(insert_res.is_ok(), "Insert failed within transaction: {:?}", insert_res.err());
            conn.commit_transaction().await.expect("Failed to commit transaction");

            // Verify data is present after commit
            let users_result = conn.query_typed::<DbUser>("SELECT id, name, NULL as email FROM transaction_test_table WHERE id = 1;").await;
            assert!(users_result.is_ok(), "Query after commit failed: {:?}", users_result.err());
            let users = users_result.unwrap();
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].name, "Commit Test");
            
            conn.close().await.expect("Failed to close connection after commit test");
        }

        // Test Rollback
        {
            let mut conn = PostgresConnection::connect(connect_options.clone()).await.expect("Failed to connect for rollback test");
            // Table should still exist with data from commit test
            
            conn.begin_transaction().await.expect("Failed to begin transaction for rollback");
            let insert_res = conn.execute_raw_query("INSERT INTO transaction_test_table (id, name) VALUES (2, 'Rollback Test');").await;
            assert!(insert_res.is_ok(), "Insert failed within transaction: {:?}", insert_res.err());
            conn.rollback_transaction().await.expect("Failed to rollback transaction");

            // Verify data with id=2 is NOT present after rollback
            let users_result = conn.query_typed::<DbUser>("SELECT id, name, NULL as email FROM transaction_test_table WHERE id = 2;").await;
            assert!(users_result.is_ok(), "Query after rollback failed: {:?}", users_result.err());
            let users = users_result.unwrap();
            assert!(users.is_empty(), "Data from rolled-back transaction should not be present.");

            // Verify original data (id=1) is still there
            let original_data_result = conn.query_typed::<DbUser>("SELECT id, name, NULL as email FROM transaction_test_table WHERE id = 1;").await;
            assert!(original_data_result.is_ok());
            assert_eq!(original_data_result.unwrap().len(), 1, "Original committed data should still exist.");

            // Cleanup
            let _ = conn.execute_raw_query("DROP TABLE transaction_test_table;").await;
            conn.close().await.expect("Failed to close connection after rollback test");
        }
    }

    #[tokio::test]
    #[ignore] // Requires live DB
    async fn test_postgres_pool_get_connection_and_use() {
        let db_url = get_test_db_url();
        let config = PostgresConfig {
            database_url: db_url,
            max_connections: Some(2),
            min_connections: Some(1),
            connect_timeout_seconds: Some(5),
            idle_timeout_seconds: Some(300),
            max_lifetime_seconds: Some(1800),
        };
        let pool = PostgresPool::new_pool(config).await.expect("Failed to create pool for get_connection test");

        { // Scope for the pooled connection
            let conn_result = pool.get_connection().await;
            assert!(conn_result.is_ok(), "Failed to get connection from pool: {:?}", conn_result.err());
            if let Ok(mut pooled_conn) = conn_result {
                // Setup: Ensure table exists and is empty
                let _ = pooled_conn.execute_raw_query("DROP TABLE IF EXISTS pool_test_users;").await;
                let create_res = pooled_conn.execute_raw_query(
                    "CREATE TABLE pool_test_users (id SERIAL PRIMARY KEY, name TEXT NOT NULL, email TEXT);"
                ).await;
                assert!(create_res.is_ok(), "Failed to create pool_test_users table: {:?}", create_res.err());

                let insert_res = pooled_conn.execute_raw_query("INSERT INTO pool_test_users (name, email) VALUES ('Pool User', 'pool@example.com');").await;
                assert!(insert_res.is_ok());

                let users_result = pooled_conn.query_typed::<DbUser>("SELECT id, name, email FROM pool_test_users WHERE name = 'Pool User';").await;
                assert!(users_result.is_ok(), "query_typed on pooled connection failed: {:?}", users_result.err());
                
                if let Ok(users) = users_result {
                    assert_eq!(users.len(), 1);
                    assert_eq!(users[0].name, "Pool User");
                }
                // Connection is returned to pool when pooled_conn is dropped here (at end of scope)
                // Or by calling pooled_conn.close().await;
            }
        } // pooled_conn is dropped here

        pool.close_pool().await.expect("Failed to close pool");
    }
}
