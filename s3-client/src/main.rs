#![allow(dead_code)]

use anyhow::{bail, Result};
use async_recursion::async_recursion;
use s3::creds::Credentials;
use s3::region::Region;
use s3::Bucket;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

const CHUNK_SIZE: usize = 100 * 1024 * 1024;
const SINGLE_PUT_LIMIT: usize = 5 * 1000 * 1000 * 1000;

struct Config {
    bucket_name: String,
    region: String,
    endpoint: Option<String>,
}

async fn prepare_client(config: Config) -> Result<Bucket> {
    let access_key = env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY not set");
    let secret_key = env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY not set");

    let s3_endpoint;
    if config.endpoint.is_none() {
        let inferred_region = Region::from_str(&config.region).unwrap();
        s3_endpoint = inferred_region.endpoint();
    } else {
        s3_endpoint = config.endpoint.clone().unwrap();
    }

    // Prepare existing bucket
    Ok(Bucket::new(
        &config.bucket_name,
        //Region::EuWest3,
        //Region::from_str("eu-west-3").unwrap(),
        Region::Custom {
            region: config.region.clone(),
            endpoint: s3_endpoint,
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
    )?
    .with_path_style())
}

async fn crud(bucket: &Bucket) -> Result<()> {
    let path = "test.txt";
    let content = b"Rust rocks!";

    // Create
    let resp = bucket.put_object(path, content).await?;
    assert_eq!(resp.status_code(), 200);
    println!("[DEBUG] Object uploaded to the bucket.");

    // Read
    let resp = bucket.get_object(path).await?;
    assert_eq!(resp.status_code(), 200);
    assert_eq!(content, resp.as_slice());
    println!("[DEBUG] Object retrieved.");

    // List all objects in bucket
    let results = bucket
        // list the content of a sub bucket
        //.list("mtms-util/".to_string(), Some("/".to_string()))
        .list(String::default(), Some("/".to_string()))
        .await?;

    for res in results {
        println!("[DEBUG] In bucket {}", res.name);
        match res.common_prefixes {
            Some(items) => {
                for item in items {
                    println!("  - {}", item.prefix);
                }
            }
            None => (),
        }

        for ct in res.contents {
            println!("  - {}", ct.key);
        }
    }

    // Delete
    let resp = bucket.delete_object(path).await?;
    assert_eq!(resp.status_code(), 204);
    println!("[DEBUG] Object deleted.");
    Ok(())
}

async fn create_objs(bucket: &Bucket) -> Result<()> {
    let prefix = "test";
    let content = b"Rust rocks!";

    for x in 0..10 {
        let resp = bucket
            .put_object(format!("{}{}", prefix, x), content)
            .await?;
        assert_eq!(resp.status_code(), 200);
        println!("[DEBUG] Object {} uploaded to the bucket.", x);
    }

    Ok(())
}

async fn list_all_objs(bucket: &Bucket, path: String) -> Result<()> {
    let results = bucket.list(path, Some("/".to_string())).await?;

    for res in results {
        println!("[DEBUG] In bucket {}", res.name);
        match res.common_prefixes {
            Some(items) => {
                for item in items {
                    match get_obj(bucket, format!("{}latest", item.prefix)).await {
                        Ok(content) => println!("  - {}\n    [{}]", item.prefix, content,),
                        Err(_) => (),
                    }
                }
            }
            None => (),
        }

        for ct in res.contents {
            println!("  - {}", ct.key);
        }
    }

    Ok(())
}

#[async_recursion]
async fn list_obj_recursive(
    bucket: &Bucket,
    path: String,
    level: u8,
    indent: String,
) -> Result<()> {
    if level == 0 {
        return Ok(());
    }

    let results = bucket.list(path.clone(), Some("/".to_string())).await?;

    for res in results {
        //println!("{}  [DEBUG] Dir under: {}", indent, path);
        match res.common_prefixes {
            Some(items) => {
                for item in items {
                    println!("{}  -> {}", indent, item.prefix);
                    let _ =
                        list_obj_recursive(bucket, item.prefix, level - 1, format!("{}  ", indent))
                            .await?;
                }
            }
            None => (),
        }

        //println!("{}  [DEBUG] Doc under: {}", indent, path);
        for ct in res.contents {
            if ct.key.ends_with("latest") {
                println!(
                    "{}  * {} [{}]",
                    indent,
                    ct.key,
                    get_obj(bucket, format!("{}", ct.key)).await?
                );
            }
        }
    }

    Ok(())
}

#[async_recursion]
async fn del_obj_recursive(bucket: &Bucket, path: String) -> Result<()> {
    let results = bucket.list(path.clone(), Some("/".to_string())).await?;

    for res in results {
        println!("[DEBUG] Dir under: {}", path);
        match res.common_prefixes {
            Some(items) => {
                for item in items {
                    println!("  -> {}", item.prefix);
                    let _ = del_obj_recursive(bucket, item.prefix).await?;
                }
            }
            None => (),
        }

        println!("[DEBUG] File under: {}", path);
        for ct in res.contents {
            println!("  - {}", ct.key);
            let _ = del_obj(bucket, ct.key).await;
        }
    }

    Ok(())
}

async fn get_obj(bucket: &Bucket, path: String) -> Result<String> {
    match bucket.get_object(path).await {
        Ok(resp) => {
            let rslt = resp.to_string()?;
            //println!("Object retrieved [{}]: {}", resp.status_code(), rslt);
            Ok(rslt)
        }
        Err(e) => {
            bail!("Got error from get_obj: {}", e);
        }
    }
}

async fn put_obj(bucket: &Bucket, path: String, content: &[u8]) -> Result<()> {
    match bucket.put_object(path, content).await {
        Ok(resp) => {
            println!(
                "[DEBUG] Get response [{}]: {}",
                resp.status_code(),
                resp.to_string()?
            );
            Ok(())
        }
        Err(e) => {
            bail!("Got error from put_obj: {}", e);
        }
    }
}

async fn put_obj_stream(bucket: &Bucket, dump_name: &str, s3_path: String) -> Result<()> {
    let mut file = tokio::fs::File::open(dump_name).await?;
    match bucket.put_object_stream(&mut file, s3_path).await {
        Ok(resp) => {
            println!("Object updated [{}] ", resp.status_code());
            Ok(())
        }
        Err(e) => {
            bail!("Got error from put_obj_stream: {:?}", e);
        }
    }
}

async fn put_obj_multipart(bucket: &Bucket, dump_name: &str, s3_path: &str) -> Result<()> {
    let file = std::fs::File::open(dump_name)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    println!("[{}] File size: {}", dump_name, file_size);
    if file_size as usize > SINGLE_PUT_LIMIT {
        println!("Found a big file to upload");
    }
    let mut reader = std::io::BufReader::new(file);

    let init_resp = bucket
        .initiate_multipart_upload(s3_path, "application/octet-stream")
        .await?;
    println!("Upload ID: {:?}", init_resp.upload_id);

    let mut chunk_num = 1;
    let mut parts = Vec::new();
    let mut buf = vec![0; CHUNK_SIZE];

    while let Ok(num_bytes) = reader.read(&mut buf) {
        println!("Read {} bytes", num_bytes);
        if num_bytes == 0 {
            break;
        }

        let chunk = &buf[..num_bytes];
        let part = bucket
            .put_multipart_chunk(
                chunk.to_vec(),
                s3_path,
                chunk_num,
                &init_resp.upload_id,
                "application/octet-stream",
            )
            .await?;
        println!("Chunk {} uploaded (part: [{:?}])", chunk_num, part);
        parts.push(part);

        chunk_num += 1;
    }

    bucket
        .complete_multipart_upload(s3_path, &init_resp.upload_id, parts)
        .await?;
    println!("Done");
    Ok(())
}

async fn get_obj_multipart(bucket: &Bucket, out_name: &str, s3_path: &str) -> Result<()> {
    let (head, _) = bucket.head_object(s3_path).await?;
    let obj_size = head.content_length.unwrap() as usize;
    println!("[{}] File size: {}", s3_path, obj_size);

    let mut file = OpenOptions::new().write(true).create(true).open(out_name)?;

    let mut start = 0;
    while start < obj_size {
        let end = (start + CHUNK_SIZE - 1).min(obj_size - 1);
        println!("Dealing with {}-{}", start, end);

        match bucket
            .get_object_range(s3_path, start as u64, Some(end as u64))
            .await
        {
            Ok(data) => {
                file.seek(std::io::SeekFrom::Start(start as u64))?;
                file.write_all(data.as_slice())?;
                println!("Downloaded {}-{}", start, end);
            }
            Err(e) => {
                println!("Error occurred downloading {}-{}: {}", start, end, e);
                sleep(Duration::from_secs(3));
                continue;
            }
        }

        start += CHUNK_SIZE;
    }

    println!("Done");
    Ok(())
}

async fn del_obj(bucket: &Bucket, path: String) -> Result<()> {
    match bucket.delete_object(path).await {
        Ok(resp) => {
            println!(
                "[DEBUG] Get response [{}]: {}",
                resp.status_code(),
                resp.to_string()?
            );
            Ok(())
        }
        Err(e) => {
            println!("[DEBUG] Got error from del_obj: {}", e);
            bail!("{}", e);
        }
    }
}

fn write_to_file(file_name: &str, content: &str) -> Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    /*
    // Create a new bucket
    use s3::BucketConfiguration;
    use anyhow::Result;

    let create_resp = Bucket::create(
        &bucket_name,
        Region::Custom {
            region: "eu".to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
        BucketConfiguration::default()
    ).await?;

    if !create_resp.success() {
        bail!("Failed to create bucket: {}", create_resp.response_text);
    }

    let bucket = create_resp.bucket;
    */
    let bucket_name = std::env::args().nth(1).expect("No bucket name given");
    /*
        let path = std::env::args()
            .nth(2)
            .expect("The object's path is required");
    */

    // AWS
    let config = Config {
        bucket_name: bucket_name,
        region: "eu-west-3".to_owned(),
        endpoint: None,
    };

    // Minio
    /*
    let config = Config {
        bucket_name: bucket_name,
        region: "eu".to_owned(),
        endpoint: Some("http://172.19.0.11:9000".to_owned()),
    };
    */

    // GCP
    /*
        let config = Config {
            bucket_name: bucket_name,
            region: "eu".to_owned(),
            endpoint: Some("https://storage.googleapis.com".to_owned()),
        };
    */

    let bucket = prepare_client(config).await?;

    //create_objs(&bucket).await.unwrap();
    //crud(&bucket).await
    //let _ = list_all_objs(&bucket, path).await;
    //list_all_objs(&bucket, "mtms-util/".to_string()).await
    //list_all_objs(&bucket, String::from("data/")).await;

    /*
        match get_obj(&bucket, String::from("test/crackstation-human-only.txt")).await {
            Ok(content) => {
                println!("Read content ok, write to file ...");
                write_to_file("test.txt", &content)?;
            }
            Err(e) => println!("File doesn't exist: {}", e),
        }
    */
    //put_obj(&bucket, String::from("foo/bar"), b"Rust rocks!").await?;
    //let results = bucket.list(String::from("/"), Some("/".to_string())).await?;
    /*
        put_obj_multipart(
            &bucket,
            "crackstation-human-only.txt",
            "test/crackstation-human-only.txt",
        )
        .await?;
    */
    get_obj_multipart(&bucket, "test.txt", "test/crackstation-human-only.txt").await?;

    //let results = bucket.list(String::from(""), Some("/".to_string())).await?;
    //println!("{:?}", results);
    /*
        if get_obj(&bucket, String::from("test_dir/")).await.is_err() {
            println!("test_dir does not exist, creating...");
            put_obj(&bucket, String::from("test_dir/"), b"").await?;
        }

        println!("In test_dir");
        list_all_objs(&bucket, String::from("test_dir/")).await;
        println!("Put obj");
        put_obj(&bucket, String::from("test_dir/test"), b"Rust rocks!").await?;
        println!("In test_dir");
        list_all_objs(&bucket, String::from("test_dir/")).await;
    */
    /*
    let path_to_file = "psql-dump/202311071020.sql";
    let content = get_obj(&bucket, String::from(path_to_file)).await?;
    write_to_file("test.sql", &content)?;
    println!("Done");

    let path_to_file = "empty/shard_rafal_logging_latest";
    match del_obj(&bucket, String::from(path_to_file)).await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    */

    //del_obj_recursive(&bucket, String::from("shard_rafal_logging_latest")).await;
    //del_obj_recursive(&bucket, path).await?;
    //list_obj_recursive(&bucket, path, 3, String::default()).await?;

    Ok(())
}
