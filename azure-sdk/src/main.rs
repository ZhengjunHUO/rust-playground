use anyhow::{bail, Result};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
//use lazy_static::lazy_static;
use log::{debug, info};
use std::io::Read;
//use tokio::sync::Mutex;

const CHUNK_SIZE: usize = 100 * 1024 * 1024;

/*
lazy_static! {
    pub(crate) static ref REMOTE_STORAGE_CACHE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

async fn check_or_provision_cache(client: &ContainerClient, id: &str) -> Result<()> {
    let mut guard = REMOTE_STORAGE_CACHE.lock().await;
    if guard.is_empty() {
        info!("[Critical][{}] Retrieve info from remote ...", id);
        *guard = list(client, String::new()).await?;
        info!("[Critical][{}] Retrieve info done", id);
    }

    Ok(())
}

async fn files_under_path(path: &str) -> Vec<String> {
    REMOTE_STORAGE_CACHE
        .lock()
        .await
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
        info!("[{}] Do check_or_provision_cache ...", self.id);
        check_or_provision_cache(&self.container, &self.id).await?;
        info!("[{}] Check_or_provision_cache done", self.id);
        Ok(find_folders(files_under_path(&path).await))
    }

    async fn list_files(&self, path: String) -> Result<Vec<String>> {
        info!("[{}] Do check_or_provision_cache ...", self.id);
        check_or_provision_cache(&self.container, &self.id).await?;
        info!("[{}] Check_or_provision_cache done", self.id);
        Ok(find_files(files_under_path(&path).await))
    }

    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()> {
        let resp = self
            .container
            .blob_client(path)
            .put_block_blob(content.to_vec())
            .await?;
        info!("[DEBUG] put_obj got resp: {:?}", resp);
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
        info!("[DEBUG] put_obj_stream got resp: {:?}", resp);
        Ok(())
    }

    async fn get_obj(&self, path: String) -> Result<String> {
        let content = self.container.blob_client(path).get_content().await?;
        Ok(String::from_utf8_lossy(&content).to_string())
    }

    async fn del_obj(&self, path: String) -> Result<()> {
        if !path.ends_with('/') {
            let resp = self.container.blob_client(path).delete().await?;
            info!("[DEBUG] del_obj got resp: {:?}", resp);
        } else {
            let list_to_delete = list(&self.container, path).await?;
            for elem in list_to_delete {
                info!("[DEBUG] delete {}", elem);
                let resp = self.container.blob_client(elem).delete().await?;
                info!("[DEBUG] del_obj got resp: {:?}", resp);
            }
        }

        Ok(())
    }
}
*/

async fn list(client: &ContainerClient, path: String) -> Result<Vec<String>> {
    let mut rslt = Vec::new();
    let mut stream = client.list_blobs().prefix(path).into_stream();
    while let Some(resp) = stream.next().await {
        info!("[DEBUG] Got a new page");
        match resp {
            Ok(elem) => {
                let blobs: Vec<_> = elem.blobs.blobs().collect();
                blobs.into_iter().for_each(|blob| {
                    rslt.push(blob.name.clone());
                    //info!("[DEBUG] {:?}", blob.name)
                });
            }
            _ => bail!("Sth wrong happened"),
        }
    }
    Ok(rslt)
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
    async fn download_obj_multipart(&self, out_name: &str, path: String) -> Result<()>;
    async fn put_obj_multipart(&self, in_name: &str, path: String) -> Result<()>;
}

impl Crud for AzureStorageClient {
    async fn list_folders(&self, path: String) -> Result<Vec<String>> {
        let list = self
            .container
            .list_blobs()
            .prefix(path.clone())
            .delimiter("/");
        let mut rslt = Vec::new();

        let mut stream = list.into_stream();
        while let Some(resp) = stream.next().await {
            info!("[DEBUG] Got a new page");
            match resp {
                Ok(elem) => {
                    let blobs: Vec<_> = elem.blobs.prefixes().collect();
                    blobs.into_iter().for_each(|blob| {
                        rslt.push(
                            blob.name
                                .strip_prefix(&path)
                                .unwrap()
                                .strip_suffix('/')
                                .unwrap()
                                .to_owned(),
                        );
                        debug!("Got {:?}", blob.name)
                    });
                }
                _ => bail!("Sth wrong happened"),
            }
        }
        Ok(rslt)
    }

    async fn list_files(&self, path: String) -> Result<Vec<String>> {
        let list = self
            .container
            .list_blobs()
            .prefix(path.clone())
            .delimiter("/");
        let mut rslt = Vec::new();

        let mut stream = list.into_stream();
        while let Some(resp) = stream.next().await {
            info!("[DEBUG] Got a new page");
            match resp {
                Ok(elem) => {
                    let blobs: Vec<_> = elem.blobs.blobs().collect();
                    blobs.into_iter().for_each(|blob| {
                        rslt.push(blob.name.strip_prefix(&path).unwrap().to_owned());
                        debug!("Got {:?}", blob.name)
                    });
                }
                _ => bail!("Sth wrong happened"),
            }
        }
        Ok(rslt)
    }

    async fn put_obj(&self, path: String, content: &[u8]) -> Result<()> {
        let resp = self
            .container
            .blob_client(path)
            .put_block_blob(content.to_vec())
            .await?;
        info!("[DEBUG] put_obj got resp: {:?}", resp);
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
        info!("[DEBUG] put_obj_stream got resp: {:?}", resp);
        Ok(())
    }

    async fn get_obj(&self, path: String) -> Result<String> {
        let content = self.container.blob_client(path).get_content().await?;
        Ok(String::from_utf8_lossy(&content).to_string())
    }

    async fn del_obj(&self, path: String) -> Result<()> {
        if !path.ends_with('/') {
            let resp = self.container.blob_client(path).delete().await?;
            info!("[DEBUG] del_obj got resp: {:?}", resp);
        } else {
            let list_to_delete = list(&self.container, path).await?;
            for elem in list_to_delete {
                info!("[DEBUG] delete {}", elem);
                let resp = self.container.blob_client(elem).delete().await?;
                info!("[DEBUG] del_obj got resp: {:?}", resp);
            }
        }

        Ok(())
    }

    async fn download_obj_multipart(&self, out_name: &str, path: String) -> Result<()> {
        Ok(())
    }

    async fn put_obj_multipart(&self, in_name: &str, path: String) -> Result<()> {
        let file = std::fs::File::open(in_name)?;
        let metadata = file.metadata()?;
        let file_size = metadata.len();
        println!("[{}] File size: {}", in_name, file_size);
        let mut reader = std::io::BufReader::new(file);

        let client = self.container.blob_client(&path);
        let mut block_list = BlockList { blocks: Vec::new() };
        let mut chunk_num = 0;
        let mut buf = vec![0; CHUNK_SIZE];

        while let Ok(num_bytes) = reader.read(&mut buf) {
            println!("Read {} bytes", num_bytes);
            if num_bytes == 0 {
                break;
            }

            let block_id = format!("{:08}", chunk_num);
            let block_data = buf[..num_bytes].to_vec();
            let resp = client.put_block(block_id.clone(), block_data).await?;
            block_list
                .blocks
                .push(BlobBlockType::new_uncommitted(block_id.clone()));
            println!("Chunk {} uploaded (resp: [{:?}])", block_id, resp);
            chunk_num += 1;
        }
        client.put_block_list(block_list).await?;

        println!("Done");
        Ok(())
    }
}

#[derive(Clone)]
struct AzureStorageClient {
    id: String,
    container: ContainerClient,
}

impl AzureStorageClient {
    fn new(account: String, access_key: String, container: String, id: String) -> Self {
        let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
        AzureStorageClient {
            id,
            container: ClientBuilder::new(account, storage_credentials)
                .container_client(&container),
        }
    }
}

/*
async fn list_files_native(client: &ContainerClient, path: String) -> Result<Vec<String>> {
    let list = client.list_blobs().prefix(path).delimiter("/");
    let mut rslt = Vec::new();

    let mut stream = list.into_stream();
    while let Some(resp) = stream.next().await {
        info!("[DEBUG] Got a new page");
        match resp {
            Ok(elem) => {
                let blobs: Vec<_> = elem.blobs.blobs().collect();
                blobs.into_iter().for_each(|blob| {
                    rslt.push(blob.name.clone());
                    debug!("Got {:?}", blob.name)
                });
            }
            _ => bail!("Sth wrong happened"),
        }
    }
    Ok(rslt)
}

async fn list_folders_native(client: &ContainerClient, path: String) -> Result<Vec<String>> {
    let list = client.list_blobs().prefix(path).delimiter("/");
    let mut rslt = Vec::new();

    let mut stream = list.into_stream();
    while let Some(resp) = stream.next().await {
        info!("[DEBUG] Got a new page");
        match resp {
            Ok(elem) => {
                let blobs: Vec<_> = elem.blobs.prefixes().collect();
                blobs.into_iter().for_each(|blob| {
                    rslt.push(blob.name.clone());
                    debug!("Got {:?}", blob.name)
                });
            }
            _ => bail!("Sth wrong happened"),
        }
    }
    Ok(rslt)
}
*/

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    //mulit_hosts_cache_access().await?;
    //poc().await?;
    test_multipart().await?;
    Ok(())
}

pub async fn test_multipart() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not set");
    let container_client =
        AzureStorageClient::new(account, access_key, container, String::from("test"));

    container_client
        .put_obj_multipart("crackstation-human-only.txt", "test/test.txt".to_owned())
        .await?;
    Ok(())
}

pub async fn poc() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not set");
    let container_client =
        AzureStorageClient::new(account, access_key, container, String::from("test"));

    let path = String::from("default/shard_StdData/202501211311/shard0/");
    println!("files under {}:", path);
    let list = container_client.list_files(path.clone()).await?;
    println!("  {:?}", list);
    println!("folders under {}:", path);
    let list = container_client.list_folders(path).await?;
    println!("  {:?}", list);

    let path = String::from("default/shard_StdData/202501211311/");
    println!("files under {}:", path);
    let list = container_client.list_files(path.clone()).await?;
    println!("  {:?}", list);
    println!("folders under {}:", path);
    let list = container_client.list_folders(path).await?;
    println!("  {:?}", list);

    println!("files under root");
    let list = container_client.list_files(String::new()).await?;
    println!("  {:?}", list);
    println!("folders under root");
    let list = container_client.list_folders(String::new()).await?;
    println!("  {:?}", list);

    Ok(())
}

pub async fn simple_test() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not set");
    let container_client =
        AzureStorageClient::new(account, access_key, container, String::from("test"));

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

    //info!("Result: {:?}", container_client.list(String::default()).await?);
    //let result = container_client.list_folders(String::from("xva/")).await?;
    let path = String::from("bar/");
    let folders = container_client.list_folders(path.clone()).await?;
    info!("Folders: {:?}({})", folders, folders.len());

    let files = container_client.list_files(path.clone()).await?;
    info!("Files: {:?}({})", files, files.len());

    info!(
        "Read content: {}",
        container_client.get_obj(new_blob.clone()).await?
    );

    //container_client.del_obj("bar/rust/".to_string()).await?;
    Ok(())
}

pub async fn mulit_hosts_cache_access() -> Result<()> {
    let account = std::env::var("AZURE_ACCOUNT_NAME").expect("AZURE_ACCOUNT_NAME not set");
    let access_key = std::env::var("AZURE_ACCESS_KEY").expect("AZURE_ACCESS_KEY not set");
    let container = std::env::var("AZURE_CONTAINER_NAME").expect("AZURE_CONTAINER_NAME not set");

    let client_foo = AzureStorageClient::new(
        account.clone(),
        access_key.clone(),
        container.clone(),
        "foo".to_owned(),
    );
    let client_bar = AzureStorageClient::new(
        account.clone(),
        access_key.clone(),
        container.clone(),
        "bar".to_owned(),
    );
    let client_bar_clone = client_bar.clone();

    let handlers = vec![
        tokio::spawn(async move {
            let path = "default/shard_StdData/202501211311/shard0/".to_owned();
            let list = client_foo.list_files(path.clone()).await;
            println!("[foo] {} files: {:?}", path, list.unwrap_or_default());
        }),
        tokio::spawn(async move {
            let path = String::from("psql-dump/");
            let list = client_bar.list_folders(path.clone()).await;
            println!("[bar] {} folders: {:?}", path, list.unwrap_or_default());
        }),
        tokio::spawn(async move {
            let path = "default/shard_StdData/202501211311/".to_owned();
            let list = client_bar_clone.list_folders(path.clone()).await;
            println!(
                "[bar_clone] {} folders: {:?}",
                path,
                list.unwrap_or_default()
            );
        }),
    ];

    for h in handlers {
        h.await.unwrap();
    }

    info!("Done");
    Ok(())
}
