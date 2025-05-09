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
*   **Web Services**:
    *   REST API framework (e.g., using Axum - see `web::rest_api`)
    *   HTML Templating (e.g., using Askama - see `web::templating`)
    *   WebAssembly (Wasm) support (conceptual, see `web::wasm`)
*   **Database Interaction**:
    *   SQL connectors and traits (conceptual, see `database` module)
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

To run the main OmniRust binary (which might start a server or provide CLI tools, depending on its `main.rs`):
```sh
cargo run
# or, to pass arguments to the OmniRust CLI
cargo run -- util reverse "hello from omnirust cli"
```

Check `src/main.rs` in the OmniRust project root to see what the main binary is configured to do.

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
