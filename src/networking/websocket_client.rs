// src/networking/websocket_client.rs

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream};
use tokio_tungstenite::tungstenite::error::Error as WsError;
use tokio_tungstenite::tungstenite::handshake::client::Response as HandshakeResponse;
use url::Url;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use std::fmt;

type WebSocketStream = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug)]
pub enum WebSocketClientError {
    UrlParseError(url::ParseError),
    ConnectionError(WsError),
    HandshakeError(HandshakeResponse), // Store the whole response for more details
    SendError(WsError),
    ReceiveError(WsError),
    NotConnected,
    AlreadyConnected,
}

impl fmt::Display for WebSocketClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebSocketClientError::UrlParseError(e) => write!(f, "URL parse error: {}", e),
            WebSocketClientError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            WebSocketClientError::HandshakeError(resp) => write!(f, "Handshake error: HTTP {}", resp.status()),
            WebSocketClientError::SendError(e) => write!(f, "Send error: {}", e),
            WebSocketClientError::ReceiveError(e) => write!(f, "Receive error: {}", e),
            WebSocketClientError::NotConnected => write!(f, "Client is not connected"),
            WebSocketClientError::AlreadyConnected => write!(f, "Client is already connected"),
        }
    }
}

impl std::error::Error for WebSocketClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WebSocketClientError::UrlParseError(e) => Some(e),
            WebSocketClientError::ConnectionError(e) => Some(e),
            WebSocketClientError::SendError(e) => Some(e),
            WebSocketClientError::ReceiveError(e) => Some(e),
            _ => None,
        }
    }
}

pub struct WebSocketClient {
    stream: Option<WebSocketStream>,
    url: Url,
}

impl WebSocketClient {
    pub fn new(url_str: &str) -> Result<Self, WebSocketClientError> {
        let url = Url::parse(url_str).map_err(WebSocketClientError::UrlParseError)?;
        Ok(WebSocketClient { stream: None, url })
    }

    pub async fn connect(&mut self) -> Result<(), WebSocketClientError> {
        if self.stream.is_some() {
            return Err(WebSocketClientError::AlreadyConnected);
        }
        let (ws_stream, response) = connect_async(self.url.clone())
            .await
            .map_err(WebSocketClientError::ConnectionError)?;

        // Check if handshake was successful
        if response.status().is_success() || response.status().is_informational() { // 101 is Switching Protocols
             self.stream = Some(ws_stream);
            Ok(())
        } else {
            Err(WebSocketClientError::HandshakeError(response))
        }
    }

    pub async fn send_text(&mut self, text: String) -> Result<(), WebSocketClientError> {
        if let Some(stream) = self.stream.as_mut() {
            stream.send(Message::Text(text)).await.map_err(WebSocketClientError::SendError)
        } else {
            Err(WebSocketClientError::NotConnected)
        }
    }

    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), WebSocketClientError> {
        if let Some(stream) = self.stream.as_mut() {
            stream.send(Message::Binary(data)).await.map_err(WebSocketClientError::SendError)
        } else {
            Err(WebSocketClientError::NotConnected)
        }
    }
    
    pub async fn send_ping(&mut self, data: Vec<u8>) -> Result<(), WebSocketClientError> {
        if let Some(stream) = self.stream.as_mut() {
            stream.send(Message::Ping(data)).await.map_err(WebSocketClientError::SendError)
        } else {
            Err(WebSocketClientError::NotConnected)
        }
    }

    pub async fn receive(&mut self) -> Option<Result<Message, WebSocketClientError>> {
        if let Some(stream) = self.stream.as_mut() {
            stream.next().await.map(|res| res.map_err(WebSocketClientError::ReceiveError))
        } else {
            None // Or return Err(WebSocketClientError::NotConnected) if preferred
        }
    }

    pub async fn close(&mut self, code: Option<u16>, reason: Option<String>) -> Result<(), WebSocketClientError> {
        if let Some(stream) = self.stream.as_mut() {
            let close_frame = tokio_tungstenite::tungstenite::protocol::frame::CloseFrame {
                code: code.map_or(
                    tokio_tungstenite::tungstenite::protocol::CloseCode::Normal, 
                    |c| tokio_tungstenite::tungstenite::protocol::CloseCode::from(c)
                ),
                reason: reason.map_or(std::borrow::Cow::Borrowed(""), |r| std::borrow::Cow::Owned(r)),
            };
            stream.send(Message::Close(Some(close_frame))).await.map_err(WebSocketClientError::SendError)?;
            // The stream will be fully closed when the server acknowledges or after a timeout.
            // For simplicity, we just mark it as None here. Proper handling might involve waiting for the close handshake.
            self.stream = None; 
            Ok(())
        } else {
            Err(WebSocketClientError::NotConnected)
        }
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use futures_util::{SinkExt, StreamExt};

    // Helper to start a simple echo server for testing
    async fn start_test_server(addr: &str) -> tokio::task::JoinHandle<()> {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind test server");
        let addr_clone = addr.to_string(); // Clone for the closure
        tokio::spawn(async move {
            println!("Test WebSocket server listening on ws://{}", addr_clone);
            if let Ok((stream, _)) = listener.accept().await {
                if let Ok(mut websocket) = accept_async(stream).await {
                    println!("Test server: Client connected");
                    while let Some(msg) = websocket.next().await {
                        match msg {
                            Ok(Message::Text(txt)) => {
                                println!("Test server received text: {}", txt);
                                if websocket.send(Message::Text(format!("Echo: {}", txt))).await.is_err() {
                                    break;
                                }
                            }
                            Ok(Message::Binary(bin)) => {
                                println!("Test server received binary: {:?}", bin);
                                if websocket.send(Message::Binary(bin)).await.is_err() {
                                    break;
                                }
                            }
                            Ok(Message::Ping(data)) => {
                                println!("Test server received ping");
                                if websocket.send(Message::Pong(data)).await.is_err() {
                                    break;
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("Test server received close");
                                break;
                            }
                            Err(e) => {
                                eprintln!("Test server error: {}", e);
                                break;
                            }
                            _ => {} // Ignore Pong, etc.
                        }
                    }
                    println!("Test server: Client disconnected");
                }
            }
        })
    }

    #[tokio::test]
    async fn test_websocket_client_connect_send_receive_close() {
        let server_addr = "127.0.0.1:12345";
        let server_handle = start_test_server(server_addr).await;

        let mut client = WebSocketClient::new(&format!("ws://{}", server_addr)).unwrap();
        
        // Test connect
        assert!(!client.is_connected());
        let connect_result = client.connect().await;
        assert!(connect_result.is_ok(), "Connect failed: {:?}", connect_result.err());
        assert!(client.is_connected());

        // Test send text
        let send_text_result = client.send_text("Hello WebSocket".to_string()).await;
        assert!(send_text_result.is_ok(), "Send text failed: {:?}", send_text_result.err());

        // Test receive text
        if let Some(Ok(Message::Text(response))) = client.receive().await {
            assert_eq!(response, "Echo: Hello WebSocket");
        } else {
            panic!("Did not receive expected text message");
        }

        // Test send binary
        let binary_data = vec![1, 2, 3, 4, 5];
        let send_binary_result = client.send_binary(binary_data.clone()).await;
         assert!(send_binary_result.is_ok(), "Send binary failed: {:?}", send_binary_result.err());

        // Test receive binary
        if let Some(Ok(Message::Binary(response_bin))) = client.receive().await {
            assert_eq!(response_bin, binary_data);
        } else {
            panic!("Did not receive expected binary message");
        }
        
        // Test ping
        let ping_data = vec![7,8,9];
        let send_ping_result = client.send_ping(ping_data.clone()).await;
        assert!(send_ping_result.is_ok(), "Send ping failed: {:?}", send_ping_result.err());

        // Test receive pong (server should auto-reply with pong)
        if let Some(Ok(Message::Pong(pong_data))) = client.receive().await {
             assert_eq!(pong_data, ping_data);
        } else {
            panic!("Did not receive expected pong message");
        }

        // Test close
        let close_result = client.close(Some(1000), Some("Goodbye".to_string())).await;
        assert!(close_result.is_ok(), "Close failed: {:?}", close_result.err());
        // is_connected might still be true until the stream is fully dropped or next read fails
        // For this test, we assume close initiated successfully. A more robust check would involve
        // trying to receive again and expecting an error or None.
        // assert!(!client.is_connected()); // This might not be immediately false

        // Allow server to process close and shut down
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        server_handle.abort(); // Stop the test server
    }

    #[tokio::test]
    async fn test_connect_to_nonexistent_server() {
        let mut client = WebSocketClient::new("ws://127.0.0.1:54321").unwrap(); // Non-existent port
        let result = client.connect().await;
        assert!(result.is_err());
        if let Err(WebSocketClientError::ConnectionError(_)) = result {
            // Expected error
        } else {
            panic!("Expected ConnectionError, got {:?}", result);
        }
    }
    
    #[tokio::test]
    async fn test_send_without_connect() {
        let mut client = WebSocketClient::new("ws://127.0.0.1:12346").unwrap();
        let result = client.send_text("test".to_string()).await;
        assert!(matches!(result, Err(WebSocketClientError::NotConnected)));
    }
}
