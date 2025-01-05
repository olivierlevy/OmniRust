use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn connect_to_websocket(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    let (write, read) = ws_stream.split();

    // Example: Send a message
    tokio::spawn(async move {
        let _ = write.send(Message::Text("Hello WebSocket!".to_string())).await;
    });

    // Example: Read messages
    read.for_each(|message| async {
        if let Ok(msg) = message {
            println!("Received: {:?}", msg);
        }
    })
    .await;

    Ok(())
}
