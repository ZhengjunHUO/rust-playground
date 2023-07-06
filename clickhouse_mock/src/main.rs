use futures::{
    task::{Context, Poll},
    Stream, StreamExt,
};
use opensrv_clickhouse::{
    connection::Connection,
    errors::Result,
    types::{Block, Progress},
    CHContext, ClickHouseMetadata, ClickHouseServer,
};
use std::{
    env,
    error::Error,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "clickhouse_srv=debug");
    let host_port = "127.0.0.1:9000";

    let listener = TcpListener::bind(host_port).await?;

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("Server start at {}", host_port);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = ClickHouseServer::run_on_stream(
                Arc::new(Session {
                    last_progress_send: Instant::now(),
                    metadata: ClickHouseMetadata::default()
                        .with_name("ClickHouse-X")
                        .with_major_version(2021)
                        .with_minor_version(5)
                        .with_patch_version(0)
                        .with_tcp_protocol_version(54406)
                        .with_timezone("UTC")
                        .with_display_name("ClickHouse-X"),
                }),
                stream,
            )
            .await
            {
                println!("Error: {:?}", e);
            }
        });
    }
}

struct Session {
    last_progress_send: Instant,
    metadata: ClickHouseMetadata,
}

#[async_trait::async_trait]
impl opensrv_clickhouse::ClickHouseSession for Session {
    async fn execute_query(&self, ctx: &mut CHContext, connection: &mut Connection) -> Result<()> {
        let query = ctx.state.query.clone();
        tracing::debug!("Receive query {}", query);

        let start = Instant::now();

        if query.starts_with("INSERT") || query.starts_with("insert") {
            // ctx.state.out
            let sample_block = Block::new().column("foo", Vec::<u32>::new());
            let (sender, rec) = mpsc::channel(4);
            ctx.state.out = Some(sender);
            connection.write_block(&sample_block).await?;

            let sent_all_data = ctx.state.sent_all_data.clone();
            tokio::spawn(async move {
                let mut rows = 0;
                let mut stream = ReceiverStream::new(rec);
                while let Some(block) = stream.next().await {
                    rows += block.row_count();
                    println!(
                        "got insert block: {:?}, total_rows: {}",
                        block.row_count(),
                        rows
                    );
                }
                sent_all_data.notify_one();
            });
            return Ok(());
        }

        let mut clickhouse_stream = SimpleBlockStream {
            idx: 0,
            start: 10,
            end: 24,
            blocks: 10,
        };

        while let Some(block) = clickhouse_stream.next().await {
            let block = block?;
            connection.write_block(&block).await?;

            if self.last_progress_send.elapsed() >= Duration::from_millis(10) {
                let progress = self.get_progress();
                connection
                    .write_progress(progress, ctx.client_revision)
                    .await?;
            }
        }

        let duration = start.elapsed();
        tracing::debug!(
            "ClickHouseHandler executor cost:{:?}, statistics:{:?}",
            duration,
            "xxx",
        );
        Ok(())
    }

    fn metadata(&self) -> &ClickHouseMetadata {
        &self.metadata
    }

    fn get_progress(&self) -> Progress {
        Progress {
            rows: 100,
            bytes: 1000,
            total_rows: 1000,
        }
    }
}

struct SimpleBlockStream {
    idx: u32,
    start: u32,
    end: u32,
    blocks: u32,
}

impl Stream for SimpleBlockStream {
    type Item = Result<Block>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.idx += 1;
        if self.idx > self.blocks {
            return Poll::Ready(None);
        }
        let block = Some(Block::new().column("abc", (self.start..self.end).collect::<Vec<u32>>()));

        thread::sleep(Duration::from_millis(100));
        Poll::Ready(block.map(Ok))
    }
}
