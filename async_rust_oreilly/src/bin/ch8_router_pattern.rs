use serde_json;
use std::f32::MANTISSA_DIGITS;
use std::{collections::HashMap, sync::OnceLock};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot,
};
use tokio::time::{self, Duration, Instant};

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
    KeepAlive(ActorType),
    Reset(ActorType),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ActorType {
    KVActor,
    WriterActor,
}

enum WriterPayload {
    Set(String, Vec<u8>),
    Get(oneshot::Sender<HashMap<String, Vec<u8>>>),
    Del(String),
}

impl WriterPayload {
    // construct WriterPayload from the KVPayload without consuming it
    fn from_kv_payload(msg: &KVPayload) -> Option<WriterPayload> {
        match msg {
            KVPayload::Set(payload) => Some(WriterPayload::Set(
                payload.key.clone(),
                payload.value.clone(),
            )),
            KVPayload::Get(_) => None,
            KVPayload::Del(payload) => Some(WriterPayload::Del(payload.key.clone())),
        }
    }
}

async fn load_from_file(path: &str) -> io::Result<HashMap<String, Vec<u8>>> {
    let mut file = File::open(path).await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;
    let rslt = serde_json::from_str(&buf)?;
    Ok(rslt)
}

async fn load_state_map(path: &str) -> HashMap<String, Vec<u8>> {
    match load_from_file(path).await {
        Ok(state) => {
            println!("State resumed from file successfully: {:?}", state);
            return state;
        }
        Err(e) => {
            println!(
                "Error occurred reading from file: {:?}\nUse empty state instead.",
                e
            );
            return HashMap::new();
        }
    }
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
            RoutingPayload::KeepAlive(actor_type) => todo!(),
            RoutingPayload::Reset(actor_type) => todo!(),
        }
    }
}

// 接收并处理Router分发的msg
async fn kv_actor(mut recv: Receiver<KVPayload>) {
    // 创建一个writer actor，把收到的msg抄送一份给它，让它写到一个文件中
    let (tx_writer, rx_writer) = mpsc::channel(128);
    tokio::spawn(writer_actor(rx_writer));

    // 让writer actor从文件中读取保存的进度作为初始dict
    let (tx_writer_oneshot, rx_writer_oneshot) = oneshot::channel();
    let _ = tx_writer.send(WriterPayload::Get(tx_writer_oneshot)).await;
    let mut dict = rx_writer_oneshot.await.unwrap();

    let timeout = Duration::from_millis(200);
    let tx_router = TX_ROUTER.get().unwrap().clone();

    loop {
        match time::timeout(timeout, recv.recv()).await {
            Ok(Some(msg)) => {
                // 向writer actor发送收到的消息的副本，同步写入文件
                if let Some(msg_for_writer) = WriterPayload::from_kv_payload(&msg) {
                    let _ = tx_writer.send(msg_for_writer).await;
                }

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
            Ok(None) => break,
            Err(_) => {
                // send a heartbeat message to the router: the key-value store is still alive
                tx_router
                    .send(RoutingPayload::KeepAlive(ActorType::KVActor))
                    .await
                    .unwrap();
            }
        }
    }
}

// created by kv_actor
async fn writer_actor(mut recv: Receiver<WriterPayload>) -> io::Result<()> {
    let mut state = load_state_map("./saved_state.json").await;
    let mut file = File::create("./saved_state.json").await.unwrap();

    let timeout = Duration::from_millis(200);
    let tx_router = TX_ROUTER.get().unwrap().clone();

    loop {
        match time::timeout(timeout, recv.recv()).await {
            Ok(Some(msg)) => {
                match msg {
                    WriterPayload::Set(key, value) => {
                        state.insert(key, value);
                    }
                    WriterPayload::Get(sender) => {
                        let _ = sender.send(state.clone());
                    }
                    WriterPayload::Del(key) => {
                        state.remove(&key);
                    }
                }
                let buf = serde_json::to_string(&state).unwrap();
                file.set_len(0).await?;
                file.seek(std::io::SeekFrom::Start(0)).await?;
                file.write_all(buf.as_bytes()).await?;
                file.flush().await?;
            }
            Ok(None) => break,
            Err(_) => {
                tx_router
                    .send(RoutingPayload::KeepAlive(ActorType::WriterActor))
                    .await
                    .unwrap();
            }
        }
    }
    Ok(())
}

async fn supervisor_actor(mut recv: Receiver<ActorType>) {
    let timeout = Duration::from_millis(200);
    let mut dict = HashMap::new();

    loop {
        match time::timeout(timeout, recv.recv()).await {
            Ok(Some(actor)) => {
                dict.insert(actor, Instant::now());
            }
            Ok(None) => break,
            Err(_) => continue,
        }

        let second_ago = Instant::now() - Duration::from_secs(1);
        for (key, &value) in dict.iter() {
            if value < second_ago {
                match key {
                    ActorType::KVActor | ActorType::WriterActor => {
                        // 重启KVActor也会重启WriterActor
                        TX_ROUTER
                            .get()
                            .unwrap()
                            .send(RoutingPayload::Reset(ActorType::KVActor))
                            .await
                            .unwrap();
                        dict.remove(&ActorType::KVActor);
                        dict.remove(&ActorType::WriterActor);
                        break;
                    }
                }
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

pub async fn get(key: String) -> Result<Option<Vec<u8>>, std::io::Error> {
    let (tx, rx) = oneshot::channel();
    TX_ROUTER
        .get()
        .unwrap()
        .send(RoutingPayload::KV(KVPayload::Get(GetKVPayload {
            key,
            resp: tx,
        })))
        .await
        .unwrap();
    let result = rx.await.unwrap();
    Ok(result)
}

pub async fn del(key: String) -> Result<(), std::io::Error> {
    let (tx, rx) = oneshot::channel();
    TX_ROUTER
        .get()
        .unwrap()
        .send(RoutingPayload::KV(KVPayload::Del(DelKVPayload {
            key,
            resp: tx,
        })))
        .await
        .unwrap();
    rx.await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Prepare router actor
    let (tx, rx) = mpsc::channel(128);
    // Init global var
    TX_ROUTER.set(tx).unwrap();
    tokio::spawn(router_actor(rx));

    // Inputs
    let key = "rust".to_owned();
    set(key.clone(), b"rocks".to_vec()).await?;
    println!("Set done");
    let val = get(key.clone()).await?.unwrap();
    println!("Got: {:?}", String::from_utf8(val));
    /*
        del(key.clone()).await?;
        println!("Delete done");
        let val = get(key).await?;
        println!("Got: {:?}", val);
    */
    // 让writer actor有足够的时间写文件
    std::thread::sleep(Duration::from_secs(3));
    Ok(())
}
