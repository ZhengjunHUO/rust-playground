use tokio::sync::{mpsc::Receiver, oneshot};

struct SimplePayload {
    value: usize,
    reply: oneshot::Sender<String>,
}

// async function listening for messages: most basic actor
async fn simple_actor(mut rx: Receiver<SimplePayload>) {
    let mut counter = 0;
    while let Some(msg) = rx.recv().await {
        counter += msg.value;
        println!(
            "[Actor] Recv value: {}; Total count: {}",
            msg.value, counter
        );
        if let Err(err) = msg.reply.send(format!("[Actor] Recv value: {}", msg.value)) {
            eprint!(
                "[Actor] Error occurred sending message back to main: {}",
                err
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<SimplePayload>(20);

    tokio::spawn(simple_actor(rx));
    for i in 1..6 {
        let (one_tx, one_rx) = oneshot::channel::<String>();
        tx.send(SimplePayload {
            value: i,
            reply: one_tx,
        })
        .await
        .unwrap();
        match one_rx.await {
            Ok(_) => println!("[Main] Ack"),
            Err(_) => println!("[Main] Can't recv ack from actor"),
        }
    }
}
