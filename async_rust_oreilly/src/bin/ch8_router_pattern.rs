use std::{collections::HashMap, sync::OnceLock};

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};

struct SetKVPayload {
    key: String,
    value: Vec<u8>,
    resp: oneshot::Sender<()>,
}

struct GetKVPayload {
    key: String,
    resp: oneshot::Sender<Option<Vec<u8>>>,
}

struct DelKVPayload {
    key: String,
    resp: oneshot::Sender<()>,
}

enum KVPayload {
    Set(SetKVPayload),
    Get(GetKVPayload),
    Del(DelKVPayload),
}

enum RoutingPayload {
    KV(KVPayload),
}

// 作为interface接收外部发送的msg，转发给kv_actor（由其创建）
async fn router_actor(mut recv: Receiver<RoutingPayload>) {
    // 创建kv_actor
    let (tx_kv, rx_kv) = mpsc::channel(128);
    tokio::spawn(kv_actor(rx_kv));

    // 接收外部发送的msg
    while let Some(msg) = recv.recv().await {
        match msg {
            RoutingPayload::KV(kv_payload) => {
                let _ = tx_kv.send(kv_payload).await;
            }
        }
    }
}

// 接收并处理Router分发的msg
async fn kv_actor(mut recv: Receiver<KVPayload>) {
    let mut dict = HashMap::new();
    while let Some(msg) = recv.recv().await {
        match msg {
            KVPayload::Set(SetKVPayload { key, value, resp }) => {
                dict.insert(key, value);
                let _ = resp.send(());
            }
            KVPayload::Get(GetKVPayload { key, resp }) => {
                let _ = resp.send(dict.get(&key).cloned());
            }
            KVPayload::Del(DelKVPayload { key, resp }) => {
                dict.remove(&key);
                let _ = resp.send(());
            }
        }
    }
}

// router_actor的发送端，提供给外部发送信息
static TX_ROUTER: OnceLock<Sender<RoutingPayload>> = OnceLock::new();

// 模拟外部发送的请求
pub async fn set(key: String, value: Vec<u8>) -> Result<(), std::io::Error> {
    let (tx, rx) = oneshot::channel();
    TX_ROUTER
        .get()
        .unwrap()
        .send(RoutingPayload::KV(KVPayload::Set(SetKVPayload {
            key,
            value,
            resp: tx,
        })))
        .await
        .unwrap();
    rx.await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {}
