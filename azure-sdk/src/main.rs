use anyhow::{bail, Result};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use std::{collections::HashSet, io::Read, sync::Mutex};

lazy_static! {
    pub(crate) static ref REMOTE_STORAGE_CACHE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

trait Crud {
    /// List all folders under path in bucket
    async fn list_folders(&self, path: String) -> Result<Vec<String>>;
    /// List all files under path in bucket
    async fn list_files(&self, path: String) -> Result<Vec<String>>;
    /// Read the object's content
    async fn get_obj(&self, path: String) -> Result<String>;
    /// Create an object in bucket
    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()>;
    /// Should delete recursively all the objects inside if the path is a "folder"
    async fn del_obj(&self, path: String) -> Result<()>;
    async fn put_obj_stream(&self, dump_name: &str, s3_path: String) -> Result<()>;
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

async fn check_or_provision_cache(client: &ContainerClient) -> Result<()> {
    if REMOTE_STORAGE_CACHE
        .lock()
        .is_ok_and(|list| list.is_empty())
    {
        println!("[DEBUG] Retrieve info from remote ...");
        *REMOTE_STORAGE_CACHE.lock().unwrap() = list(client, String::new()).await?;
    }
    Ok(())
}

fn files_under_path(path: &str) -> Vec<String> {
    REMOTE_STORAGE_CACHE
        .lock()
        .unwrap()
        .iter()
        .filter_map(|elem| elem.strip_prefix(path))
        .map(|striped| striped.to_owned())
        .collect()
}

fn find_folders(list: Vec<String>) -> Vec<String> {
    let mut result = HashSet::<String>::new();
    list.into_iter().for_each(|path| {
        if path.contains('/') {
            let elems = path.split('/').collect::<Vec<_>>();

            if !elems.is_empty() {
                result.insert(String::from(elems[0]));
            }
        }
    });

    result.into_iter().collect()
}

fn find_files(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        .filter(|path| !path.contains('/'))
        .map(|elem| elem.to_owned())
        .collect()
}

impl Crud for AzureStorageClient {
    async fn list_folders(&self, path: String) -> Result<Vec<String>> {
        check_or_provision_cache(&self.container).await?;
        Ok(find_folders(files_under_path(&path)))
    }

    async fn list_files(&self, path: String) -> Result<Vec<String>> {
        check_or_provision_cache(&self.container).await?;
        Ok(find_files(files_under_path(&path)))
    }

    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()> {
        let resp = self
            .container
            .blob_client(path)
            .put_block_blob(content.to_vec())
            .await?;
        println!("[DEBUG] put_obj got resp: {:?}", resp);
        Ok(())
    }

    async fn put_obj_stream(&self, dump_name: &str, s3_path: String) -> Result<()> {
        let mut buf_reader = std::io::BufReader::new(std::fs::File::open(dump_name)?);
        let mut content = Vec::new();
        buf_reader.read_to_end(&mut content)?;
        let resp = self
            .container
            .blob_client(s3_path)
            .put_block_blob(content)
            .await?;
        println!("[DEBUG] put_obj_stream got resp: {:?}", resp);
        Ok(())
    }

    async fn get_obj(&self, path: String) -> Result<String> {
        let content = self.container.blob_client(path).get_content().await?;
        Ok(String::from_utf8_lossy(&content).to_string())
    }

    async fn del_obj(&self, path: String) -> Result<()> {
        if !path.ends_with('/') {
            let resp = self.container.blob_client(path).delete().await?;
            println!("[DEBUG] del_obj got resp: {:?}", resp);
        } else {
            let list_to_delete = list(&self.container, path).await?;
            for elem in list_to_delete {
                println!("[DEBUG] delete {}", elem);
                let resp = self.container.blob_client(elem).delete().await?;
                println!("[DEBUG] del_obj got resp: {:?}", resp);
            }
        }

        Ok(())
    }
}

struct AzureStorageClient {
    container: ContainerClient,
}

impl AzureStorageClient {
    fn new(account: String, access_key: String, container: String) -> Self {
        let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
        AzureStorageClient {
            container: ClientBuilder::new(account, storage_credentials)
                .container_client(&container),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not set");
    let container_client = AzureStorageClient::new(account, access_key, container);

    let new_blob = String::from("bar/baz/newfile");
    /*
    let content = String::from("RTFM please");
    container_client
        .put_obj(new_blob.clone(), content.as_bytes())
        .await?;
    */

    let upload_file_path = String::from("Cargo.toml");
    container_client
        .put_obj_stream(&upload_file_path, new_blob.clone())
        .await?;

    //println!("Result: {:?}", container_client.list(String::default()).await?);
    //let result = container_client.list_folders(String::from("xva/")).await?;
    let path = String::from("bar/");
    let folders = container_client.list_folders(path.clone()).await?;
    println!("Folders: {:?}({})", folders, folders.len());

    let files = container_client.list_files(path.clone()).await?;
    println!("Files: {:?}({})", files, files.len());

    println!(
        "Read content: {}",
        container_client.get_obj(new_blob.clone()).await?
    );

    //container_client.del_obj("bar/rust/".to_string()).await?;
    Ok(())
}
