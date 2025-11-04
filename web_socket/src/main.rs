use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error occurred during WebSocket handshake: {}", e);
            return;
        }
    };

    println!("New WebSocket connection established.");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text.to_string());
                if text == "quit" {
                    if let Err(e) = write.send(Message::Close(None)).await {
                        eprintln!("Error responding to client for close: {}", e);
                    }
                    break
                }
                if write.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Ok(Message::Close(frame)) => {
                println!("Client initiated disconnection [{:?}]", frame);
                // if let Err(e) = write.send(Message::Close(None)).await {
                //     eprintln!("Error responding to client for close: {}", e);
                // }
                break;
            }
            Ok(Message::Ping(data)) => {
                if write.send(Message::Pong(data)).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error occurred receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("Connection closed");
}