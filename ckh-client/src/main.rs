#![allow(dead_code)]

use anyhow::Result;
use clickhouse_rs::{row, types::Block, ClientHandle, Pool};
use futures_util::{StreamExt, TryStreamExt};
use std::{env, future};

async fn insert(mut client: ClientHandle) -> Result<()> {
    let mut block = Block::with_capacity(5);
    block.push(row! { foo: 1_u32 })?;
    block.push(row! { foo: 3_u32 })?;
    block.push(row! { foo: 5_u32 })?;
    block.push(row! { foo: 7_u32 })?;
    block.push(row! { foo: 9_u32 })?;

    client.insert("bar_db", block).await?;
    Ok(())
}

async fn show_tables(mut client: ClientHandle) -> Result<()> {
    client
        .query(r"show tables")
        //.stream_blocks()
        //.try_for_each(|block| {
        //    println!("Result: {:?}", block);
        .stream()
        .try_for_each(|row| {
            println!("Found table {} !", row.get::<&str, usize>(0).unwrap());
            future::ready(Ok(()))
        })
        .await?;
    Ok(())
}

async fn crud(mut client: ClientHandle) -> Result<()> {
    let ddl = r"
        CREATE TABLE IF NOT EXISTS payment (
            customer_id  UInt32,
            amount       UInt32,
            account_name Nullable(FixedString(3))
        ) Engine=Memory";

    let mut block = Block::with_capacity(5);
    block.push(row! { customer_id: 1_u32, amount:  2_u32, account_name: Some("foo") })?;
    block.push(row! { customer_id: 3_u32, amount:  4_u32, account_name: None::<&str> })?;
    block.push(row! { customer_id: 5_u32, amount:  6_u32, account_name: None::<&str> })?;
    block.push(row! { customer_id: 7_u32, amount:  8_u32, account_name: None::<&str> })?;
    block.push(row! { customer_id: 9_u32, amount: 10_u32, account_name: Some("bar") })?;

    client.execute(ddl).await?;
    client.insert("payment", block).await?;

    let mut stream = client.query("SELECT * FROM payment").stream();
    while let Some(row) = stream.next().await {
        let row = row?;
        let id: u32 = row.get("customer_id")?;
        let amount: u32 = row.get("amount")?;
        let name: Option<&str> = row.get("account_name")?;
        println!("Found payment {}: {} {:?}", id, amount, name);
    }

    Ok(())
}

async fn get_client() -> Result<ClientHandle> {
    let endpoint =
        env::var("DATABASE_URL").unwrap_or_else(|_| "tcp://127.0.0.1:9000?compression=lz4".into());
        //env::var("DATABASE_URL").unwrap_or_else(|_| "tcp://<USERNAME>:<PASSWORD>@<IP>:<PORT>/<DATABASE>?compression=lz4".into());

    let pool = Pool::new(endpoint);
    Ok(pool.get_handle().await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "clickhouse_rs=debug");
    env_logger::init();

    let client = get_client().await?;
    //crud(client).await
    //insert(client).await
    show_tables(client).await
}
