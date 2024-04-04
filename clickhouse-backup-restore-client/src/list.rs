use crate::client::{Client, TableInfoGetter};
use anyhow::Result;
use std::collections::HashSet;

pub async fn list_clickhouse_tables<C: TableInfoGetter + Sync>(
    client: &Client<C>,
) -> Result<HashSet<String>> {
    client.list_tables().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_clickhouse_tables() {
        let client = Client::new();
        let expected = HashSet::new();
        assert_eq!(expected, list_clickhouse_tables(&client).await.unwrap());
    }
}
