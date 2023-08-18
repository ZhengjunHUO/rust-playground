use anyhow::Result;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let access_key = env::var("GCS_ACCESS_KEY")?;
    let secret_key = env::var("GCS_SECRET_KEY")?;

    let bucket = Bucket::new(
        "test-ckh-backup-demo",
        Region::Custom {
            region: "eu".to_owned(),
            endpoint: "https://storage.googleapis.com".to_owned(),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
    )?
    .with_path_style();

    let s3_path = "shard_mtms/latest";

    let task_num = 200;
    let mut pool = Vec::with_capacity(task_num);
    for i in 0..task_num {
        let mut b = bucket.clone();
        // Workaround, to use independant cred
        b.set_credentials(
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
        );
        pool.push(tokio::spawn(async move {
            let response_data = b.get_object(s3_path).await;
            match response_data {
                //Ok(_) => println!("[{}] Ok", i),
                Ok(_) => {}
                Err(e) => eprintln!("[{}] {}", e, i),
            }
        }));
    }

    let mut async_rslt = Vec::with_capacity(task_num);
    for t in pool {
        async_rslt.push(t.await.unwrap());
    }

    Ok(())
}
