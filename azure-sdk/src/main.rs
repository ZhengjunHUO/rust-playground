use anyhow::{bail, Result};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref IS_WORKING: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

trait Crud {
    async fn list_folders(&self, path: String) -> Result<Vec<String>>;
    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()>;
}

async fn list(client: &ContainerClient, path: String) -> Result<Vec<String>> {
    let list = if path.is_empty() {
        client.list_blobs()
    } else {
        client.list_blobs().prefix(path)
    };

    let mut rslt = Vec::new();

    let mut stream = list.into_stream();
    while let Some(resp) = stream.next().await {
        println!("[DEBUG] Got a new page");
        match resp {
            Ok(elem) => {
                let blobs: Vec<_> = elem.blobs.blobs().collect();
                blobs.into_iter().for_each(|blob| {
                    rslt.push(blob.name.clone());
                    println!("[DEBUG] {:?}", blob.name)
                });
            }
            _ => bail!("Sth wrong happened"),
        }
    }
    Ok(rslt)
}

impl Crud for ContainerClient {
    async fn list_folders(&self, path: String) -> Result<Vec<String>> {
        if IS_WORKING.lock().is_ok_and(|list| list.is_empty()) {
            println!("[DEBUG] Retrieve info");
            *IS_WORKING.lock().unwrap() = list(self, String::new()).await?;
        }

        Ok(IS_WORKING
            .lock()
            .unwrap()
            .iter()
            .filter(|&elem| elem.starts_with(&path))
            .map(|elem| format!("{}", elem))
            .collect())
    }

    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()> {
        let resp = self
            .blob_client(path)
            .put_block_blob(content.to_vec())
            .await?;
        println!("[DEBUG] Got resp: {:?}", resp);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = String::from("solution-blob-container");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        ClientBuilder::new(account, storage_credentials).container_client(&container);

    /*
        let content = String::from("RTFM");
        container_client
            .put_obj(String::from("bar/baz/readme"), content.as_bytes())
            .await?;
    */
    //println!("Result: {:?}", container_client.list(String::default()).await?);
    println!(
        "Result: {:?}",
        container_client.list_folders(String::from("xva")).await?
    );

    Ok(())
}
