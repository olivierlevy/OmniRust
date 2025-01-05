use tokio::io::{AsyncRead, AsyncWrite};
use std::error::Error;
use async_graphql::futures_util::{StreamExt, SinkExt};

pub async fn handle_connection<S>(mut ws_stream: tokio_tungstenite::WebSocketStream<S>) -> Result<(), Box<dyn Error>>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    ws_stream.send(msg).await?;
                }
            },
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
