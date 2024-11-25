use anyhow::{bail, Result};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

trait Crud {
    async fn list(&self) -> Result<Vec<String>>;
}

impl Crud for ContainerClient {
    async fn list(&self) -> Result<Vec<String>> {
        let list = self.list_blobs();

        let mut rslt = Vec::new();

        let mut stream = list.into_stream();
        while let Some(resp) = stream.next().await {
            println!("[DEBUG] Found something");
            match resp {
                Ok(elem) => {
                    let blobs: Vec<_> = elem.blobs.blobs().collect();
                    blobs.into_iter().for_each(|blob| {
                        rslt.push(blob.name.clone());
                        println!("[DEBUG] {:?}", blob)
                    });
                }
                _ => bail!("Sth wrong happened"),
            }
        }
        Ok(rslt)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = String::from("rustacean");
    //let blob_name = String::from("bar/readme");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        ClientBuilder::new(account, storage_credentials).container_client(&container);
    println!("Result: {:?}", container_client.list().await?);

    Ok(())
}
