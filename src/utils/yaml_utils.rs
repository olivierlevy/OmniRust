// src/utils/yaml_utils.rs

use serde::{Serialize, de::DeserializeOwned};
use serde_yaml;

/// Serializes a Rust data structure into a YAML string.
pub fn to_yaml_string<T: Serialize>(value: &T) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(value)
}

/// Deserializes a YAML string into a Rust data structure.
pub fn from_yaml_string<'a, T: DeserializeOwned>(yaml_str: &'a str) -> Result<T, serde_yaml::Error> {
    serde_yaml::from_str(yaml_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct AppConfig {
        name: String,
        version: String,
        settings: Settings,
        features: Vec<String>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Settings {
        debug_mode: bool,
        port: u16,
    }

    fn get_test_app_config() -> AppConfig {
        AppConfig {
            name: "OmniApp".to_string(),
            version: "1.2.3".to_string(),
            settings: Settings {
                debug_mode: true,
                port: 8080,
            },
            features: vec!["logging".to_string(), "api".to_string(), "ui".to_string()],
        }
    }

    #[test]
    fn test_to_yaml_string() {
        let config = get_test_app_config();
        let yaml_str = to_yaml_string(&config).unwrap();

        // Basic checks for YAML structure. Exact output can vary slightly.
        assert!(yaml_str.contains("name: OmniApp"));
        assert!(yaml_str.contains("version: 1.2.3"));
        assert!(yaml_str.contains("settings:"));
        assert!(yaml_str.contains("  debug_mode: true"));
        assert!(yaml_str.contains("  port: 8080"));
        assert!(yaml_str.contains("features:"));
        assert!(yaml_str.contains("- logging"));
        assert!(yaml_str.contains("- api"));
        assert!(yaml_str.contains("- ui"));
    }

    #[test]
    fn test_from_yaml_string() {
        let config = get_test_app_config();
        let yaml_str = to_yaml_string(&config).unwrap();
        let deserialized_config: AppConfig = from_yaml_string(&yaml_str).unwrap();
        assert_eq!(config, deserialized_config);
    }

    #[test]
    fn test_from_yaml_string_with_different_types() {
        let yaml_content = r#"
name: MyService
version: "0.1.0"
settings:
  debug_mode: false
  port: 3000
features:
  - auth
  - payments
"#;
        let deserialized: AppConfig = from_yaml_string(yaml_content).unwrap();
        assert_eq!(deserialized.name, "MyService");
        assert_eq!(deserialized.version, "0.1.0");
        assert_eq!(deserialized.settings.debug_mode, false);
        assert_eq!(deserialized.settings.port, 3000);
        assert_eq!(deserialized.features, vec!["auth".to_string(), "payments".to_string()]);
    }

    #[test]
    fn test_from_yaml_string_error() {
        // Example of invalid YAML (incorrect indentation for a map item)
        let invalid_yaml_str = "name: TestApp\n version: 1.0\nsettings:\n debug_mode: true\n  port: not_a_port";
        let result: Result<AppConfig, _> = from_yaml_string(invalid_yaml_str);
        assert!(result.is_err());
    }
}
