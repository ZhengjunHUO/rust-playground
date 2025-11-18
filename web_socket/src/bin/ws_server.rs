use futures_channel::mpsc::{self, UnboundedSender};
use futures_util::{TryStreamExt, future, pin_mut, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::net::TcpListener;
use tokio::time::interval;
use tokio_native_tls::{TlsAcceptor, native_tls};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};

#[derive(Deserialize)]
struct HandshakeMessage {
    #[serde(rename = "type")]
    msg_type: String,
    name: String,
    version: Option<String>,
}

#[derive(Serialize)]
struct SystemMessage {
    #[serde(rename = "type")]
    msg_type: String,
    message: String,
    timestamp: u64,
    connected_clients: usize,
}

#[derive(Clone)]
struct ClientInfo {
    sender: UnboundedSender<Message>,
    name: String,
    connected_at: SystemTime,
}

type ConnManager = Arc<Mutex<HashMap<SocketAddr, ClientInfo>>>;

async fn system_message_broadcaster(manager: ConnManager) {
    let mut interval = interval(Duration::from_secs(15)); // Broadcast every 15 seconds

    loop {
        interval.tick().await;

        // Generate system message
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let guard = manager.lock().unwrap();
        let connected_count = guard.len();

        if connected_count > 0 {
            let system_msg = SystemMessage {
                msg_type: "system".to_string(),
                message: "Server heartbeat".to_string(),
                timestamp,
                connected_clients: connected_count,
            };

            let system_msg_json = serde_json::to_string(&system_msg).unwrap();

            // Collect client names for logging
            let client_names: Vec<String> =
                guard.values().map(|client| client.name.clone()).collect();

            println!(
                "Broadcasting system message to {} clients [{}]: {}",
                connected_count,
                client_names.join(", "),
                system_msg_json
            );

            // Collect failed senders to remove them later
            let mut failed_senders = Vec::new();

            for (addr, client_info) in guard.iter() {
                if client_info
                    .sender
                    .unbounded_send(Message::Text(system_msg_json.clone().into()))
                    .is_err()
                {
                    // Mark this sender as failed (client probably disconnected)
                    failed_senders.push(*addr);
                }
            }

            // Clean up failed senders
            drop(guard); // Release the lock before removing entries
            if !failed_senders.is_empty() {
                let mut guard = manager.lock().unwrap();
                for addr in failed_senders {
                    if let Some(client_info) = guard.remove(&addr) {
                        println!(
                            "Removing failed connection: {} ({})",
                            addr, client_info.name
                        );
                    }
                }
            }
        }
    }
}

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

    // Start the system message broadcaster task
    let broadcaster_manager = manager.clone();
    tokio::spawn(async move {
        system_message_broadcaster(broadcaster_manager).await;
    });

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
    let tx_cloned = tx.clone();

    // Initially insert with placeholder name
    let client_info = ClientInfo {
        sender: tx,
        name: "unknown".to_string(),
        connected_at: SystemTime::now(),
    };
    manager.lock().unwrap().insert(addr, client_info);

    let manager_clone = manager.clone();
    let read_and_broadcast = stream.try_for_each(move |msg| {
        match msg {
            Message::Text(text) => {
                // Try to parse as handshake message first
                if let Ok(handshake) = serde_json::from_str::<HandshakeMessage>(&text)
                    && handshake.msg_type == "handshake"
                {
                    // Update client info with the provided name
                    let mut guard = manager_clone.lock().unwrap();
                    if let Some(client_info) = guard.get_mut(&addr) {
                        client_info.name = handshake.name.clone();
                        println!("Client {} registered with name: {}", addr, handshake.name);
                    }
                    return future::ok(());
                }

                // Regular message - broadcast to all clients
                let guard = manager_clone.lock().unwrap();
                let sender_name = guard
                    .get(&addr)
                    .map(|client| client.name.as_str())
                    .unwrap_or("unknown");

                println!("Received message from {} ({}): {}", addr, sender_name, text);

                // Broadcast to all connected clients
                for (client_addr, client_info) in guard.iter() {
                    if *client_addr != addr {
                        // Don't echo back to sender
                        let formatted_msg = format!("[{}]: {}", sender_name, text);
                        let _ = client_info
                            .sender
                            .unbounded_send(Message::Text(formatted_msg.into()));
                    }
                }
            }
            Message::Close(_) => {
                // Disconnect
                tx_cloned.unbounded_send(msg).unwrap();
            }
            _ => {}
        }

        future::ok(())
    });

    let recv_and_writeback = rx.map(Ok).forward(sink);

    pin_mut!(read_and_broadcast, recv_and_writeback);
    tokio::select! {
        _ = read_and_broadcast => {},
        _ = recv_and_writeback => {},
    };

    let client_name = manager
        .lock()
        .unwrap()
        .get(&addr)
        .map(|client| client.name.clone())
        .unwrap_or_else(|| "unknown".to_string());

    println!("Client {} ({}) disconnected.", addr, client_name);
    manager.lock().unwrap().remove(&addr);
}
