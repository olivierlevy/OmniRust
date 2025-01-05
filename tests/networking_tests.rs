use omnirust::networking::http;

#[tokio::test]
async fn test_http_get() {
    let response = http::get("https://httpbin.org/get").await.unwrap();
    assert!(response.contains("\"url\": \"https://httpbin.org/get\""));
}

#[tokio::test]
async fn test_http_post() {
    let body = serde_json::json!({ "key": "value" });
    let response = http::post("https://httpbin.org/post", &body).await.unwrap();
    assert!(response.contains("\"key\": \"value\""));
}
