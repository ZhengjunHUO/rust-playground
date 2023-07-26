use anyhow::{bail, Result};
use s3::creds::Credentials;
use s3::region::Region;
use s3::Bucket;
use std::env;

async fn prepare_client() -> Result<Bucket> {
    let bucket_name = std::env::args().nth(1).expect("No bucket name given");
    let access_key = env::var("GCS_ACCESS_KEY")?;
    let secret_key = env::var("GCS_SECRET_KEY")?;

    // Prepare existing bucket
    Ok(Bucket::new(
        &bucket_name,
        Region::Custom {
            region: "eu".to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
    )?
    .with_path_style())
}

async fn crud(bucket: Bucket) -> Result<()> {
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

async fn create_objs(bucket: Bucket) -> Result<()> {
    let prefix = "test";
    let content = b"Rust rocks!";

    for x in 0..100 {
        let resp = bucket
            .put_object(format!("{}{}", prefix, x), content)
            .await?;
        assert_eq!(resp.status_code(), 200);
        println!("[DEBUG] Object {} uploaded to the bucket.", x);
    }

    Ok(())
}

async fn list_all_objs(bucket: Bucket, path: String) -> Result<()> {
    let results = bucket.list(path, Some("/".to_string())).await?;

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

    Ok(())
}

async fn get_obj(bucket: &Bucket, path: String) -> Result<String> {
    match bucket.get_object(path).await {
        Ok(resp) => {
            let rslt = resp.to_string()?;
            println!(
                "[DEBUG] Object retrieved [{}]: {}",
                resp.status_code(),
                rslt
            );
            Ok(rslt)
        }
        Err(e) => {
            bail!("Got error from get_object: {}", e);
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
            bail!("Got error from put_object: {}", e);
        }
    }
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
    let bucket = prepare_client().await?;
    //create_objs(bucket).await
    //crud(bucket).await
    //list_all_objs(bucket, "mtms-util/".to_string()).await
    //list_all_objs(bucket, String::default()).await

    let path_to_file = "shard_rafal_logging/latest";
    match get_obj(&bucket, String::from(path_to_file)).await {
        Ok(content) => {
            println!("Read content from {}: {}", path_to_file, content);
        }
        Err(e) => println!("{} doesn't exist: {}", path_to_file, e),
    }

    let content = b"202309012345";
    put_obj(&bucket, String::from(path_to_file), content).await?;
    get_obj(&bucket, String::from(path_to_file)).await?;

    Ok(())
}
