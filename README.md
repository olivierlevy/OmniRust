# OmniRust

OmniRust is a comprehensive, modular, and scalable Rust framework designed to accelerate the development of robust applications. It provides a rich set of pre-built components and utilities, covering areas such as backend services, data handling, command-line interfaces, and networking.

## Features

OmniRust offers a wide array of functionalities, including but not limited to:

*   **Core Services**:
    *   Configuration Management (`core::config`)
    *   Asynchronous Logging (`core::init_logger`)
*   **Utility Libraries**:
    *   String manipulation (`utils::string_utils`)
    *   File operations (`utils::file_utils`)
    *   Date and time handling (`utils::datetime_utils`)
    *   JSON, TOML, and YAML parsing/serialization (`utils::json_utils`, `utils::toml_utils`, `utils::yaml_utils`)
*   **Data Structures**:
    *   Tree (`data_structures::tree`)
    *   Graph (`data_structures::graph`)
    *   Circular Buffer (`data_structures::circular_buffer`)
*   **Command-Line Interface (CLI)**:
    *   Argument parsing (`cli::arg_parser`)
    *   Terminal User Interface (TUI) components (conceptual)
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
*   **Machine Learning** (`ml` module):
    *   Matrix operations (`ml::matrix`) using `ndarray`.
    *   Basic Linear Regression algorithm (`ml::algorithms::linear_regression`).
    *   Placeholders for further algorithms and integrations with external ML libraries.
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
