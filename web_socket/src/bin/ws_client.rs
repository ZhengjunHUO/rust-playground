use futures_channel::mpsc::{UnboundedSender, unbounded};
use futures_util::{pin_mut, stream::StreamExt};
use names::Generator;
use std::env;
use tokio::io::{AsyncBufReadExt, BufReader, stdin};
use tokio_native_tls::native_tls;
use tokio_tungstenite::{
    Connector, connect_async, connect_async_tls_with_config, tungstenite::Message,
};

#[tokio::main]
async fn main() {
    let sock = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_owned());

    let tls_enabled = env::args()
        .nth(2)
        .map(|arg| !arg.is_empty())
        .unwrap_or(false);

    let url = format!("{}{}", if tls_enabled { "wss://" } else { "ws://" }, sock);

    let (wsstream, _) = if tls_enabled {
        let tls_connector = Connector::NativeTls(
            native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("Error initiating tls connector"),
        );
        connect_async_tls_with_config(&url, None, false, Some(tls_connector))
            .await
            .expect("Error connecting to ws server")
    } else {
        connect_async(&url)
            .await
            .expect("Error connecting to ws server")
    };
    println!("Connected to {}.", url);
    let (sink, mut stream) = wsstream.split();

    let mut generator = Generator::default();
    let client_name = generator.next().unwrap_or("rust_client".to_owned());

    let (tx, rx) = unbounded::<Message>();
    tokio::spawn(client_input(tx, client_name));

    let recv_and_write = rx.map(Ok).forward(sink);
    let read_and_print = tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("← Received: {}", text),
                Ok(Message::Close(_)) => {
                    println!("Connection closed by server");
                    break;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    pin_mut!(recv_and_write, read_and_print);
    tokio::select! {
        _ = recv_and_write => {},
        _ = read_and_print => {},
    }
}

async fn client_input(tx: UnboundedSender<Message>, client_name: String) {
    let input = stdin();
    let mut reader = BufReader::new(input);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    tx.unbounded_send(Message::Text(
                        format!("[{}] {}", client_name, trimmed).into(),
                    ))
                    .unwrap();
                };
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}
