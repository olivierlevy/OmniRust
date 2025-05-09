use omnirust::utils::{string_utils, file_utils, datetime_utils, json_utils, toml_utils, yaml_utils};
use serde::{Serialize, Deserialize}; // Needed for the derive macros

pub fn run_utils_showcase() {
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
                // Consider cleaning up the file if appropriate for the sample's context
                // match std::fs::remove_file(sample_file_path) {
                //     Ok(_) => println!("    Cleaned up {}.", sample_file_path),
                //     Err(e) => println!("    Error cleaning up {}: {}", sample_file_path, e),
                // }
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
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SampleJson { name: String, version: f32 }
    let json_data = SampleJson { name: "omnirust_json_sample".to_string(), version: 0.1 };
    let json_string = json_utils::to_json_string_pretty(&json_data).expect("Failed to serialize to JSON");
    println!("    Serialized JSON:\n{}", json_string);
    let deserialized_json: SampleJson = json_utils::from_json_string(&json_string).expect("Failed to deserialize JSON");
    assert_eq!(json_data, deserialized_json);
    println!("    JSON deserialized successfully.");

    // TOML Utilities
    println!("\n  TOML Utilities:");
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SampleToml { package: PackageToml }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct PackageToml { name: String, version: String }
    let toml_data = SampleToml { package: PackageToml { name: "omnirust_toml_sample".to_string(), version: "0.1.0".to_string() }};
    let toml_string = toml_utils::to_toml_string(&toml_data).expect("Failed to serialize to TOML");
    println!("    Serialized TOML:\n{}", toml_string);
    let deserialized_toml: SampleToml = toml_utils::from_toml_string(&toml_string).expect("Failed to deserialize TOML");
    assert_eq!(toml_data, deserialized_toml);
    println!("    TOML deserialized successfully.");

    // YAML Utilities
    println!("\n  YAML Utilities:");
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SampleYaml { settings: SettingsYaml, users: Vec<String> }
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SettingsYaml { enabled: bool, port: u16 }
    let yaml_data = SampleYaml { settings: SettingsYaml { enabled: true, port: 8080 }, users: vec!["admin".to_string(), "guest".to_string()]};
    let yaml_string = yaml_utils::to_yaml_string(&yaml_data).expect("Failed to serialize to YAML");
    println!("    Serialized YAML:\n{}", yaml_string);
    let deserialized_yaml: SampleYaml = yaml_utils::from_yaml_string(&yaml_string).expect("Failed to deserialize YAML");
    assert_eq!(yaml_data, deserialized_yaml);
    println!("    YAML deserialized successfully.");
}
