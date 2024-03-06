use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = String::from("test");
    let blob_name = String::from("rusty");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_client =
        ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);

    blob_client
        .put_block_blob("Rust rocks!")
        .content_type("text/plain")
        .await?;

    let mut result: Vec<u8> = vec![];
    let mut stream = blob_client.get().into_stream();
    while let Some(value) = stream.next().await {
        let mut body = value?.data;
        while let Some(value) = body.next().await {
            let value = value?;
            result.extend(&value);
        }
    }

    println!("result: {}", String::from_utf8_lossy(&result));
    Ok(())
}
