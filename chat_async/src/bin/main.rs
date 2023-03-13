use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::{io, net, task};
use chat_async::{protocol, utils};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

fn main() -> utils::Result<()> {
    let ep = env::args()
        .nth(1)
        .expect("Wait for an endpoint, example:\n  localhost:8080");

    // create a struct to holds rooms
    let room_dict = Arc::new(RoomMap::new());

    task::block_on(async {
        let socket = net::TcpListener::bind(ep).await?;

        let mut conns = socket.incoming();
        while let Some(conn) = conns.next().await {
            let req = conn?;
            let point_to_room_dict = room_dict.clone();
            task::spawn(async {
                // broadcast message inside group
                handle_err(handle(req, point_to_room_dict).await);
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
}

impl Room {
    pub fn new(name: Arc<String>) -> Room {
        Room { name }
    }

    //pub fn attach(&self, reply: Arc<Reply>) {}
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

pub async fn handle(req: TcpStream, room_dict: Arc<RoomMap>) -> utils::Result<()> {
    let reply_chan = Arc::new(Reply::new(req.clone()));

    let br = io::BufReader::new(req);
    let mut s = utils::recv_and_unmarshal(br);

    while let Some(package) = s.next().await {
        let msg = package?;

        let rslt = match msg {
            protocol::ProtoClient::Reg { room } => {
                let r = room_dict.get_or_create(room);
                //r.attach(reply_chan.clone())
                Ok(())
            }

            protocol::ProtoClient::Envoy { room, content } => match room_dict.get(&room) {
                Some(r) => Ok(()),
                None => Err(format!("Unknown room name: {}!", room)),
            },
        };

        if let Err(e) = rslt {
            reply_chan.send(protocol::ProtoServer::Error(e)).await?;
        }
    }

    Ok(())
}

pub struct Reply(async_std::sync::Mutex<TcpStream>);

impl Reply {
    pub fn new(reply: TcpStream) -> Reply {
        Reply(async_std::sync::Mutex::new(reply))
    }

    pub async fn send(&self, content: protocol::ProtoServer) -> utils::Result<()> {
        let mut g = self.0.lock().await;
        utils::marshal_and_send(&mut *g, &content).await?;
        g.flush().await?;
        Ok(())
    }
}
