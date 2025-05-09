use omnirust::core::config::AppConfig;
use omnirust::core::init_logger::{log_info, log_error};

pub fn run_core_showcase() {
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
}
