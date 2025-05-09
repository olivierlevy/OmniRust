// src/utils/json_utils.rs

use serde::{Serialize, de::DeserializeOwned};
use serde_json;

/// Serializes a Rust data structure into a JSON string.
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serializes a Rust data structure into a pretty-printed JSON string.
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

/// Deserializes a JSON string into a Rust data structure.
pub fn from_json_string<'a, T: DeserializeOwned>(json_str: &'a str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        name: String,
        age: u32,
        is_active: bool,
        point: Point,
        tags: Vec<String>,
    }

    fn get_test_data() -> TestData {
        TestData {
            name: "OmniRust".to_string(),
            age: 1,
            is_active: true,
            point: Point { x: 10, y: 20 },
            tags: vec!["rust".to_string(), "json".to_string(), "utility".to_string()],
        }
    }

    #[test]
    fn test_to_json_string() {
        let data = get_test_data();
        let json_str = to_json_string(&data).unwrap();
        // Basic check, exact string can be brittle due to field order
        assert!(json_str.contains("\"name\":\"OmniRust\""));
        assert!(json_str.contains("\"age\":1"));
        assert!(json_str.contains("\"is_active\":true"));
        assert!(json_str.contains("\"point\":{\"x\":10,\"y\":20}"));
        assert!(json_str.contains("\"tags\":[\"rust\",\"json\",\"utility\"]"));
    }

    #[test]
    fn test_to_json_string_pretty() {
        let data = get_test_data();
        let json_str_pretty = to_json_string_pretty(&data).unwrap();
        // Check for newlines and indentation, characteristic of pretty print
        assert!(json_str_pretty.contains("\n"));
        assert!(json_str_pretty.contains("  \"name\": \"OmniRust\""));
    }

    #[test]
    fn test_from_json_string() {
        let data = get_test_data();
        let json_str = to_json_string(&data).unwrap();
        let deserialized_data: TestData = from_json_string(&json_str).unwrap();
        assert_eq!(data, deserialized_data);
    }

    #[test]
    fn test_from_json_string_error() {
        let invalid_json_str = r#"{"name":"Test", "age":"not_a_number"}"#;
        let result: Result<TestData, _> = from_json_string(invalid_json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_point_serialization_deserialization() {
        let p = Point { x: 1, y: 2 };
        let serialized = to_json_string(&p).unwrap();
        assert_eq!(serialized, r#"{"x":1,"y":2}"#);
        let deserialized: Point = from_json_string(&serialized).unwrap();
        assert_eq!(deserialized, p);
    }
}
