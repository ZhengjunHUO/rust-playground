use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{SinkExt, StreamExt};

#[derive(Debug, Serialize, Deserialize)]
struct ToolCall {
    tool: String,
    arguments: serde_json::Value,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Model says "call search_weather"
    let model_output = r#"
    {
        "tool": "search_weather",
        "arguments": { "city": "San Francisco" }
    }
    "#;

    let tool_call: ToolCall = serde_json::from_str(model_output)?;

    // 2. Connect to MCP server
    let url = Url::parse("ws://localhost:8080")?;
    let (ws_stream, _) = connect_async(&url).await?;
    let (mut write, mut read) = ws_stream.split();

    // 3. Construct MCP-compliant JSON-RPC request
    let request_id = 1;
    let mcp_request = json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "callTool",
        "params": {
            "name": tool_call.tool,
            "arguments": tool_call.arguments
        }
    });

    // Send request
    write
        .send(Message::Text(mcp_request.to_string().into()))
        .await?;

    // 4. Wait for response from MCP server
    if let Some(msg) = read.next().await {
        let response = msg?;
        println!("MCP Server Response: {}", response);
    }

    Ok(())
}
