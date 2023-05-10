use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::broadcast};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message}; // Add this line

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let tx = tx.clone();

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    let _ = ws_sender.send(Message::Text(msg)).await;
                }
            });

            while let Some(msg) = ws_receiver.next().await {
                let msg = msg.unwrap();
                if msg.is_text() || msg.is_binary() {
                    let msg = match msg {
                        Message::Text(txt) => txt,
                        Message::Binary(bin) => String::from_utf8_lossy(&bin).to_string(),
                        _ => continue,
                    };
                    tx.send(msg).unwrap();
                }
            }
        });
    }
}
