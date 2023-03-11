use async_std::prelude::*;
use async_std::{io, net};
use chat_async::{utils, ProtoClient};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ROOM: Mutex<String> = Mutex::new(String::new());
}

fn main() -> utils::Result<()> {
    Ok(())
}

async fn post(mut post_to_server: net::TcpStream) -> utils::Result<()> {
    println!("<Usage>: \n  1) Join a room\n  $ checkout <ROOM>\n  2) Send message\n  $ <MESSAGE>\n");

    let mut inputs = io::BufReader::new(io::stdin()).lines();
    while let Some(content) = inputs.next().await {
        let input = content?;
        let req = match parse_input(&input) {
            Some(r) => r,
            None => continue,
        };

        utils::marshal_and_send(&mut post_to_server, &req).await?;
        post_to_server.flush().await?;
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
            content: Arc::new(rest.to_string()),
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
