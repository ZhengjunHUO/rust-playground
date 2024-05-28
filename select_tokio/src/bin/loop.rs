use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(8);
    let (tx2, mut rx2) = mpsc::channel(8);

    tokio::spawn(async move {
        let _ = tx1.send(1).await;
        let _ = tx1.send(3).await;
    });

    tokio::spawn(async move {
        let _ = tx2.send(2).await;
        let _ = tx2.send(4).await;
    });

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => {
                println!("[DEBUG] Recv msg from channel 1: {:?}", msg);
                msg
            }
            Some(msg) = rx2.recv() => {
                println!("[DEBUG] Recv msg from channel 2: {:?}", msg);
                msg
            }
            else => {
                println!("[DEBUG] else branch matched, quit loop !");
                break
            }
        };

        println!("{msg}");
    }

    println!("Done");
}
