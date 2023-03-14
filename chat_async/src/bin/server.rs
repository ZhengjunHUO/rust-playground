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
    // reads in user specified ip:port to listen on
    let ep = env::args()
        .nth(1)
        .expect("Wait for an endpoint, example:\n  localhost:8080");

    // creates a struct to holds rooms
    let room_dict = Arc::new(RoomMap::new());

    task::block_on(async {
        // gets listener on ip:port
        let l = net::TcpListener::bind(ep).await?;

        // returns a stream of incoming connections
        let mut stream = l.incoming();
        // wait for new connection comming in
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

    // called when client wants to enter a room
    // attach client-provided stream to broadcast messages to client later
    pub fn attach(&self, reply: Arc<Reply>) {
        let recvr = self.sender.subscribe();
        task::spawn(handle_receiver(self.name.clone(), recvr, reply));
    }

    // called when some client in room sends a message
    // use broadcast::Sender to broadcast it to others
    pub fn send(&self, content: Arc<String>) {
        let _rslt = self.sender.send(content);
    }
}

async fn handle_receiver(
    room: Arc<String>,
    mut recv: broadcast::Receiver<Arc<String>>,
    reply: Arc<Reply>,
) {
    // subscribes to some Room, constructs reply from server
    // sends it back to client using the TcpStream "reply"
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
    // splits egress stream from igress
    // used to write message back to client
    let reply_chan = Arc::new(Reply::new(conn.clone()));

    // prepares a stream returning unmarshalled request from client
    let br = io::BufReader::new(conn);
    let mut s = utils::recv_and_unmarshal(br);

    while let Some(package) = s.next().await {
        let msg = package?;

        println!("[DEBUG] Recieve msg: {:?}", msg);
        let rslt = match msg {
            // case client wants to join to a chat room
            ProtoClient::Reg { room } => {
                let r = room_dict.get_or_create(room);
                // attach egress stream (write back to client) to the specific room
                r.attach(reply_chan.clone());
                Ok(())
            }

            // case client wants to send a message across the room he entered
            ProtoClient::Envoy { room, content } => match room_dict.get(&room) {
                Some(r) => {
                    // broadcast::Sender will send the content to all recievers subscribed
                    // (other clients in the same room)
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
