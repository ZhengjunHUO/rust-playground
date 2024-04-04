use indicatif::ProgressBar;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::sync::mpsc::{channel, Sender};

async fn handle(ctx: Context, tx: Sender<Payload>) {
    let client_id = ctx.client_id;
    println!("ckh-{}: Worker start !", client_id);
    loop {
        let table;

        {
            let mut garde = ctx.table_list.lock().unwrap();
            table = garde.pop_front();
        }

        match table {
            Some(table_name) => {
                let secs = 3;
                if let Err(e) = tx
                    .send(Payload {
                        message: format!(
                            "[ckh-{}] Dealing with the table {}, estimate done in {} secs",
                            client_id, table_name, secs
                        ),
                    })
                    .await
                {
                    println!(
                        "[WARN] ckh-{}: Send message with error returned: {}",
                        client_id, e
                    );
                };
                /*
                println!(
                    "[ckh-{}] Dealing with the table {}, estimate done in {} secs",
                    client_id, table_name, secs
                );
                */
                thread::sleep(time::Duration::from_secs(secs));
                if let Err(e) = tx
                    .send(Payload {
                        message: format!("[ckh-{}] Done with the table {}", client_id, table_name),
                    })
                    .await
                {
                    println!(
                        "[WARN] ckh-{}: Send message with error returned: {}",
                        client_id, e
                    );
                };
                //println!("[ckh-{}] Done with the table {}", client_id, table_name);
            }
            None => {
                if let Err(e) = tx
                    .send(Payload {
                        message: format!(
                            "[ckh-{}] Todo list is empty, mission complete",
                            client_id
                        ),
                    })
                    .await
                {
                    println!(
                        "[WARN] ckh-{}: Send message with error returned: {}",
                        client_id, e
                    );
                };
                //println!("[ckh-{}] Todo list is empty, mission complete", client_id);
                break;
            }
        }
    }
}

type TableList = Arc<Mutex<VecDeque<String>>>;

struct Context {
    client_id: String,
    table_list: TableList,
}

struct Payload {
    message: String,
}

/*
struct Async_Sched {

}

impl Async_Sched {

}
*/

#[tokio::main]
async fn main() {
    doit(handle).await
}

async fn doit<F, Fut>(handle_func: F)
where
    F: Fn(Context, Sender<Payload>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let path = env::args()
        .nth(1)
        .expect("Expect a path to file containing list of tables to be dealt with.");

    let mut table_list = VecDeque::new();
    for line in read_to_string(path).unwrap().lines() {
        table_list.push_back(line.to_string())
    }
    let num_job = table_list.len() as u64;

    let tables = Arc::new(Mutex::new(table_list));

    let eps: Vec<String> = ["0-0", "0-1", "1-0", "1-1"]
        .iter()
        .map(|&s| s.into())
        .collect();
    let mut tasks = Vec::with_capacity(eps.len());

    let (tx, mut rx) = channel(8);
    let mut senders = Vec::with_capacity(eps.len());
    for _ in 0..eps.len() - 1 {
        senders.push(tx.clone());
    }
    senders.push(tx);

    let ind = ProgressBar::new(num_job);
    for ep in eps.into_iter() {
        let list = tables.clone();
        let context = Context {
            client_id: ep,
            table_list: list,
        };
        tasks.push(tokio::spawn(handle_func(context, senders.pop().unwrap())));
    }

    /*
    let mut rslt = Vec::with_capacity(tasks.len());
    for task in tasks {
        rslt.push(task.await.unwrap());
    }
    */

    println!("[main] Receiving message !");
    while let Some(payload) = rx.recv().await {
        ind.println(&payload.message);
        if payload.message.contains("Done") {
            ind.inc(1);
        }
    }
    ind.finish_with_message("Complete");

    //println!("{:?}", rslt);
    println!("[main] All done!");
}
