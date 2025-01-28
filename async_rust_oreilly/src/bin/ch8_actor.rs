use std::sync::Arc;

use tokio::sync::{mpsc::Receiver, oneshot, Mutex};

struct SimplePayload {
    value: usize,
    reply: oneshot::Sender<String>,
}

// async function listening for messages: most basic actor
async fn simple_actor(mut rx: Receiver<SimplePayload>) {
    let mut counter = 0;
    while let Some(msg) = rx.recv().await {
        counter += msg.value;
        /*
        println!(
            "[Actor] Recv value: {}; Total count: {}",
            msg.value, counter
        );
        */
        if let Err(err) = msg.reply.send(format!("[Actor] Recv value: {}", msg.value)) {
            eprint!(
                "[Actor] Error occurred sending message back to main: {}",
                err
            );
        }
    }
}

async fn simple_actor_using_mutex(counter: Arc<Mutex<usize>>, value: usize) {
    let mut guard = counter.lock().await;
    *guard += value;
}

async fn test_with_simple_actor_using_mutex(round: usize) {
    println!("[test_with_simple_actor_using_mutex] Running ...");

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();
    let now = tokio::time::Instant::now();

    for i in 0..round {
        let counter_clone = counter.clone();
        handlers.push(tokio::spawn(async move {
            simple_actor_using_mutex(counter_clone, i).await
        }));
    }

    for h in handlers {
        h.await.unwrap();
    }

    println!(
        "[test_with_simple_actor_using_mutex] Done <{:?}>",
        now.elapsed()
    );
}

async fn test_with_simple_actor(round: usize) {
    println!("[test_with_simple_actor] Running ...");

    let (tx, rx) = tokio::sync::mpsc::channel::<SimplePayload>(round);
    tokio::spawn(simple_actor(rx));

    let mut handlers = Vec::new();
    let now = tokio::time::Instant::now();

    for i in 0..round {
        let tx_clone = tx.clone();

        handlers.push(tokio::spawn(async move {
            let (one_tx, one_rx) = oneshot::channel::<String>();
            tx_clone
                .send(SimplePayload {
                    value: i,
                    reply: one_tx,
                })
                .await
                .unwrap();

            match one_rx.await {
                Ok(_) => {}
                Err(_) => println!("[Main] Can't recv ack from actor"),
            }
        }));
    }

    for h in handlers {
        h.await.unwrap();
    }

    println!("[test_with_simple_actor] Done <{:?}>", now.elapsed());
}
/*
async fn test_with_simple_actor() {
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
*/

#[tokio::main]
async fn main() {
    let round = 10000000;
    test_with_simple_actor_using_mutex(round).await;
    test_with_simple_actor(round).await;
}
