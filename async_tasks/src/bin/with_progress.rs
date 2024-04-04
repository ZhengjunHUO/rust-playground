use indicatif::ProgressBar;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::sync::mpsc::{channel, Sender};

async fn handle(idx: u8, client_id: String, table_list: TableList, tx: Sender<String>) -> String {
    println!("ckh-{}: Worker start !", client_id);
    loop {
        let table;

        {
            let mut garde = table_list.lock().unwrap();
            table = garde.pop_front();
        }

        let seed = [
            1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, idx,
        ];
        let mut rng = StdRng::from_seed(seed);

        match table {
            Some(table_name) => {
                let secs = rng.gen_range(3..=8);
                if let Err(e) = tx
                    .send(format!(
                        "[ckh-{}] Dealing with the table {}, estimate done in {} secs",
                        client_id, table_name, secs
                    ))
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
                    .send(format!(
                        "[ckh-{}] Done with the table {}",
                        client_id, table_name
                    ))
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
                    .send(format!(
                        "[ckh-{}] Todo list is empty, mission complete",
                        client_id
                    ))
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
    "Done".to_string()
}

type TableList = Arc<Mutex<VecDeque<String>>>;

#[tokio::main]
async fn main() {
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
    for (i, ep) in eps.into_iter().enumerate() {
        let list = tables.clone();
        tasks.push(tokio::spawn(handle(
            i as u8,
            ep,
            list,
            senders.pop().unwrap(),
        )));
    }

    /*
    let mut rslt = Vec::with_capacity(tasks.len());
    for task in tasks {
        rslt.push(task.await.unwrap());
    }
    */

    println!("[main] Receiving message !");
    while let Some(msg) = rx.recv().await {
        ind.println(&msg);
        if msg.contains("Done") {
            ind.inc(1);
        }
    }
    ind.finish_with_message("Complete");

    //println!("{:?}", rslt);
    println!("[main] All done!");
}
