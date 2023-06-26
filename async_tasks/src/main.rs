use rand::Rng;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::{thread, time};

async fn handle(client_id: String, table_list: TableList) -> String {
    loop {
        let table;

        {
            let mut garde = table_list.lock().unwrap();
            table = garde.pop_front();
        }

        match table {
            Some(table_name) => {
                let mut rng = rand::thread_rng();
                let secs = rng.gen_range(3..=20);
                println!(
                    "[ckh-{}] Dealing with the table {}, estimate done in {} secs",
                    client_id, table_name, secs
                );
                thread::sleep(time::Duration::from_secs(secs));
                println!("[ckh-{}] Done with the table {}", client_id, table_name);
            }
            None => {
                println!("[ckh-{}] Todo list is empty, mission complete", client_id);
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

    let tables = Arc::new(Mutex::new(table_list));

    let eps: Vec<String> = ["0-0", "0-1", "1-0", "1-1"]
        .iter()
        .map(|&s| s.into())
        .collect();
    let mut tasks = Vec::with_capacity(eps.len());
    for ep in eps {
        let list = tables.clone();
        tasks.push(tokio::spawn(handle(ep, list)));
    }

    let mut rslt = Vec::with_capacity(tasks.len());
    for task in tasks {
        rslt.push(task.await.unwrap());
    }

    println!("{:?}", rslt);
    println!("All done!");
}
