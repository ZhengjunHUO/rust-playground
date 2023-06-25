use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = client::connect("127.0.0.1:6379").await?;
    let key = "foo";

    conn.set(key, "bar".into()).await?;
    let rslt = conn.get(key).await?;

    println!("Server response: {:?}", rslt);

    Ok(())
}
