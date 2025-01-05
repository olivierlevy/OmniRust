use serde::Deserialize;
use std::fs;

use super::common::OmniResult;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub port: u16,
}

impl Config {
    pub fn load_from_file(path: &str) -> OmniResult<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| super::common::OmniError::Parse(e.to_string()))?;
        Ok(config)
    }
}
