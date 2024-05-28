use std::{thread, time};

async fn do_something() {
    let mut counter = 0;
    loop {
        println!("{}", counter);
        counter = incr_counter(counter).await;
        tokio::task::yield_now().await;
    }
}

async fn incr_counter(counter: u32) -> u32 {
    thread::sleep(time::Duration::from_millis(1000));
    counter + 1
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    let task = do_something();
    tokio::pin!(task);

    tokio::spawn(async move {
        let _ = tx.send(26).await;
        thread::sleep(time::Duration::from_millis(2000));
        let _ = tx.send(49).await;
        thread::sleep(time::Duration::from_millis(2000));
        let _ = tx.send(88).await;
    });

    loop {
        tokio::select! {
            _ = &mut task => break,
            Some(msg) = rx.recv() => {
                if msg == 88 {
                    break;
                }
                println!("[DEBUG] Recv: {}", msg);
            }
        }
    }

    println!("[DEBUG] Done");
}
