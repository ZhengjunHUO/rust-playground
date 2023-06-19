use anyhow::Result;
use clickhouse::Client;
use clickhouse::Row;
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Row, Deserialize)]
#[allow(non_snake_case)]
struct MyRow<'a> {
    level: &'a str,
    className: &'a str,
}

fn main() -> Result<()> {
    let client = Client::default()
        .with_url("http://ckh.huo.io:80")
        .with_user("rafal")
        .with_password("thisIsDevPassword")
        .with_database("default");

    let rt = Runtime::new().unwrap();
    let mut cursor = client
        .query("SELECT ?fields FROM rafal_logging")
        .fetch::<MyRow<'_>>()?;

    rt.block_on(async {
        while let Some(row) = cursor.next().await? {
            println!("Got level: {}; classname: {}", row.level, row.className);
        }
        Ok(())
    })
}
