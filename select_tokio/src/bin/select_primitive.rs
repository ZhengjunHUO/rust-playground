use std::{thread, time};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async {
        thread::sleep(time::Duration::from_millis(100));
        let _ = tx.send("foo");
    });

    tokio::select! {
        Ok(resp) = rx => {
            println!("rx won the race with [{:?}]", resp);
        }
        sock = TcpListener::bind("127.0.0.1:1234") => {
            println!("binding [{:?}] won the race", sock);
        }
    }
    println!("Done !");
}
