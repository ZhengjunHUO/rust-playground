use futures_channel::mpsc::{self, UnboundedSender};
use futures_util::{TryStreamExt, future, pin_mut, stream::StreamExt};
use std::{
    collections::HashMap,
    env,
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tokio_native_tls::{TlsAcceptor, native_tls};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

type ConnManager = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tls_acceptor: Option<TlsAcceptor> = None;

    let tls_enabled = env::args()
        .nth(2)
        .map(|arg| !arg.is_empty())
        .unwrap_or(false);
    if tls_enabled {
        let cert = std::fs::read("server.crt.pem").expect("Error reading server certificate");
        let key = std::fs::read("server.key.pem").expect("Error reading server key");

        let id = native_tls::Identity::from_pkcs8(&cert, &key)
            .expect("Error preparing identity from cert and key");
        tls_acceptor = Some(TlsAcceptor::from(
            native_tls::TlsAcceptor::builder(id)
                .build()
                .expect("Error building tls acceptor"),
        ));
    }

    let sock = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_owned());
    let listener = TcpListener::bind(&sock).await?;
    println!(
        "Server up on {}{}",
        if tls_enabled { "wss://" } else { "ws://" },
        sock
    );

    let manager = ConnManager::new(Mutex::new(HashMap::new()));

    while let Ok((conn, addr)) = listener.accept().await {
        let tls_accptor_clone = tls_acceptor.clone();
        let manager_clone = manager.clone();
        tokio::spawn(async move {
            match tls_accptor_clone {
                Some(acceptor) => {
                    let tls_stream = acceptor
                        .accept(conn)
                        .await
                        .expect("Error establishing a connection over TLS");
                    let stream = accept_async(tls_stream)
                        .await
                        .expect("Error occurred accepting a new websocket");
                    handle_conn(stream, addr, manager_clone).await;
                }
                None => {
                    let stream = accept_async(conn)
                        .await
                        .expect("Error occurred accepting a new websocket");
                    handle_conn(stream, addr, manager_clone).await;
                }
            };
        });
    }
    Ok(())
}

async fn handle_conn<S>(conn: WebSocketStream<S>, addr: SocketAddr, manager: ConnManager)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    println!("Connection established with {} !", addr);
    let (sink, stream) = conn.split();

    let (tx, rx) = mpsc::unbounded::<Message>();
    manager.lock().unwrap().insert(addr, tx);

    let read_and_broadcast = stream.try_for_each(|msg| {
        println!("Received message from {}: {}", addr, msg.to_text().unwrap());
        let guard = manager.lock().unwrap();

        let senders = guard.values();
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
