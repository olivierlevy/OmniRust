use omnirust::core::config::AppConfig;
use omnirust::core::init_logger::{init_logger, log_info, log_error};
use omnirust::utils::{string_utils, file_utils, datetime_utils, json_utils, toml_utils, yaml_utils};
use omnirust::data_structures::{tree::Tree, graph::Graph, circular_buffer::CircularBuffer};
use omnirust::cli::arg_parser::{self as omni_arg_parser, Commands as OmniCommands, UtilCommands as OmniUtilCommands};
use omnirust::networking::http_client::HttpClient;
// Import other omnirust modules as needed for examples

use anyhow::Result; // For easy error handling in the sample app
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger from OmniRust
    init_logger();
    log_info("OmniRust Sample Application - Starting");

    // --- 1. Core: Configuration Loading ---
    println!("\n--- Core: Configuration ---");
    match AppConfig::load() {
        Ok(config) => {
            log_info(&format!("Sample App: Loaded config. Database URL: {}, Log Level: {}", config.database_url, config.log_level));
            println!("AppConfig loaded successfully.");
            println!("  Database URL: {}", config.database_url);
            println!("  Log Level: {}", config.log_level);
        }
        Err(e) => {
            log_error(&format!("Sample App: Failed to load AppConfig: {}", e));
            println!("Failed to load AppConfig: {}", e);
            println!("Ensure 'config.toml' (or equivalent) is in the current directory, or environment variables are set.");
            println!("A 'config.example.toml' is available in the OmniRust project root.");
        }
    }

    // --- 2. Utilities ---
    println!("\n--- Utilities Showcase ---");

    // String Utilities
    println!("\n  String Utilities:");
    let s1 = "  hello world  ";
    let s2 = "";
    println!("    is_blank(\"{}\"): {}", s1, string_utils::is_blank(s1));
    println!("    is_blank(\"{}\"): {}", s2, string_utils::is_blank(s2));
    println!("    reverse(\"omnirust\"): {}", string_utils::reverse("omnirust"));

    // File Utilities (Basic example, careful with actual file operations in a generic sample)
    println!("\n  File Utilities (example - creates sample_file.txt):");
    let sample_file_path = "sample_file.txt";
    let file_content = "Hello from OmniRust file_utils!";
    match file_utils::write_string_to_file(sample_file_path, file_content) {
        Ok(_) => {
            println!("    Successfully wrote to {}", sample_file_path);
            match file_utils::read_to_string(sample_file_path) {
                Ok(content_read) => println!("    Read from {}: \"{}\"", sample_file_path, content_read),
                Err(e) => println!("    Error reading {}: {}", sample_file_path, e),
            }
            if file_utils::path_exists(sample_file_path) {
                println!("    Path {} exists.", sample_file_path);
                // std::fs::remove_file(sample_file_path)?; // Clean up
            }
        }
        Err(e) => println!("    Error writing to {}: {}", sample_file_path, e),
    }
    
    // DateTime Utilities
    println!("\n  DateTime Utilities:");
    let now = datetime_utils::now_utc();
    println!("    Current UTC time: {}", datetime_utils::format_datetime_utc(&now, None));
    println!("    Year: {}, Month: {}, Day: {}", datetime_utils::year(&now), datetime_utils::month(&now), datetime_utils::day(&now));

    // JSON Utilities
    println!("\n  JSON Utilities:");
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct SampleJson { name: String, version: f32 }
    let json_data = SampleJson { name: "omnirust_json_sample".to_string(), version: 0.1 };
    let json_string = json_utils::to_json_string_pretty(&json_data).expect("Failed to serialize to JSON");
    println!("    Serialized JSON:\n{}", json_string);
    let deserialized_json: SampleJson = json_utils::from_json_string(&json_string).expect("Failed to deserialize JSON");
    assert_eq!(json_data, deserialized_json);
    println!("    JSON deserialized successfully.");

    // TOML Utilities
    println!("\n  TOML Utilities:");
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct SampleToml { package: PackageToml }
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct PackageToml { name: String, version: String }
    let toml_data = SampleToml { package: PackageToml { name: "omnirust_toml_sample".to_string(), version: "0.1.0".to_string() }};
    let toml_string = toml_utils::to_toml_string(&toml_data).expect("Failed to serialize to TOML");
    println!("    Serialized TOML:\n{}", toml_string);
    let deserialized_toml: SampleToml = toml_utils::from_toml_string(&toml_string).expect("Failed to deserialize TOML");
    assert_eq!(toml_data, deserialized_toml);
    println!("    TOML deserialized successfully.");

    // YAML Utilities
    println!("\n  YAML Utilities:");
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct SampleYaml { settings: SettingsYaml, users: Vec<String> }
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct SettingsYaml { enabled: bool, port: u16 }
    let yaml_data = SampleYaml { settings: SettingsYaml { enabled: true, port: 8080 }, users: vec!["admin".to_string(), "guest".to_string()]};
    let yaml_string = yaml_utils::to_yaml_string(&yaml_data).expect("Failed to serialize to YAML");
    println!("    Serialized YAML:\n{}", yaml_string);
    let deserialized_yaml: SampleYaml = yaml_utils::from_yaml_string(&yaml_string).expect("Failed to deserialize YAML");
    assert_eq!(yaml_data, deserialized_yaml);
    println!("    YAML deserialized successfully.");


    // --- 3. Data Structures ---
    println!("\n--- Data Structures Showcase ---");
    // Tree
    println!("\n  Tree Example:");
    let mut tree = Tree::with_root("Root".to_string());
    if let Some(root) = &tree.root {
        let mut root_mut = root.borrow_mut();
        let child1 = root_mut.add_child("Child 1".to_string());
        root_mut.add_child("Child 2".to_string());
        child1.borrow_mut().add_child("Grandchild 1.1".to_string());
    }
    println!("    Tree structure:\n{}", tree);

    // Graph
    println!("\n  Graph Example (Weighted, Directed):");
    let mut graph: Graph<String, i32> = Graph::new();
    let n0 = graph.add_node("Node0".to_string());
    let n1 = graph.add_node("Node1".to_string());
    let n2 = graph.add_node("Node2".to_string());
    graph.add_edge(n0, n1, 10);
    graph.add_edge(n1, n2, 20);
    graph.add_edge(n0, n2, 5); // Shorter path
    println!("    Graph: {:?}, Nodes: {}, Edges: {}", graph, graph.node_count(), graph.edge_count());
    println!("    BFS from Node0: {:?}", graph.bfs(n0));
    println!("    DFS from Node0: {:?}", graph.dfs(n0));

    // Circular Buffer
    println!("\n  Circular Buffer Example (capacity 3):");
    let mut c_buffer: CircularBuffer<i32> = CircularBuffer::new(3);
    c_buffer.push_back(1);
    c_buffer.push_back(2);
    c_buffer.push_back(3);
    println!("    Buffer full: {:?}", c_buffer.iter().collect::<Vec<_>>()); // [1, 2, 3]
    c_buffer.push_back(4); // Overwrites 1
    println!("    Pushed 4 (overwrite): {:?}", c_buffer.iter().collect::<Vec<_>>()); // [2, 3, 4]
    println!("    Popped: {:?}", c_buffer.pop_front()); // Some(2)
    println!("    Buffer after pop: {:?}", c_buffer.iter().collect::<Vec<_>>()); // [3, 4]


    // --- 4. CLI Argument Parsing (demonstrating how the library's CLI module could be used by an app) ---
    // This sample app doesn't take its own CLI args in this main function,
    // but it shows how one *could* use OmniRust's CLI parsing.
    // To test OmniRust's own CLI, you'd run `cargo run --manifest-path ../Cargo.toml -- util reverse "test"`
    println!("\n--- CLI Argument Parsing (Conceptual Example) ---");
    // Simulate some args for OmniRust's parser
    let simulated_args = vec!["omnirust-cli", "util", "reverse", "dlrow olleh"];
    match omni_arg_parser::CliArgs::try_parse_from(&simulated_args) {
        Ok(omni_cli) => {
            println!("    Simulated OmniRust CLI args parsed: {:?}", omni_cli);
            if let Some(OmniCommands::Util(util_args)) = omni_cli.command {
                if let OmniUtilCommands::Reverse { input_string } = util_args.command {
                    println!("    Simulated util reverse input: {}", input_string);
                    println!("    Simulated util reverse output: {}", string_utils::reverse(&input_string));
                }
            }
        }
        Err(e) => println!("    Error parsing simulated OmniRust CLI args: {}", e),
    }

    // --- 5. Networking: HTTP Client ---
    println!("\n--- Networking: HTTP Client (example GET) ---");
    let http_client = HttpClient::new(Some(10)); // 10 second timeout
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    log_info(&format!("Sample App: Making HTTP GET request to {}", url));
    #[derive(serde::Deserialize, Debug)]
    struct Todo {
        #[serde(rename = "userId")]
        user_id: i32,
        id: i32,
        title: String,
        completed: bool,
    }
    match http_client.get::<Todo>(url, None, None).await {
        Ok(todo) => {
            log_info(&format!("Sample App: HTTP GET request successful. Title: {}", todo.title));
            println!("    Fetched Todo: {:?}", todo);
        }
        Err(e) => {
            log_error(&format!("Sample App: HTTP GET request failed: {}", e));
            println!("    HTTP GET request to {} failed: {}", url, e);
        }
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
