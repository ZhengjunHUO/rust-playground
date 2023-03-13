use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::{io, net, task};
use chat_async::{
    protocol::{ProtoClient, ProtoServer},
    utils,
};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{self, error::RecvError};

fn main() -> utils::Result<()> {
    let ep = env::args()
        .nth(1)
        .expect("Wait for an endpoint, example:\n  localhost:8080");

    // create a struct to holds rooms
    let room_dict = Arc::new(RoomMap::new());

    task::block_on(async {
        let l = net::TcpListener::bind(ep).await?;

        // Returns a stream of incoming connections
        let mut stream = l.incoming();
        while let Some(s) = stream.next().await {
            let conn = s?;
            let point_to_room_dict = room_dict.clone();
            println!("[DEBUG] Connection established: {:?}", conn);
            task::spawn(async {
                // broadcast message inside group
                handle_err(handle(conn, point_to_room_dict).await);
            });
        }

        Ok(())
    })
}

fn handle_err(rslt: utils::Result<()>) {
    if let Err(e) = rslt {
        eprintln!("{e}");
    }
}

pub struct Room {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Room {
    pub fn new(name: Arc<String>) -> Room {
        let (sender, _receiver) = broadcast::channel(1000);
        Room { name, sender }
    }

    pub fn attach(&self, reply: Arc<Reply>) {
        let recvr = self.sender.subscribe();
        task::spawn(handle_receiver(self.name.clone(), recvr, reply));
    }

    pub fn send(&self, content: Arc<String>) {
        let _rslt = self.sender.send(content);
    }
}

async fn handle_receiver(
    room: Arc<String>,
    mut recv: broadcast::Receiver<Arc<String>>,
    reply: Arc<Reply>,
) {
    loop {
        let message = match recv.recv().await {
            Ok(content) => ProtoServer::Envoy {
                room: room.clone(),
                content: content.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                ProtoServer::Error(format!("[Romm {}]: {} skipped messages!", room, n))
            }
            Err(RecvError::Closed) => break,
        };

        if reply.send(message).await.is_err() {
            break;
        }
    }
}

pub struct RoomMap(Mutex<HashMap<Arc<String>, Arc<Room>>>);

impl RoomMap {
    pub fn new() -> RoomMap {
        RoomMap(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, room: &String) -> Option<Arc<Room>> {
        self.0.lock().unwrap().get(room).cloned()
    }

    pub fn get_or_create(&self, room: Arc<String>) -> Arc<Room> {
        self.0
            .lock()
            .unwrap()
            .entry(room.clone())
            .or_insert_with(|| Arc::new(Room::new(room)))
            .clone()
    }
}

pub async fn handle(conn: TcpStream, room_dict: Arc<RoomMap>) -> utils::Result<()> {
    let reply_chan = Arc::new(Reply::new(conn.clone()));

    let br = io::BufReader::new(conn);
    let mut s = utils::recv_and_unmarshal(br);

    while let Some(package) = s.next().await {
        let msg = package?;

        println!("[DEBUG] Recieve msg: {:?}", msg);
        let rslt = match msg {
            ProtoClient::Reg { room } => {
                let r = room_dict.get_or_create(room);
                r.attach(reply_chan.clone());
                Ok(())
            }

            ProtoClient::Envoy { room, content } => match room_dict.get(&room) {
                Some(r) => {
                    r.send(content);
                    Ok(())
                }
                None => Err(format!("Unknown room name: {}!", room)),
            },
        };

        if let Err(e) = rslt {
            reply_chan.send(ProtoServer::Error(e)).await?;
        }
    }

    Ok(())
}

pub struct Reply(async_std::sync::Mutex<TcpStream>);

impl Reply {
    pub fn new(reply: TcpStream) -> Reply {
        Reply(async_std::sync::Mutex::new(reply))
    }

    pub async fn send(&self, content: ProtoServer) -> utils::Result<()> {
        let mut g = self.0.lock().await;
        utils::marshal_and_send(&mut *g, &content).await?;
        g.flush().await?;
        Ok(())
    }
}
