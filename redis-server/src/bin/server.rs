use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Dict = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let ep = "127.0.0.1:6379";
    let lstnr = TcpListener::bind(ep).await.unwrap();
    println!("Server is up on {}", ep);

    let dict = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (sock, _) = lstnr.accept().await.unwrap();
        let d = dict.clone();

        tokio::spawn(async move {
            handle(sock, d).await;
        });
    }
}

async fn handle(sock: TcpStream, dict: Dict) {
    let mut conn = Connection::new(sock);

    while let Some(pkt) = conn.read_frame().await.unwrap() {
        println!("Receive from client: {:?}", pkt);

        let resp = match Command::from_frame(pkt).unwrap() {
            Set(cmd) => {
                let mut d = dict.lock().unwrap();
                d.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let d = dict.lock().unwrap();
                if let Some(val) = d.get(cmd.key()) {
                    Frame::Bulk(val.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => Frame::Error(format!("Unknown op: {:?} !", cmd)),
        };
        conn.write_frame(&resp).await.unwrap();
    }
}
