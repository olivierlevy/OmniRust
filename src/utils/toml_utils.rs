// src/utils/toml_utils.rs

use serde::{Serialize, de::DeserializeOwned};
use toml;

/// Serializes a Rust data structure into a TOML string.
pub fn to_toml_string<T: Serialize>(value: &T) -> Result<String, toml::ser::Error> {
    toml::to_string(value)
}

/// Deserializes a TOML string into a Rust data structure.
pub fn from_toml_string<'a, T: DeserializeOwned>(toml_str: &'a str) -> Result<T, toml::de::Error> {
    toml::from_str(toml_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Config {
        ip: String,
        port: Option<u16>,
        keys: Vec<String>,
        server: ServerConfig,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ServerConfig {
        host: String,
        threads: u32,
    }

    fn get_test_config() -> Config {
        Config {
            ip: "127.0.0.1".to_string(),
            port: Some(8080),
            keys: vec!["key1".to_string(), "key2".to_string()],
            server: ServerConfig {
                host: "localhost".to_string(),
                threads: 4,
            },
        }
    }

    #[test]
    fn test_to_toml_string() {
        let config = get_test_config();
        let toml_str = to_toml_string(&config).unwrap();

        // Basic checks for TOML structure
        assert!(toml_str.contains("ip = \"127.0.0.1\""));
        assert!(toml_str.contains("port = 8080"));
        assert!(toml_str.contains("keys = [\"key1\", \"key2\"]"));
        assert!(toml_str.contains("[server]"));
        assert!(toml_str.contains("host = \"localhost\""));
        assert!(toml_str.contains("threads = 4"));
    }

    #[test]
    fn test_from_toml_string() {
        let config = get_test_config();
        let toml_str = to_toml_string(&config).unwrap();
        let deserialized_config: Config = from_toml_string(&toml_str).unwrap();
        assert_eq!(config, deserialized_config);
    }

    #[test]
    fn test_from_toml_string_with_optional_field_not_present() {
        let toml_content = r#"
            ip = "0.0.0.0"
            keys = ["test"]
            [server]
            host = "example.com"
            threads = 2
        "#;
        let deserialized: Config = from_toml_string(toml_content).unwrap();
        assert_eq!(deserialized.ip, "0.0.0.0");
        assert_eq!(deserialized.port, None); // Check that optional field is None
        assert_eq!(deserialized.keys, vec!["test".to_string()]);
        assert_eq!(deserialized.server.host, "example.com");
        assert_eq!(deserialized.server.threads, 2);
    }

    #[test]
    fn test_from_toml_string_error() {
        let invalid_toml_str = "ip = 12345 # Port should be a string or in a table";
        let result: Result<Config, _> = from_toml_string(invalid_toml_str);
        assert!(result.is_err());
    }
}
