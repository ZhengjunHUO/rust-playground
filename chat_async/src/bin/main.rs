use async_std::prelude::*;
use async_std::{net, task};
use async_std::net::TcpStream;
use chat_async::utils;
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
        self.0.lock().unwrap().entry(room.clone()).or_insert_with(|| Arc::new(Room::new(room))).clone()
    }
}

pub async fn handle(req: TcpStream, room_dict: Arc<RoomMap>) -> utils::Result<()> {
    Ok(())
}
