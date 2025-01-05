mod core;
mod utils;
mod networking;
mod models;
mod graphql;
mod websocket;
mod caching;
mod logging;

use crate::core::init_logger::{init_logger, log_info, log_error};
use core::config::AppConfig;
use graphql::schema::QueryRoot;
use async_graphql::Schema;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use tokio::net::TcpListener;
use std::error::Error;
use crate::websocket::server;
use tokio_tungstenite::accept_async; // Add this import

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logger();

    let config = AppConfig::load()?;
    log_info(&format!("Database URL: {}", config.database_url)); // Log database URL
    log_info(&format!("Log Level: {}", config.log_level));       // Log log level

    let schema = Schema::build(QueryRoot { system_status: "System is running".to_string() }, EmptyMutation, EmptySubscription).finish();

    tokio::spawn(async move {
        if let Err(e) = graphql::server::start_server(schema).await {
            log_error(&format!("Error starting GraphQL server: {}", e));
        }
    });

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    log_info("WebSocket server started at ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let stream = stream;
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    if let Err(e) = server::handle_connection(ws_stream).await {
                        log_error(&format!("Error handling connection: {}", e));
                    }
                },
                Err(e) => log_error(&format!("Error accepting WebSocket connection: {}", e)),
            }
        });
    }

    Ok(())
}