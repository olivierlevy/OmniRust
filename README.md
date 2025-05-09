# OmniRust

OmniRust is a comprehensive, modular, and scalable Rust framework designed to accelerate the development of robust applications. It provides a rich set of pre-built components and utilities, covering areas such as backend services, data handling, command-line interfaces, and networking.

## Features

OmniRust offers a wide array of functionalities, including but not limited to:

*   **Core Services**:
*   Configuration Management (`core::config`): Supports TOML, YAML, and JSON files, and environment variables.
*   Advanced Logging (`logging::logger`): Uses `tracing` for structured, configurable logging.
*   Custom Error Handling (`core::errors`): Centralized `OmniRustError` type.
*   **Utility Libraries**:
*   String manipulation (`utils::string_utils`)
*   Asynchronous File operations (`utils::file_utils`) using `tokio::fs`.
    *   Date and time handling (`utils::datetime_utils`)
    *   JSON, TOML, and YAML parsing/serialization (`utils::json_utils`, `utils::toml_utils`, `utils::yaml_utils`)
*   **Data Structures**:
    *   Tree (`data_structures::tree`)
    *   Graph (`data_structures::graph`)
    *   Circular Buffer (`data_structures::circular_buffer`)
    *   Priority Queue (`data_structures::priority_queue`)
*   **Command-Line Interface (CLI)** (`cli` module):
    *   Argument parsing (`cli::arg_parser`) using `clap`.
    *   Basic TUI (Terminal User Interface) example: a counter application (`cli::tui_components`) using `ratatui` and `crossterm`.
*   **Networking**:
    *   Asynchronous HTTP Client (`networking::http_client`)
    *   WebSocket Client & Server (conceptual, see `websocket` and `networking` modules)
    *   gRPC integration (conceptual, see `networking::grpc` module)
*   **Web Services** (`web` module):
    *   REST API framework (using Axum in `web::rest_api`):
        *   Example routes for basic CRUD operations.
        *   Basic token-based authentication middleware (`web::auth`).
        *   Input validation for request payloads (using `validator` crate).
    *   HTML Templating (using Askama in `web::templating`).
    *   GraphQL API Server (`graphql` module).
    *   WebAssembly (Wasm) support (conceptual in `web::wasm`).
*   **Database Interaction** (`database` module):
    *   Generic traits for `DbConnection` and `DbConnectionPool`.
    *   PostgreSQL connector (`sql_connector.rs`) implementing these traits using `sqlx`.
    *   Support for raw SQL queries and typed queries (mapping rows to structs via `sqlx::FromRow`).
    *   Basic transaction management (BEGIN, COMMIT, ROLLBACK).
*   **Concurrency Tools**:
    *   Schedulers, Worker Pools, Lock-Free data structures (conceptual, see `concurrency` module)
*   **Caching**:
    *   Flexible caching mechanisms (`caching` module)
*   **Machine Learning**:
    *   Matrix operations, algorithms, integrations (conceptual, see `ml` module)
*   **Plugin System**:
    *   Support for dynamic plugin loading (conceptual, see `plugins` module)

## Setup

1.  **Clone the repository**:
    ```sh
    git clone https://github.com/your-repo/omnirust.git # Replace with the actual repository URL
    cd omnirust
    ```

2.  **Build the project**:
    This will build both the OmniRust library and the sample application.
    ```sh
    cargo build
    ```
    To build in release mode:
    ```sh
    cargo build --release
    ```

## Running OmniRust

OmniRust itself is primarily a library, but it also includes a main binary that can demonstrate some of its server capabilities or act as a CLI tool.

### Running the Main Application (Server/CLI)

To run the main OmniRust binary (which starts the GraphQL and WebSocket servers by default if no CLI command is given):
```sh
cargo run
```
The GraphQL server will typically be available at `http://127.0.0.1:PORT/graphql` (check `src/graphql/server.rs` or `src/main.rs` for the exact port, often 8000 or similar). You can use a GraphQL client like Postman or Insomnia to interact with it.

To pass arguments to the OmniRust CLI:
```sh
cargo run -- util reverse "hello from omnirust cli"
# To run the TUI counter demo:
cargo run -- tui-counter
```

Check `src/main.rs` in the OmniRust project root to see what the main binary is configured to do.

## Usage

### GraphQL API

OmniRust includes a GraphQL server powered by `async-graphql`. When the main application is run (via `cargo run` without specific CLI commands), it starts a GraphQL server.

**Available Queries:**

*   `systemStatus: String`: Returns the current operational status of the system.
    ```graphql
    query {
      systemStatus
    }
    ```
*   `items: [Item!]!`: Retrieves a list of all available items.
    ```graphql
    query {
      items {
        id
        name
      }
    }
    ```
*   `item(id: ID!): Item`: Retrieves a specific item by its ID.
    ```graphql
    query {
      item(id: "1") {
        id
        name
      }
    }
    ```

**Available Mutations:**

*   `addItem(name: String!): Item!`: Adds a new item with the given name.
    ```graphql
    mutation {
      addItem(name: "New Awesome Item") {
        id
        name
      }
    }
    ```
*   `updateItem(id: ID!, name: String): Item`: Updates the name of an existing item. Returns the updated item or `null` if not found.
    ```graphql
    mutation {
      updateItem(id: "1", name: "Updated Item Name") {
        id
        name
      }
    }
    ```
*   `deleteItem(id: ID!): Boolean!`: Deletes an item by its ID. Returns `true` if successful, `false` otherwise.
    ```graphql
    mutation {
      deleteItem(id: "1")
    }
    ```

**Available Subscriptions:**

*   `itemEvents: ItemEvent!`: Subscribes to real-time events for items (ADDED, UPDATED, DELETED).
    ```graphql
    subscription {
      itemEvents {
        eventType
        item {
          id
          name
        }
      }
    }
    ```

**Item Type:**
```graphql
type Item {
  id: ID!
  name: String!
}

type ItemEvent {
  eventType: String!
  item: Item!
}
```
(Currently, items are stored in an in-memory list. Future enhancements could connect this to the database module.)

### WebSocket Communication

Example message (details depend on the `websocket_server::handle_connection` implementation):
```json
{
  "type": "sendMessage",
  "content": "Hello, OmniRust!"
}
```

## OmniRust Sample Application (`omnirust_sample_app`)

A sample application is provided in the `sample/` directory to demonstrate how to use various features of the OmniRust framework.

### Purpose

The sample application (`omnirust_sample_app`) showcases:
*   Loading application configuration.
*   Using various utility functions (strings, files, datetime, JSON, TOML, YAML).
*   Implementing and using common data structures (Tree, Graph, Circular Buffer).
*   Simulating the use of OmniRust's CLI argument parser.
*   Making HTTP requests with the networking client.

### Running the Sample Application

To run the sample application:
```sh
cargo run -p omnirust_sample_app
```
Alternatively, navigate to the sample directory and run:
```sh
cd sample
cargo run
cd ..
```

### Structure of the Sample Application

The sample application's `main.rs` (`sample/src/main.rs`) is organized into modules, each demonstrating a specific set of OmniRust features:

*   `showcase_core.rs`: Demonstrates core functionalities like configuration loading.
*   `showcase_utils.rs`: Shows examples of using string, file, datetime, JSON, TOML, and YAML utilities.
*   `showcase_data_structures.rs`: Illustrates the usage of Tree, Graph, and CircularBuffer data structures.
*   `showcase_cli.rs`: Provides a conceptual example of how OmniRust's CLI argument parser can be used.
*   `showcase_networking.rs`: Demonstrates making an HTTP GET request using OmniRust's HTTP client.

## User Guide

This section provides a brief guide on how to use some of the key features and recent improvements in OmniRust.

### Configuration

OmniRust's configuration is managed by the `core::config` module. You can configure the application using TOML, YAML, or JSON files, or by setting environment variables.

**Configuration Files:**

Create a `config.toml`, `config.yaml`, or `config.json` file in the root of your project (or the directory where the application binary is run).

Example `config.toml`:
```toml
database_url = "postgres://user:pass@host/db"
log_level = "debug" # Can be trace, debug, info, warn, error
```

Example `config.yaml`:
```yaml
database_url: "postgres://user:pass@host/db"
log_level: "debug"
```

Example `config.json`:
```json
{
  "database_url": "postgres://user:pass@host/db",
  "log_level": "debug"
}
```

**Environment Variables:**

Override file configurations using environment variables prefixed with `APP_`. For nested keys, use `_`.

```sh
export APP_DATABASE_URL="postgres://another_user:another_pass@another_host/another_db"
export APP_LOG_LEVEL="trace"
```

The application will load these in the order: `config.toml`, `config.yaml`, `config.json`, then environment variables (with later sources overriding earlier ones).

### Logging

Logging is handled by the `tracing` crate, configured via `logging::logger`. The log level is set by the `log_level` field in the configuration (see above) or the `RUST_LOG` environment variable.

Logs include timestamps, level, source file, line number, and thread ID.

Example usage in your code:
```rust
use tracing::{info, warn, error, debug, trace};

fn my_function() {
    trace!("This is a detailed trace message.");
    debug!("Debugging information for my_function.");
    info!("my_function executed successfully.");
    warn!("Something looks a bit off here.");
    error!("A critical error occurred in my_function!");
}
```

### Priority Queue

The `data_structures::priority_queue::PriorityQueue` provides a generic priority queue.

```rust
use omnirust::data_structures::priority_queue::PriorityQueue;

let mut pq = PriorityQueue::new();
pq.push("Urgent Task", 10);
pq.push("Normal Task", 5);
pq.push("Low Prio Task", 1);

assert_eq!(pq.pop(), Some("Urgent Task"));
```

### Error Handling

The framework uses a custom `OmniRustError` enum defined in `core::errors`. This allows for consistent error handling. Many utility functions and core components will return `Result<T, OmniRustError>`.

```rust
use omnirust::core::errors::OmniRustError;
use omnirust::utils::file_utils::read_to_string; // Example async function

async fn process_file(path: &str) -> Result<String, OmniRustError> {
    let content = read_to_string(path).await?;
    // ... process content
    Ok(content)
}
```

### Asynchronous File Utilities

The `utils::file_utils` module provides asynchronous versions of common file operations.

```rust
use omnirust::utils::file_utils::{read_to_string, write_string_to_file, path_exists};
use omnirust::core::errors::OmniRustError;

#[tokio::main]
async fn main() -> Result<(), OmniRustError> {
    let file_path = "my_async_file.txt";
    let content = "Hello from async OmniRust!";

    write_string_to_file(file_path, content).await?;
    assert!(path_exists(file_path).await);

    let read_content = read_to_string(file_path).await?;
    assert_eq!(read_content, content);

    tokio::fs::remove_file(file_path).await?; // Clean up
    Ok(())
}
```
The output of the sample application will display the results of these showcases in the console.

## Key Dependencies

OmniRust leverages several powerful crates from the Rust ecosystem. Some of the key ones include:

*   `tokio`: Asynchronous runtime.
*   `serde`: Framework for serializing and deserializing Rust data structures efficiently and generically.
*   `serde_json`, `toml`, `serde_yaml`: For specific data format handling.
*   `reqwest`: Ergonomic, asynchronous HTTP Client.
*   `clap`: For command-line argument parsing.
*   `async-graphql`: GraphQL server library.
*   `axum`: Web application framework.
*   `askama`: Template rendering engine.
*   `sqlx`: Asynchronous SQL toolkit.
*   `tracing`: Application-level tracing.
*   `cached`: Caching macros.
*   `anyhow`, `thiserror`: Error handling.

(This list is not exhaustive. Refer to `Cargo.toml` for the full list of dependencies.)
