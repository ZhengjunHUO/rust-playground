use std::{thread, time};

async fn do_something(input: Option<u32>) -> Option<u32> {
    println!("[DEBUG] Calculating {:?}", input);
    let num = input?;
    thread::sleep(time::Duration::from_millis(3000));
    return Some((num / 2) * (num / 2));
}

#[tokio::main]
async fn main() {
    let mut done = false;
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    let task = do_something(None);
    tokio::pin!(task);

    tokio::spawn(async move {
        let _ = tx.send(26).await;
        let _ = tx.send(49).await;
        //thread::sleep(time::Duration::from_millis(100));
        let _ = tx.send(18).await;
    });

    loop {
        tokio::select! {
            result = &mut task, if !done  => {
                done = true;

                if let Some(v) = result {
                    println!("Result: {}", v);
                    return;
                }
            },
            Some(val) = rx.recv() => {
                println!("[DEBUG] Recv from rx: {}", val);
                if val % 2 == 0 {
                    task.set(do_something(Some(val)));
                    done = false;
                }
            }
        }
    }
}
