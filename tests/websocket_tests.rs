use omnirust::networking::websocket::connect_to_websocket;

#[tokio::test]
async fn test_websocket() {
    let url = "wss://echo.websocket.org"; // Example WebSocket echo server
    let result = connect_to_websocket(url).await;
    assert!(result.is_ok());
}
