use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::io::{AsyncBufReadExt, BufReader};
//use tokio::time::{timeout, Duration};
use futures_util::{StreamExt, SinkExt};
use std::error::Error;

// async fn close_connection_gracefully(
//     mut write: futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
//     mut read: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>
// ) -> Result<(), Box<dyn Error>> {
//     write.send(Message::Close(None)).await?;
//     println!("Sent close frame, waiting for server response...");

//     // Wait for close response with timeout
//     match timeout(Duration::from_secs(5), async {
//         while let Some(msg) = read.next().await {
//             if let Ok(Message::Close(_)) = msg {
//                 return true;
//             }
//         }
//         false
//     }).await {
//         Ok(true) => println!("Received close confirmation"),
//         Ok(false) => println!("Connection closed without confirmation"),
//         Err(_) => println!("Timeout waiting for close confirmation"),
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "ws://127.0.0.1:8080";
    
    println!("Try connecting to {} ...", url);
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connection established!");

    let (mut write, mut read) = ws_stream.split();

    // write.send(Message::Text("Hello from Rust client!".to_string().into())).await?;
    // println!("Hello message sent.");

    // while let Some(msg) = read.next().await {
    //     match msg {
    //         Ok(Message::Text(text)) => {
    //             println!("Received: {}", text.to_string());
    //         }
    //         Ok(Message::Close(_)) => {
    //             println!("Connection closed by server");
    //             break;
    //         }
    //         Err(e) => {
    //             eprintln!("Error occurred: {}", e);
    //             break;
    //         }
    //         _ => {}
    //     }
    // }

    // println!("Connection closed gracefully.");


    let read_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("← Received: {}", text),
                Ok(Message::Close(_)) => {
                    println!("Connection closed by server");
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        println!("Reader exit.");
    });

    let write_task = tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    // if trimmed == "quit" {
                    //     println!("Closing ...");
                    //     if write.send(Message::Close(None)).await.is_err() {
                    //         println!("Error occurred sending closing message to server");
                    //         break;
                    //     }
                    // } else if !trimmed.is_empty() {
                    if !trimmed.is_empty() {
                        if write.send(Message::Text(trimmed.to_string().into())).await.is_err() {
                            break;
                        }
                        println!("→ Sent: {}", trimmed);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading stdin: {}", e);
                    break;
                }
            }
        }
        println!("Writer exit.");
    });

    tokio::select! {
        _ = read_task => {},
        _ = write_task => {},
    }

    // let _ = read_task.await;

    Ok(())
}