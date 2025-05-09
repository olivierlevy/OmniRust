// Use the omnirust library crate
use omnirust::core::init_logger::{init_logger, log_info, log_error};
use omnirust::core::config::AppConfig;
// NOTE: The following block was duplicated by the previous tool use and is now corrected.
use omnirust::graphql;
use omnirust::graphql::schema::{QueryRoot, MutationRoot, SubscriptionRoot};
use omnirust::websocket::server as websocket_server;
use omnirust::cli::arg_parser::{self, Commands, UtilCommands};
use omnirust::cli::tui_components; // Added TUI components import
use omnirust::utils::string_utils;

use async_graphql::Schema;
use tokio::net::TcpListener;
use std::error::Error;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger early, as it might be used by arg parsing or config loading.
    init_logger();

    // Parse CLI arguments
    let cli_args = arg_parser::parse_args();

    // Handle CLI commands if provided
    if let Some(command) = cli_args.command {
        match command {
            Commands::Test(test_args) => {
                if test_args.list {
                    println!("Listing all tests (not implemented yet)...");
                } else if let Some(case) = &test_args.case {
                    println!("Running test case: {} (not implemented yet)...", case);
                } else {
                    println!("Running all tests (not implemented yet)...");
                }
            }
            Commands::Util(util_args) => {
                match util_args.command {
                    UtilCommands::Reverse { input_string } => {
                        let reversed = string_utils::reverse(&input_string);
                        println!("Reversed string: {}", reversed);
                    }
                    UtilCommands::IsBlank { input_string } => {
                        let is_blank = string_utils::is_blank(&input_string);
                        println!("Is blank: {}", is_blank);
                    }
                }
            }
            Commands::TuiCounter => {
                log_info("Launching TUI Counter application...");
                if let Err(e) = tui_components::run_counter_tui_app() {
                    log_error(&format!("TUI application error: {}", e));
                    // Depending on desired behavior, you might want to return an error code
                    // For now, just log and exit gracefully.
                }
            }
        }
        return Ok(()); // Exit after handling CLI command
    }

    // If no specific CLI command was handled, proceed with default server startup
    log_info("No specific CLI command given, starting default servers...");

    let config = AppConfig::load()?;
    log_info(&format!("Database URL: {}", config.database_url));
    log_info(&format!("Log Level: {}", config.log_level));

    // Instantiate QueryRoot (now an empty struct) and MutationRoot
    let query_root = QueryRoot {}; 
    let mutation_root = MutationRoot {};
    let subscription_root = SubscriptionRoot {}; // Instantiate SubscriptionRoot
    // Build the schema with QueryRoot, MutationRoot, and SubscriptionRoot
    let schema = Schema::build(query_root, mutation_root, subscription_root).finish();

    // Spawn GraphQL server
    let gql_schema = schema.clone(); // Clone schema for the GraphQL server
    tokio::spawn(async move {
        if let Err(e) = graphql::server::start_server(gql_schema).await {
            log_error(&format!("Error starting GraphQL server: {}", e));
        }
    });
    
    // Start REST API Server (example, assuming port 3000)
    // You might want to make the address configurable
    // tokio::spawn(async {
    //     if let Err(e) = omnirust::web::rest_api::start_rest_server("127.0.0.1:3000").await {
    //         log_error(&format!("Error starting REST API server: {}", e));
    //     }
    // });

    // Start WebSocket server
    let ws_listener = TcpListener::bind("127.0.0.1:8080").await?;
    log_info("WebSocket server started at ws://127.0.0.1:8080");

    loop {
        match ws_listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    match accept_async(stream).await {
                        Ok(ws_stream) => {
                            if let Err(e) = websocket_server::handle_connection(ws_stream).await {
                                log_error(&format!("Error handling WebSocket connection: {}", e));
                            }
                        },
                        Err(e) => log_error(&format!("Error accepting WebSocket connection: {}", e)),
                    }
                });
            }
            Err(e) => {
                log_error(&format!("Failed to accept WebSocket client: {}", e));
                // Consider if the loop should break or continue on accept errors
            }
        }
    }

    // Note: The loop above is infinite. Graceful shutdown for servers would need
    // to be handled, perhaps by listening for signals in the main task and
    // coordinating shutdown of spawned server tasks.
    // For now, Ctrl+C will terminate the process.
    // Ok(()) // This line is unreachable due to the infinite loop
}
