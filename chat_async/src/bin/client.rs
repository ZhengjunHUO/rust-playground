use async_std::prelude::*;
use async_std::{io, net, task};
use chat_async::{
    protocol::{ProtoClient, ProtoServer},
    utils,
};
use lazy_static::lazy_static;
use std::env;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ROOM: Mutex<String> = Mutex::new(String::new());
}

fn main() -> utils::Result<()> {
    let ep = env::args()
        .nth(1)
        .expect("Wait for an endpoint, example:\n  localhost:8080");
    task::block_on(async {
        let conn = net::TcpStream::connect(ep).await?;
        conn.set_nodelay(true)?;

        let send_fn = send(conn.clone());
        let recv_fn = recv(conn);

        recv_fn.race(send_fn).await?;
        Ok(())
    })
}

async fn send(mut send_to_server: net::TcpStream) -> utils::Result<()> {
    println!(
        "<Usage>: \n  1) Join a room\n  $ checkout <ROOM>\n  2) Send message\n  $ <MESSAGE>\n"
    );

    let mut inputs = io::BufReader::new(io::stdin()).lines();
    while let Some(content) = inputs.next().await {
        let input = content?;
        let req = match parse_input(&input) {
            Some(r) => r,
            None => continue,
        };

        utils::marshal_and_send(&mut send_to_server, &req).await?;
        send_to_server.flush().await?;
    }

    Ok(())
}

async fn recv(recv_from_server: net::TcpStream) -> utils::Result<()> {
    let br = io::BufReader::new(recv_from_server);
    let mut s = utils::recv_and_unmarshal(br);

    while let Some(msg) = s.next().await {
        match msg? {
            ProtoServer::Envoy { room, content } => {
                println!("[{}] {}", room, content);
            }
            ProtoServer::Error(err) => {
                println!("[ERROR] Server: {}", err);
            }
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> Option<ProtoClient> {
    let (first, rest) = split_first_word(input)?;
    if first == "checkout" {
        let (room, _) = split_first_word(rest)?;
        *ROOM.lock().unwrap() = room.to_string();
        return Some(ProtoClient::Reg {
            room: Arc::new(room.to_string()),
        });
    } else {
        let room_name = &*ROOM.lock().unwrap();
        if room_name.len() == 0 {
            return None;
        }
        return Some(ProtoClient::Envoy {
            room: Arc::new(room_name.to_string()),
            content: Arc::new(input.to_string()),
        });
    }
}

fn split_first_word(input: &str) -> Option<(&str, &str)> {
    let line = input.trim_start();

    if line.is_empty() {
        return None;
    }

    match line.find(char::is_whitespace) {
        Some(idx) => Some((&line[0..idx], &line[idx..])),
        None => Some((line, "")),
    }
}
