// Declare modules for each showcase
mod showcase_core;
mod showcase_utils;
mod showcase_data_structures;
mod showcase_cli;
mod showcase_networking;

use omnirust::core::init_logger::{init_logger, log_info};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger from OmniRust
    init_logger();
    log_info("OmniRust Sample Application - Starting");

    // --- 1. Core: Configuration Loading ---
    showcase_core::run_core_showcase();

    // --- 2. Utilities ---
    showcase_utils::run_utils_showcase();

    // --- 3. Data Structures ---
    showcase_data_structures::run_data_structures_showcase();
    
    // --- 4. CLI Argument Parsing ---
    showcase_cli::run_cli_showcase();

    // --- 5. Networking: HTTP Client ---
    if let Err(e) = showcase_networking::run_networking_showcase().await {
        eprintln!("Error in networking showcase: {}", e);
        // Depending on desired behavior, you might want to propagate this error
        // return Err(e); 
    }

    // --- Placeholder for other modules ---
    // WebSocket Client/Server, gRPC, Streaming, Database, Concurrency, ML, Web (REST/Templates/Wasm), Plugins
    println!("\n--- Other Modules (Placeholders for full demonstration) ---");
    println!("    WebSocket Client/Server: (See OmniRust main.rs for server example, client in omnirust::networking)");
    println!("    gRPC: (Requires .proto definitions and server/client implementation)");
    println!("    Streaming Processors: (See omnirust::streaming::processor for traits)");
    println!("    Database Connectors: (See omnirust::database for traits and SQL placeholder)");
    println!("    Concurrency (Scheduler, WorkerPool, LockFree): (See omnirust::concurrency)");
    println!("    Machine Learning (Matrix, Algorithms, Integrations): (See omnirust::ml)");
    println!("    Web Dev (REST API, Templating, Wasm): (See omnirust::web and OmniRust main.rs for REST/Template server examples)");
    println!("    Plugins: (See omnirust::plugins for placeholder)");


    log_info("OmniRust Sample Application - Finished");
    println!("\nOmniRust Sample Application Finished.");
    Ok(())
}
