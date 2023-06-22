use anyhow::Result;
use s3::creds::Credentials;
use s3::region::Region;
use s3::Bucket;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let access_key = env::var("GCS_ACCESS_KEY")?;
    let secret_key = env::var("GCS_SECRET_KEY")?;

    let bucket = Bucket::new(
        "test-ckh-backup",
        Region::Custom {
            region: "eu".to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
    )?
    .with_path_style();

    let path = "test.txt";
    let content = b"Rust rocks!";

    let resp = bucket.put_object(path, content).await?;
    assert_eq!(resp.status_code(), 200);
    println!("[DEBUG] Object uploaded to the bucket.");

    let resp = bucket.get_object(path).await?;
    assert_eq!(resp.status_code(), 200);
    assert_eq!(content, resp.as_slice());
    println!("[DEBUG] Object retrieved.");

    let resp = bucket.delete_object(path).await?;
    assert_eq!(resp.status_code(), 204);
    println!("[DEBUG] Object deleted.");
    Ok(())
}
