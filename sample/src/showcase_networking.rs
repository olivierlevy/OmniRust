use omnirust::networking::http_client::HttpClient;
use omnirust::core::init_logger::{log_info, log_error}; // Assuming log_info and log_error are needed
use serde::Deserialize; // Needed for the derive macro
use serde_json; // For from_str
use anyhow::Result; // For easy error handling

#[derive(Deserialize, Debug)] 
struct Todo {
    #[serde(rename = "userId")]
    _user_id: i32,
    #[serde(rename = "id")] // Added rename for id
    _id: i32,
    title: String,
    #[serde(rename = "completed")] // Added rename for completed
    _completed: bool,
}

pub async fn run_networking_showcase() -> Result<()> {
    println!("\n--- Networking: HTTP Client (example GET) ---");
    let http_client = HttpClient::new(Some(10)); // 10 second timeout
    let url = "https://jsonplaceholder.typicode.com/todos/1";
    log_info(&format!("Sample App: Making HTTP GET request to {}", url));
    
    match http_client.get_raw(url, None, None).await {
        Ok(response) => {
            if response.status().is_success() {
                // Read the text first
                match response.text().await {
                    Ok(text) => {
                        // Attempt to deserialize the text
                        match serde_json::from_str::<Todo>(&text) {
                            Ok(todo) => {
                                log_info(&format!("Sample App: HTTP GET request successful. Title: {}", todo.title));
                                println!("    Fetched Todo: {:?}", todo);
                            }
                            Err(de_err) => {
                                log_error(&format!("Sample App: Failed to deserialize Todo: {}. Raw response: {}", de_err, text));
                                println!("    Failed to deserialize Todo: {}. Raw response below:", de_err);
                                println!("{}", text);
                            }
                        }
                    }
                    Err(text_err) => {
                        log_error(&format!("Sample App: Failed to read response text: {}", text_err));
                        println!("    Failed to read response text: {}", text_err);
                    }
                }
            } else {
                let status = response.status();
                let err_text = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
                log_error(&format!("Sample App: HTTP GET request failed with status {}: {}", status, err_text));
                println!("    HTTP GET request to {} failed with status {}: {}", url, status, err_text);
            }
        }
        Err(e) => {
            log_error(&format!("Sample App: HTTP GET request failed: {}", e));
            println!("    HTTP GET request to {} failed: {}", url, e);
        }
    }
    Ok(())
}
