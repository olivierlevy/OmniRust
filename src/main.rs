// Use the omnirust library crate
use omnirust::core::init_logger::{init_logger, log_info, log_error};
use omnirust::core::config::AppConfig;
use omnirust::graphql; // Keep this for schema and server
use omnirust::graphql::schema::QueryRoot; // Specific import for QueryRoot
use omnirust::websocket::server as websocket_server; // Alias to avoid conflict if main also defines 'server'

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use tokio::net::TcpListener;
use std::error::Error;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    let config = AppConfig::load()?;
    log_info(&format!("Database URL: {}", config.database_url)); // Log database URL
    log_info(&format!("Log Level: {}", config.log_level));       // Log log level

    // The QueryRoot might need to be adjusted if its definition or dependencies change
    // For now, assuming it's self-contained or its dependencies are correctly handled within the graphql module
    let schema = Schema::build(QueryRoot { system_status: "System is running".to_string() }, EmptyMutation, EmptySubscription).finish();

    tokio::spawn(async move {
        if let Err(e) = graphql::server::start_server(schema).await {
            log_error(&format!("Error starting GraphQL server: {}", e));
        }
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    log_info("WebSocket server started at ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        // let stream = stream; // This line is redundant
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    if let Err(e) = websocket_server::handle_connection(ws_stream).await {
                        log_error(&format!("Error handling connection: {}", e));
                    }
                },
                Err(e) => log_error(&format!("Error accepting WebSocket connection: {}", e)),
            }
        });
    }

    Ok(())
}
