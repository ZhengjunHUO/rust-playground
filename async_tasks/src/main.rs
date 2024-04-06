use indicatif::ProgressBar;
use std::cmp::min;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::sync::mpsc::{channel, Sender};

macro_rules! send_message {
    ($tx:ident, $client_id: ident, $payload: ident) => {
        if let Err(e) = $tx.send($payload).await {
            println!(
                "[WARN] ckh-{}: Send message with error returned: {}",
                $client_id, e
            );
        };
    };
}

trait RunAsync {
    type Backlog;
    type Context;
    type Payload;

    fn prepare_shared_backlog(&self) -> (Arc<Mutex<Self::Backlog>>, usize);
}

struct DemoProject {}

impl DemoProject {
    fn new() -> Self {
        DemoProject {}
    }
}

impl RunAsync for DemoProject {
    type Backlog = VecDeque<String>;
    type Context = DemoContext;
    type Payload = DemoPayload;

    fn prepare_shared_backlog(&self) -> (Arc<Mutex<Self::Backlog>>, usize) {
        let path = env::args()
            .nth(1)
            .expect("Expect a path to file containing list of tables to be dealt with.");

        let mut table_list = VecDeque::new();
        for line in read_to_string(path).unwrap().lines() {
            table_list.push_back(line.to_string())
        }
        let num_job = table_list.len();

        (Arc::new(Mutex::new(table_list)), num_job)
    }
}

struct DemoContext {
    client_id: String,
    table_list: Arc<Mutex<VecDeque<String>>>,
}

struct DemoPayload {
    message: String,
    done: bool,
}

impl DemoPayload {
    fn new(message: String, done: bool) -> Self {
        Self { message, done }
    }
}

async fn handle(ctx: DemoContext, tx: Sender<DemoPayload>) {
    let client_id = ctx.client_id;
    println!("ckh-{}: Worker start !", client_id);

    loop {
        let table;
        {
            let mut garde = ctx.table_list.lock().unwrap();
            table = garde.pop_front();
        }

        let secs = 3;

        match table {
            Some(table_name) => {
                // Notify start task
                let payload = DemoPayload::new(
                    format!(
                        "[ckh-{}] Dealing with the table {}, estimate done in {} secs",
                        client_id, table_name, secs
                    ),
                    false,
                );
                send_message!(tx, client_id, payload);

                // Do sth
                thread::sleep(time::Duration::from_secs(secs));

                // Notify end task
                let payload = DemoPayload::new(
                    format!("[ckh-{}] Done with the table {}", client_id, table_name),
                    true,
                );
                send_message!(tx, client_id, payload);
            }
            None => {
                // Notify quit
                let payload = DemoPayload::new(
                    format!("[ckh-{}] Todo list is empty, mission complete", client_id),
                    false,
                );
                send_message!(tx, client_id, payload);

                break;
            }
        }
    }
}

macro_rules! do_async_tasks {
    ($eps:ident, $num_job:ident, $tables:ident, $handle_func:ident) => {
        // Prepare mpsc channels
        let (tx, mut rx) = channel(min($num_job, 10));
        let mut senders = Vec::with_capacity($eps.len());
        (0..$eps.len() - 1).for_each(|_| senders.push(tx.clone()));
        senders.push(tx);

        // Optional: init progress bar
        let ind = ProgressBar::new($num_job as u64);

        // Dispatch jobs to workers
        let mut tasks = Vec::with_capacity($eps.len());
        for ep in $eps.into_iter() {
            let context = DemoContext {
                client_id: ep,
                table_list: $tables.clone(),
            };
            tasks.push(tokio::spawn($handle_func(context, senders.pop().unwrap())));
        }

        println!("[main] Receiving message !");
        while let Some(payload) = rx.recv().await {
            // Optional: update progress bar
            ind.println(&payload.message);
            if payload.done {
                ind.inc(1);
            }
        }

        // Optional: quit progress bar
        ind.finish_with_message("Complete");

        println!("[main] All done!");
    };
}

fn prepare_workers() -> Vec<String> {
    ["0-0", "0-1", "1-0", "1-1"]
        .iter()
        .map(|&s| s.into())
        .collect()
}

#[tokio::main]
async fn main() {
    let project = DemoProject::new();

    let (tables, num_job) = project.prepare_shared_backlog();
    let eps = prepare_workers();

    do_async_tasks!(eps, num_job, tables, handle);
}

/*
use std::future::Future;

async fn main() {
    doit(handle).await
}

async fn doit<F, Fut>(handle_func: F)
where
    F: Fn(DemoContext, Sender<DemoPayload>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tables, num_job) = prepare_shared_backlog();
    let eps = prepare_workers();

    do_async_tasks!(eps, num_job, tables, handle_func);
}
*/
