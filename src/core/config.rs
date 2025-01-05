use config::{Config, File, Environment};
use std::error::Error;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let settings = Config::builder()
            .add_source(File::with_name("config").required(true)) // Load config.toml
            .add_source(Environment::with_prefix("APP").separator("_")) // Allow environment variables to override
            .build()?;

        Ok(AppConfig {
            database_url: settings.get::<String>("database.url")?,
            log_level: settings.get::<String>("log.level")?,
        })
    }
}