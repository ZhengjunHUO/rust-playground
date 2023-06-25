use anyhow::{bail, Result};
use s3::creds::Credentials;
use s3::region::Region;
use s3::{Bucket, BucketConfiguration};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let bucket_name = std::env::args().nth(1).expect("No bucket name given");
    let access_key = env::var("GCS_ACCESS_KEY")?;
    let secret_key = env::var("GCS_SECRET_KEY")?;

    // Prepare existing bucket
    let bucket = Bucket::new(
        &bucket_name,
        Region::Custom {
            region: "eu".to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
    )?
    .with_path_style();

    /*
    // Create a new bucket
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
