use omnirust::ai::openai::send_prompt;

#[tokio::test]
async fn test_openai_integration() {
    let api_key = "your_openai_api_key_here";
    let prompt = "Explain the significance of Rust programming language.";
    let response = send_prompt(api_key, prompt).await.unwrap();

    assert!(response.to_lowercase().contains("rust"));
}
