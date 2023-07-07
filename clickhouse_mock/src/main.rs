use futures::{
    task::{Context, Poll},
    Stream, StreamExt,
};
use opensrv_clickhouse::{
    connection::Connection, errors::Result, types::Block, CHContext, ClickHouseMetadata,
    ClickHouseServer,
};
use std::{
    error::Error,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let endpoint = "127.0.0.1:9000";
    let listener = TcpListener::bind(endpoint).await?;

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    tracing::info!("Server up: {}", endpoint);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = ClickHouseServer::run_on_stream(
                Arc::new(Session {
                    metadata: ClickHouseMetadata::default()
                        .with_name("ClickHouse")
                        .with_major_version(23)
                        .with_minor_version(3)
                        .with_patch_version(54462)
                        .with_tcp_protocol_version(54406)
                        .with_timezone("UTC")
                        .with_display_name("ClickHouse-Mock"),
                }),
                stream,
            )
            .await
            {
                println!("Error occurred: {:?}", e);
            }
        });
    }
}

struct Session {
    metadata: ClickHouseMetadata,
}

#[async_trait::async_trait]
impl opensrv_clickhouse::ClickHouseSession for Session {
    async fn execute_query(&self, ctx: &mut CHContext, connection: &mut Connection) -> Result<()> {
        let query = ctx.state.query.clone();
        tracing::info!("Incoming query: {}", query);

        if query == "show tables" {
            let start = Instant::now();

            let mut clickhouse_stream = ShowTableStream { pivot: 0, seuil: 2 };

            while let Some(block) = clickhouse_stream.next().await {
                let block = block?;
                connection.write_block(&block).await?;
            }

            let duration = start.elapsed();
            tracing::info!("Done! Time elapsed :{:?}", duration);
        }
        Ok(())
    }

    fn metadata(&self) -> &ClickHouseMetadata {
        &self.metadata
    }
}

struct ShowTableStream {
    pivot: u32,
    seuil: u32,
}

impl Stream for ShowTableStream {
    type Item = Result<Block>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.pivot += 1;
        if self.pivot > self.seuil {
            return Poll::Ready(None);
        }

        let block = Some(Block::new().column(
            "name",
            vec![
                "shard_bar".to_string(),
                "shard_baz".to_string(),
                "shard_huo".to_string(),
            ],
        ));

        thread::sleep(Duration::from_millis(100));
        Poll::Ready(block.map(Ok))
    }
}
