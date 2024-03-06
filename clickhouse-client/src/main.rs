#![allow(dead_code)]

use anyhow::Result;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
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

#[derive(Row, Deserialize)]
struct TableName {
    name: String,
}

#[derive(Row, Deserialize)]
struct Count {
    num: usize,
}

#[derive(Row, Deserialize)]
struct ShowCreate {
    statement: String,
}

#[derive(Row, Deserialize)]
struct SystemTable {
    engine: String,
}

#[derive(Row, Deserialize)]
struct TableSize {
    db: String,
    table: String,
    rows: u64,
    size: u64,
}

#[derive(Row, Deserialize)]
struct FreeSpace {
    free_space: u64,
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

async fn show_tables(client: &Client) -> Result<()> {
    /*
        let mut cursor = client.query("show tables").fetch::<TableName<'_>>()?;

        while let Some(row) = cursor.next().await? {
            println!("{}", row.name);
        }
    */
    let rows: Vec<String> = client
        .query("show tables")
        .fetch_all::<TableName>()
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect();
    for row in rows {
        println!("{}", row);
    }

    Ok(())
}

async fn is_empty_async(client: &Client, table_name: &str) -> bool {
    let query = format!("select count() from {}", table_name);
    if let Ok(rows) = client.query(&query).fetch_all::<Count>().await {
        let rslt: Vec<usize> = rows.into_iter().map(|r| r.num).collect();
        if !rslt.is_empty() {
            if rslt[0] == 0 {
                return true;
            }
            println!(
                "[DEBUG] Table {} contains {} line(s) !",
                table_name, rslt[0]
            )
        }
    }

    false
}

async fn show_create_table(client: &Client, database_name: &str, table_name: &str) {
    let query = format!("show create table {}.{}", database_name, table_name);
    if let Ok(rows) = client.query(&query).fetch_all::<ShowCreate>().await {
        let rslt: Vec<String> = rows.into_iter().map(|r| r.statement).collect();
        if !rslt.is_empty() {
            println!(
                "[DEBUG] Table {}.{}'s creation detail:\n {}",
                database_name, table_name, rslt[0]
            )
        }
    }
}

async fn get_free_space(client: &Client) -> u64 {
    match client
        .query("SELECT free_space FROM system.disks")
        .fetch_all::<FreeSpace>()
        .await
    {
        Ok(rows) => {
            let rslt: Vec<u64> = rows.into_iter().map(|r| r.free_space).collect();
            println!(
                "[DEBUG] Free space: {} ({})",
                size_to_human_readable(rslt[0]),
                rslt[0]
            );
            rslt[0]
        }
        Err(e) => {
            panic!("Error getting disk free space: {}", e);
        }
    }
}

async fn calc_table_size(client: &Client) -> HashMap<(String, String), (u64, u64)> {
    match client
        .query("SELECT database, table, sum(rows) as rows, sum(bytes_on_disk) as size FROM system.parts WHERE active GROUP BY database, table")
        .fetch_all::<TableSize>()
        .await
    {
        Ok(rows) => {
            let rslt: HashMap<(String, String), (u64, u64)> = rows.into_iter().map(|r| ((r.db, r.table), (r.rows, r.size))).collect();
            println!("[DEBUG] Tables' rows & size (in bytes): {:?}", rslt);
            rslt
        }
        Err(e) => {
            panic!("Error getting tables' size: {}", e);
        }
    }
}

async fn is_table_sharded(client: &Client, database_name: &str, table_name: &str) -> bool {
    let query = format!(
        "select engine from system.tables where database='{}' and name='{}'",
        database_name, table_name
    );
    match client.query(&query).fetch_all::<SystemTable>().await {
        Ok(rows) => {
            let rslt: Vec<String> = rows.into_iter().map(|r| r.engine).collect();
            if !rslt.is_empty() {
                println!(
                    "[DEBUG] Table {}.{}'s engine:\n{}\nIs sharded ? {}",
                    database_name,
                    table_name,
                    rslt[0],
                    rslt[0].starts_with("Replicated")
                );

                if rslt[0].starts_with("Replicated") {
                    return true;
                }
            }
        }
        Err(e) => {
            println!(
                "[DEBUG] Error getting table {}.{}'s engine:\n{}",
                database_name, table_name, e
            );
        }
    }

    false
}

fn is_empty(client: &Client, table_name: &str) -> bool {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let query = format!("select count() from {}", table_name);
        let mut result = false;
        if let Ok(rows) = client.query(&query).fetch_all::<Count>().await {
            let rslt: Vec<usize> = rows.into_iter().map(|r| r.num).collect();
            if rslt.len() > 0 {
                if rslt[0] == 0 {
                    result = true;
                }
                println!(
                    "[DEBUG] Table {} contains {} line(s) !",
                    table_name, rslt[0]
                )
            }
        }
        result
    })
}

const UNITS: [&str; 5] = ["byte(s)", "KB", "MB", "GB", "TB"];

fn size_to_human_readable(size: u64) -> String {
    if size < 1024 {
        return format!("{} byte(s)", size);
    }

    let mut idx = 0;
    let mut rslt = size as f64;
    while rslt >= 1024.0 {
        rslt /= 1024.0;
        idx += 1;
    }

    format!("{:.2} {}", rslt, UNITS[idx])
}

fn main() -> Result<()> {
    /* #1 HTTP only client
    let client = Client::default().with_url("https://ckh-0-0.huo.io:443");
            .with_user("foo")
            .with_password("bar")
            .with_database("baz");
    */

    /* #2 HTTPS client
    // (I) Allow only https
    //let mut https_conn = hyper_tls::HttpsConnector::new();
    //https_conn.https_only(true);

    // (II) http compatible client
    let https_conn = hyper_tls::HttpsConnector::new();

    let https_client = hyper::Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        //.http2_only(true)
        //.build_http();
        .build::<_, hyper::Body>(https_conn);
    //let client = Client::with_http_client(https_client).with_url("https://ckh-0-0.huo.io:443");
    let client = Client::with_http_client(https_client).with_url("http://ckh-0-0.huo.io:80");
    */

    // #3 HTTPS insecure client
    // Read self-signed cert and create Certificate
    //const HOME_MADE_CERT: &[u8] = std::include_bytes!("../certificate.crt");
    //let cert = tokio_native_tls::native_tls::Certificate::from_pem(HOME_MADE_CERT).unwrap();

    // (3.1) Prepare tls connector
    let tls_connector = tokio_native_tls::TlsConnector::from(
        tokio_native_tls::native_tls::TlsConnector::builder()
            //.add_root_certificate(cert)
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );

    // (3.2) Prepare http connector
    let mut http_conn = hyper::client::HttpConnector::new();
    // crucial setting
    http_conn.enforce_http(false);

    // (3.3) Prepare https connector
    let https_conn =
        hyper_tls::HttpsConnector::<hyper::client::HttpConnector>::from((http_conn, tls_connector));

    // (3.4) Build hyper::client::Client from https connector
    let https_client = hyper::Client::builder().build::<_, hyper::Body>(https_conn);

    // (3.5) Use hyper::client::Client to build a ckh client
    let client = Client::with_http_client(https_client)
        .with_url("https://ckh-0-0.huo.io:443")
        .with_database("default");

    let rt = Runtime::new().unwrap();
    /* #1 Backup test
    //let path = "test-ckh-backup/mtms-20230622/";
    //rt.block_on(async { backup(client, path).await })
    let path_incr = "test-ckh-backup/mtms-20230623/";
    //rt.block_on(async { backup_incr(client, path, path_incr).await })
    rt.block_on(async { restore(client, path_incr).await })
    */

    // #2 Show tables test
    //rt.block_on(async { show_tables(client).await })

    /* #3 Test is table empty
    rt.block_on(async {
        let table_name = "rafal_logging";
        let rslt = is_empty_async(&client, &table_name).await;
        println!("Table {} is empty ? {}", table_name, rslt);
    });
    */

    /* #4 List non empty table(s)
    let tables = vec!["rafal_logging".to_string(), "rafal_queries".to_string()]
        .into_iter()
        .filter(|t| !is_empty(&client, &t))
        .collect::<Vec<String>>();
    for table in tables {
        println!("Non empty table: {:?}", table);
    }
    */

    /* #5 Test is table sharded
    rt.block_on(async {
        //let _ = show_tables(&client).await;
        //show_create_table(&client, "default", "shard_label_dist_endpoint_query_inspection").await
        println!(
            "{}",
            is_table_sharded(&client, "default", "shard_TechnicalBranch").await
        );
    });
    */

    rt.block_on(async {
        let dict = calc_table_size(&client).await;
        println!(
            "system.trace_log rows & size: {:?}",
            dict.get(&("system".to_string(), "trace_log".to_string()))
                .unwrap_or(&(0, 0))
        );
        println!(
            "system.non_exist rows & size: {:?}",
            dict.get(&("system".to_string(), "non_exist".to_string()))
                .unwrap_or(&(0, 0))
        );
        let sum = dict.iter().fold(0, |acc, (_, s)| acc + s.1);
        println!(
            "[DEBUG] Sum: {} ({} bytes)",
            size_to_human_readable(sum),
            sum
        );
        if sum < get_free_space(&client).await {
            println!("OK");
        } else {
            println!("KO");
        }
    });

    Ok(())
}
