use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
use Command::{Get, Set};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        sdr: Sender<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        sdr: Sender<()>,
    },
}

type Sender<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(64);
    let tx_clone = tx.clone();

    let handle_client = tokio::spawn(async move {
        let mut conn = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(text) = rx.recv().await {
            match text {
                Get { key, sdr } => {
                    let rslt = conn.get(&key).await;
                    let _ = sdr.send(rslt);
                }
                Set { key, val, sdr } => {
                    let rslt = conn.set(&key, val).await;
                    let _ = sdr.send(rslt);
                }
            }
        }
    });

    let handle_get = tokio::spawn(async move {
        let (tx_rslt, rx_rslt) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            sdr: tx_rslt,
        };

        tx.send(cmd).await.unwrap();
        let rslt = rx_rslt.await;
        println!("Server response: {:?}", rslt);
    });

    let handle_set = tokio::spawn(async move {
        let (tx_rslt, rx_rslt) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            sdr: tx_rslt,
        };

        tx_clone.send(cmd).await.unwrap();
        let rslt = rx_rslt.await;
        println!("Server response: {:?}", rslt);
    });

    handle_set.await.unwrap();
    handle_get.await.unwrap();
    handle_client.await.unwrap();
}
