use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut stream = tokio_stream::iter(vec!["Rust", "Rusty", "Rustacean"]);

    println!("Stream returns values: ");
    while let Some(val) = stream.next().await {
        println!("  {:?}", val);
    }
}
