use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("MCP Server running at ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received: {}", text);

                        if let Ok(req) = serde_json::from_str::<Value>(&text) {
                            let id = req.get("id").cloned().unwrap_or(json!(null));
                            let method = req.get("method").and_then(|m| m.as_str());

                            match method {
                                Some("callTool") => {
                                    let tool_name =
                                        req["params"]["name"].as_str().unwrap_or("unknown");
                                    let args = &req["params"]["arguments"];

                                    // Mock response
                                    let result = match tool_name {
                                        "search_weather" => json!({
                                            "city": args["city"].as_str().unwrap_or("unknown"),
                                            "temperature": "18C",
                                            "condition": "Foggy"
                                        }),
                                        _ => json!({"error": "Unknown tool"}),
                                    };

                                    let response = json!({
                                        "jsonrpc": "2.0",
                                        "id": id,
                                        "result": result
                                    });

                                    write
                                        .send(Message::Text(response.to_string().into()))
                                        .await
                                        .unwrap();
                                }
                                _ => {
                                    let response = json!({
                                        "jsonrpc": "2.0",
                                        "id": id,
                                        "error": {"code": -32601, "message": "Method not found"}
                                    });
                                    write
                                        .send(Message::Text(response.to_string().into()))
                                        .await
                                        .unwrap();
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    _ => {}
                }
            }
        });
    }

    Ok(())
}
