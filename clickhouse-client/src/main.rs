use anyhow::Result;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::time::UNIX_EPOCH;
use tokio::runtime::Runtime;

#[derive(Row, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct MyRow<'a> {
    level: &'a str,
    className: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Row)]
struct Entree {
    timestamp: u64,
    message: String,
    level: Level,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum Level {
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

fn now() -> u64 {
    UNIX_EPOCH
        .elapsed()
        .expect("Error retrieving system time")
        .as_nanos() as u64
}

async fn select(client: Client) -> Result<()> {
    // select query
    let mut cursor = client
        .query("SELECT ?fields FROM rafal_logging")
        .fetch::<MyRow<'_>>()?;

    while let Some(row) = cursor.next().await? {
        println!("Got level: {}; classname: {}", row.level, row.className);
    }
    Ok(())
}

async fn crud(client: Client) -> Result<()> {
    // CRUD
    println!("[DEBUG] Drop table if exsit ...");
    client
        .query("DROP TABLE IF EXISTS daily_log")
        .execute()
        .await?;

    println!("[DEBUG] Create table ...");
    client
        .query(
            "
                CREATE TABLE IF NOT EXISTS daily_log (
                    timestamp       DateTime64(9),
                    message         String,
                    level           Enum8(
                                        'Debug' = 1,
                                        'Info' = 2,
                                        'Warn' = 3,
                                        'Error' = 4
                                    )
                )
                ENGINE = Memory
            ",
        )
        .execute()
        .await?;

    println!("[DEBUG] Insert into table ...");
    let mut insert = client.insert("daily_log")?;
    insert
        .write(&Entree {
            timestamp: now(),
            message: "Hello from rust".to_string(),
            level: Level::Info,
        })
        .await?;
    insert.end().await?;

    println!("[DEBUG] Check table after insert ...");
    let entrees = client
        .query("SELECT ?fields FROM daily_log")
        .fetch_all::<Entree>()
        .await?;
    println!("Got: {:?}", entrees);

    Ok(())
}

// path example test-ckh-backup/mtms-20230622/
async fn backup(client: Client, path: &str) -> Result<()> {
    let bucket_url = "https://storage.googleapis.com/";
    let access_key = std::env::var("GCS_ACCESS_KEY")?;
    let secret_key = std::env::var("GCS_SECRET_KEY")?;
    let backup_query = format!(
        "BACKUP TABLE shard_mtms TO S3('{}{}', '{}', '{}')",
        bucket_url, path, access_key, secret_key
    );

    println!("[DEBUG] Backup table ...");
    client.query(&backup_query).execute().await?;

    Ok(())
}

async fn backup_incr(client: Client, base_path: &str, incr_path: &str) -> Result<()> {
    let bucket_url = "https://storage.googleapis.com/";
    let access_key = std::env::var("GCS_ACCESS_KEY")?;
    let secret_key = std::env::var("GCS_SECRET_KEY")?;
    let backup_query = format!(
        "BACKUP TABLE shard_mtms TO S3('{}{}', '{}', '{}') SETTINGS base_backup = S3('{}{}', '{}', '{}')",
        bucket_url, incr_path, access_key, secret_key,
        bucket_url, base_path, access_key, secret_key
    );

    println!("[DEBUG] Backup table ...");
    client.query(&backup_query).execute().await?;

    Ok(())
}

async fn restore(client: Client, path: &str) -> Result<()> {
    // restoration
    let bucket_url = "https://storage.googleapis.com/";
    let access_key = std::env::var("GCS_ACCESS_KEY")?;
    let secret_key = std::env::var("GCS_SECRET_KEY")?;
    let restore_query = format!(
        "RESTORE TABLE shard_mtms AS shard_mtms FROM S3('{}{}', '{}', '{}')",
        bucket_url, path, access_key, secret_key
    );

    println!("[DEBUG] Restore table ...");
    client.query(&restore_query).execute().await?;

    Ok(())
}

fn main() -> Result<()> {
    let client = Client::default().with_url("http://ckh.huo.io:80");
    //        .with_user("rafal")
    //        .with_password("thisIsDevPassword")
    //        .with_database("default");
    let rt = Runtime::new().unwrap();

    //let path = "test-ckh-backup/mtms-20230622/";
    //rt.block_on(async { backup(client, path).await })
    let path_incr = "test-ckh-backup/mtms-20230623/";
    //rt.block_on(async { backup_incr(client, path, path_incr).await })
    rt.block_on(async { restore(client, path_incr).await })
}
