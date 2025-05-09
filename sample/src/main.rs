// Use modules from the omnirust_sample_app library
use omnirust_sample_app::showcase_core;
use omnirust_sample_app::showcase_utils;
use omnirust_sample_app::showcase_data_structures;
use omnirust_sample_app::showcase_cli;
use omnirust_sample_app::showcase_networking;

use omnirust::logging::logger::init_logger; // Updated logger import
use omnirust::core::config::AppConfig;    // For loading config
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = AppConfig::load().expect("Failed to load AppConfig for sample app");

    // Initialize logger from OmniRust using the loaded configuration
    init_logger(&config).expect("Failed to initialize logger for sample app");
    tracing::info!("OmniRust Sample Application - Starting"); // Use tracing macro

    // --- 1. Core: Configuration Loading ---
    showcase_core::run_core_showcase();

    // --- 2. Utilities ---
    showcase_utils::run_utils_showcase();

    // --- 3. Data Structures ---
    showcase_data_structures::run_data_structures_showcase();
    showcase_data_structures::run_priority_queue_showcase(); // Added PriorityQueue showcase
    
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


    tracing::info!("OmniRust Sample Application - Finished"); // Use tracing macro
    println!("\nOmniRust Sample Application Finished.");
    Ok(())
}
