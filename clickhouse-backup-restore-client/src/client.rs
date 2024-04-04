use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashSet;

pub struct Client<C: TableInfoGetter + Sync> {
    //s3_client: Option<S3Client>,
    ckh_client: C,
    //config: Config,
}

pub(crate) struct CkhClient {
    client: clickhouse::Client,
}

/*
struct S3Client {
    client: s3::Bucket,
}
*/

#[derive(Default)]
struct Config {}

#[async_trait]
pub trait TableInfoGetter {
    async fn list_tables(&self) -> Result<HashSet<String>>;
}

#[async_trait]
impl TableInfoGetter for CkhClient {
    async fn list_tables(&self) -> Result<HashSet<String>> {
        Ok(HashSet::new())
    }
}

#[async_trait]
impl<C: TableInfoGetter + Sync> TableInfoGetter for Client<C> {
    async fn list_tables(&self) -> Result<HashSet<String>> {
        self.ckh_client.list_tables().await
    }
}

impl Client<CkhClient> {
    pub fn new() -> Self {
        Client {
            //s3_client: None,
            ckh_client: CkhClient {
                client: clickhouse::Client::default(),
            },
            //config: Config::default(),
        }
    }
}
