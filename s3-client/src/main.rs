#![allow(dead_code)]

use anyhow::{bail, Result};
use async_recursion::async_recursion;
use s3::creds::Credentials;
use s3::region::Region;
use s3::Bucket;
use std::env;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

struct Config {
    bucket_name: String,
    region: String,
    endpoint: Option<String>,
}

async fn prepare_client(config: Config) -> Result<Bucket> {
    let access_key = env::var("S3_ACCESS_KEY")?;
    let secret_key = env::var("S3_SECRET_KEY")?;

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
    /*
    let config = Config {
        bucket_name: bucket_name,
        region: "eu-west-3".to_owned(),
        endpoint: None,
    };
    */

    // Minio
    /*
    let config = Config {
        bucket_name: bucket_name,
        region: "eu".to_owned(),
        endpoint: Some("http://127.0.0.1:9000".to_owned()),
    };
    */

    // GCP
    let config = Config {
        bucket_name: bucket_name,
        region: "eu".to_owned(),
        endpoint: Some("https://storage.googleapis.com".to_owned()),
    };

    let bucket = prepare_client(config).await?;

    //create_objs(&bucket).await.unwrap();
    //crud(&bucket).await
    //let _ = list_all_objs(&bucket, path).await;
    //list_all_objs(&bucket, "mtms-util/".to_string()).await
    //list_all_objs(&bucket, String::from("data/")).await;

    /*
    match get_obj(&bucket, path.clone()).await {
        Ok(content) => {
            println!("Read content from {}: {}", path, content);
        }
        Err(e) => println!("{} doesn't exist: {}", path, e),
    }
    */

    put_obj(&bucket, String::from("test_dir/test"), b"Rust rocks!").await?;

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
