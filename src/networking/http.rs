use reqwest::Client;
use serde::Serialize;
use std::error::Error;

/// Performs a GET request to the specified URL.
pub async fn get(url: &str) -> Result<String, Box<dyn Error>> {
    let response = Client::new().get(url).send().await?;
    let text = response.text().await?;
    Ok(text)
}

/// Performs a POST request with JSON data.
pub async fn post<T: Serialize>(url: &str, body: &T) -> Result<String, Box<dyn Error>> {
    let response = Client::new().post(url).json(body).send().await?;
    let text = response.text().await?;
    Ok(text)
}
