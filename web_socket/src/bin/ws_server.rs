use futures_channel::mpsc::{self, UnboundedSender};
use futures_util::{TryStreamExt, future, pin_mut, stream::StreamExt};
use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type ConnManager = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    println!("Server up on localhost:8888");

    let manager = ConnManager::new(Mutex::new(HashMap::new()));

    while let Ok((conn, addr)) = listener.accept().await {
        let cm = manager.clone();
        tokio::spawn(handle_conn(conn, addr, cm));
    }
    Ok(())
}

async fn handle_conn(conn: TcpStream, addr: SocketAddr, manager: ConnManager) {
    let stream = accept_async(conn)
        .await
        .expect("Error occurred accepting a new websocket");
    let (sink, stream) = stream.split();

    let (tx, rx) = mpsc::unbounded::<Message>();
    manager.lock().unwrap().insert(addr, tx);

    let read_and_broadcast = stream.try_for_each(|msg| {
        println!("Received message from {}: {}", addr, msg.to_text().unwrap());
        let guard = manager.lock().unwrap();

        let senders = guard.iter().map(|(_, tx)| tx);
        for sender in senders {
            sender.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())
    });

    let recv_and_writeback = rx.map(Ok).forward(sink);
    pin_mut!(read_and_broadcast, recv_and_writeback);

    tokio::select! {
        _ = read_and_broadcast => {},
        _ = recv_and_writeback => {},
    };

    println!("Client {} disconnected.", addr);
    manager.lock().unwrap().remove(&addr);
}
