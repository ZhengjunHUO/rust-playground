#![allow(dead_code)]

use anyhow::{bail, Result};
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::cmp::max;
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
    name: String,
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

#[derive(Row, Deserialize)]
struct ClusterTopo {
    shard_num: u32,
    replica_num: u32,
}

struct Topology {
    topos: Vec<ClusterTopo>,
    shard_num: u32,
    replica_num: u32,
}

impl Topology {
    async fn new(client: &Client, cluster_name: &str) -> Self {
        let query = format!(
            "SELECT shard_num,replica_num FROM system.clusters where cluster='{}'",
            cluster_name
        );
        let topos = Topology::get_cluster_topo(client, &query).await.unwrap();
        let (shard_num, replica_num) = topos.iter().fold((0, 0), |(sn, rn), tp| {
            (max(sn, tp.shard_num), max(rn, tp.replica_num))
        });

        Topology {
            topos,
            shard_num,
            replica_num,
        }
    }

    async fn get_cluster_topo(client: &Client, query: &str) -> Result<Vec<ClusterTopo>> {
        execute_query::<ClusterTopo>(client, query).await
    }

    fn is_all_replicated(&self) -> bool {
        self.shard_num == 1 && self.replica_num > self.shard_num
    }
}

struct TableEngine {
    engine_map: HashMap<String, String>,
    is_all_replic_map: HashMap<String, bool>,
}

impl TableEngine {
    async fn new(client: &Client, database: &str) -> Self {
        let engine_map = Self::build_engine_map(client, database).await.unwrap();
        let is_all_replic_map = Self::build_is_all_replicated_map(client).await;
        TableEngine {
            engine_map,
            is_all_replic_map,
        }
    }

    fn is_sharded(&self, table_name: &str) -> bool {
        if let Some(engine) = self.engine_map.get(table_name) {
            return engine.starts_with("Replicated");
        }

        false
    }

    fn contains_macro(&self, table_name: &str, macro_name: &str) -> bool {
        if let Some(engine) = self.engine_map.get(table_name) {
            return engine.contains(macro_name);
        }

        false
    }

    fn sharded_contains_macro(&self, table_name: &str, macro_name: &str) -> bool {
        self.is_sharded(table_name) && self.contains_macro(table_name, macro_name)
    }

    fn trace_table_topology(&self, table_name: &str, prefix: &str) -> Option<String> {
        if let Some(controller_name) = Self::find_controller(table_name, prefix) {
            return self.get_topology(&controller_name);
        }

        None
    }

    fn get_topology(&self, table_name: &str) -> Option<String> {
        if let Some(engine) = self.engine_map.get(table_name) {
            if engine.find('{').is_some() {
                let (_, text) = engine.split_once('{').unwrap();
                if engine.find('}').is_some() {
                    let (topo_macro, _) = text.split_once('}').unwrap();
                    return Some(topo_macro.to_owned());
                }
            }
        }

        None
    }

    fn find_controller(table_name: &str, prefix: &str) -> Option<String> {
        if table_name.starts_with(prefix) {
            return Some(table_name.strip_prefix(prefix).unwrap().to_owned());
        }

        None
    }

    async fn build_engine_map(client: &Client, database: &str) -> Result<HashMap<String, String>> {
        let query = format!(
            "select name, engine_full from system.tables where database='{}'",
            database
        );
        match client.query(&query).fetch_all::<SystemTable>().await {
            Ok(rows) => Ok(rows
                .into_iter()
                .map(|r| (r.name, r.engine))
                .collect::<HashMap<String, String>>()),
            Err(e) => {
                bail!("Error getting table's engine info: {}", e);
            }
        }
    }

    async fn build_is_all_replicated_map(client: &Client) -> HashMap<String, bool> {
        let mut dict = HashMap::<String, bool>::new();
        let clusters =
            execute_query::<TableName>(client, "SELECT DISTINCT cluster FROM system.clusters")
                .await
                .unwrap();
        for cluster in clusters {
            let is_all_replicated = Topology::new(client, &cluster.name)
                .await
                .is_all_replicated();
            dict.insert(cluster.name, is_all_replicated);
        }

        dict
    }

    async fn get_ckh_macro_value(client: &Client, macro_name: &str) -> Option<String> {
        let query = format!("select getMacro('{}')", macro_name);
        if let Ok(row) = client.query(&query).fetch_one::<TableName>().await {
            return Some(row.name);
        }

        None
    }

    async fn is_all_replicated(&self, client: &Client, table_name: &str, prefix: &str) -> bool {
        if self.is_sharded(table_name) {
            if let Some(topo) = self.trace_table_topology(table_name, prefix) {
                if let Some(topo_value) = Self::get_ckh_macro_value(client, &topo).await {
                    if let Some(&rslt) = self.is_all_replic_map.get(&topo_value) {
                        return rslt;
                    }
                }
            }
        }

        false
    }
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

async fn show_tables(client: &Client) -> Result<Vec<String>> {
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

    Ok(rows)
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

async fn show_create_table(
    client: &Client,
    database_name: &str,
    table_name: &str,
) -> Option<String> {
    let query = format!("show create table {}.{}", database_name, table_name);
    if let Ok(row) = client.query(&query).fetch_one::<ShowCreate>().await {
        return Some(row.statement);
    }

    None
}

async fn patched_create_table(
    client: &Client,
    database_name: &str,
    table_name: &str,
    on_cluster_name: &str,
) -> Option<String> {
    if let Some(query) = show_create_table(client, database_name, table_name).await {
        return patch_create_statement(&query, on_cluster_name);
    }

    None
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
        "select name, engine_full from system.tables where database='{}' and name='{}'",
        database_name, table_name
    );
    match client.query(&query).fetch_all::<SystemTable>().await {
        Ok(rows) => {
            let rslt: Vec<String> = rows.into_iter().map(|r| r.engine).collect();
            if !rslt.is_empty() {
                /*
                                println!(
                                    "[DEBUG] Table {}.{}'s engine:\n{}\nIs sharded ? {}",
                                    database_name,
                                    table_name,
                                    rslt[0],
                                    rslt[0].starts_with("Replicated")
                                );
                */
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

async fn execute_query<S>(client: &Client, query: &str) -> Result<Vec<S>>
where
    S: Row + for<'b> Deserialize<'b>,
{
    //println!("[DEBUG] Execute query: {}", query);
    client.query(query).fetch_all::<S>().await.map_err(|e| {
        let context = format!("[DEBUG] Error executing `{}`", query);
        anyhow::Error::new(e).context(context)
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

fn patch_create_statement(query: &str, cluster_name: &str) -> Option<String> {
    let (first, second) = query.split_at(13);

    if let Some((table_name, rest)) = second.split_once('\n') {
        return Some(format!(
            "{}IF NOT EXISTS {} ON CLUSTER '{{{}}}' {}",
            first, table_name, cluster_name, rest
        ));
    }

    None
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
    let database_name = "default";
    let client = Client::with_http_client(https_client)
        .with_url("https://ckh-0-0.huo.io:443")
        .with_database(database_name);

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

    /* #6 Grab available disk size
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
    */

    rt.block_on(async {
        let table_engine_dict = TableEngine::new(&client, database_name).await;

        let tables = show_tables(&client).await.unwrap();
        for name in tables.iter() {
            // (1)
            let is_sharded = table_engine_dict.is_sharded(name);
            println!("{name} is sharded: {}", is_sharded);

            // (2)
            let contains_macro = table_engine_dict.sharded_contains_macro(name, "{uuid}");
            println!("  {name} contains macro `uuid`: {contains_macro}");

            // Special table
            if contains_macro {
                // (1) Tell if the table is all replicated
                println!(
                    "    is_all_replicated: {}",
                    table_engine_dict
                        .is_all_replicated(&client, name, "shard_")
                        .await
                );

                // (2) Prepare restoration sql
                if let Some(topo) = table_engine_dict.trace_table_topology(name, "shard_") {
                    let create_table_statement =
                        patched_create_table(&client, database_name, name, &topo)
                            .await
                            .unwrap();
                    println!("  statement:\n\n{create_table_statement}\n");
                }
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_create_statement() {
        let query = "CREATE TABLE default.shard_Foo\n(\n    `table` String,\n    `name` String,\n    `commit_hash` String,\n    `database` String DEFAULT 'default'\n)\nENGINE = ReplicatedMergeTree('{zoo_prefix}/tables/{shard}/{uuid}', '{host}')\nPARTITION BY tuple()\nORDER BY (database, table, commit_hash)\nSETTINGS index_granularity = 8192";
        let expected = "CREATE TABLE IF NOT EXISTS default.shard_Foo ON CLUSTER '{standard}' (\n    `table` String,\n    `name` String,\n    `commit_hash` String,\n    `database` String DEFAULT 'default'\n)\nENGINE = ReplicatedMergeTree('{zoo_prefix}/tables/{shard}/{uuid}', '{host}')\nPARTITION BY tuple()\nORDER BY (database, table, commit_hash)\nSETTINGS index_granularity = 8192";

        assert_eq!(
            patch_create_statement(query, "standard").unwrap(),
            expected.to_owned()
        );
    }
}
