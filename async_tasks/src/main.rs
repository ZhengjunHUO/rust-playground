use indicatif::ProgressBar;
use std::cmp::min;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::sync::mpsc::{channel, Sender};

macro_rules! send_message {
    ($tx:ident, $client_id: ident, $payload: ident) => {
        if let Err(e) = $tx.send($payload).await {
            println!(
                "[WARN] Worker {}: Send message with error returned: {}",
                $client_id, e
            );
        };
    };
}

trait RunAsync {
    type Backlog;
    type Context;
    type Payload;
    type Worker;

    fn prepare_shared_backlog() -> (Arc<Mutex<Self::Backlog>>, usize);
    fn handle(ctx: Self::Context, tx: Sender<Self::Payload>) -> impl Future<Output = ()>;
    fn prepare_workers() -> Vec<Self::Worker>;
    fn prepare_context(
        worker: Self::Worker,
        task_list: Arc<Mutex<Self::Backlog>>,
    ) -> Self::Context;
}

struct DemoProject {}

impl RunAsync for DemoProject {
    type Backlog = VecDeque<String>;
    type Context = DemoContext;
    type Payload = DemoPayload;
    type Worker = String;

    fn prepare_shared_backlog() -> (Arc<Mutex<Self::Backlog>>, usize) {
        let path = env::args()
            .nth(1)
            .expect("Expect a path to file containing list of tables to be dealt with.");

        let mut task_list = VecDeque::new();
        for line in read_to_string(path).unwrap().lines() {
            task_list.push_back(line.to_string())
        }
        let num_job = task_list.len();

        (Arc::new(Mutex::new(task_list)), num_job)
    }

    async fn handle(ctx: DemoContext, tx: Sender<DemoPayload>) {
        let client_id = ctx.client_id;
        println!("Worker {}: Worker start !", client_id);

        loop {
            let table;
            {
                let mut garde = ctx.task_list.lock().unwrap();
                table = garde.pop_front();
            }

            let secs = 3;

            match table {
                Some(table_name) => {
                    // Notify start task
                    let payload = DemoPayload::new(
                        format!(
                            "[Worker {}] Dealing with the table {}, estimate done in {} secs",
                            client_id, table_name, secs
                        ),
                        false,
                    );
                    send_message!(tx, client_id, payload);

                    // Do sth
                    thread::sleep(time::Duration::from_secs(secs));

                    // Notify end task
                    let payload = DemoPayload::new(
                        format!("[Worker {}] Done with the table {}", client_id, table_name),
                        true,
                    );
                    send_message!(tx, client_id, payload);
                }
                None => {
                    // Notify quit
                    let payload = DemoPayload::new(
                        format!("[Worker {}] Todo list is empty, mission complete", client_id),
                        false,
                    );
                    send_message!(tx, client_id, payload);

                    break;
                }
            }
        }
    }

    fn prepare_workers() -> Vec<Self::Worker> {
        ["w0", "w1", "w2", "w3"]
            .iter()
            .map(|&s| s.into())
            .collect::<Vec<_>>()
    }

    fn prepare_context(
        worker: Self::Worker,
        task_list: Arc<Mutex<Self::Backlog>>,
    ) -> Self::Context {
        DemoContext {
            client_id: worker,
            task_list,
        }
    }
}

struct DemoContext {
    client_id: String,
    task_list: Arc<Mutex<VecDeque<String>>>,
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

macro_rules! do_async_tasks {
    ($eps:ident, $num_job:ident, $tables:ident, $handle_func:ident, $prepare_context_func:ident) => {
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
            let context = $prepare_context_func(ep, $tables.clone());
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

#[tokio::main]
async fn main() {
    let (tables, num_job) = DemoProject::prepare_shared_backlog();
    let eps = DemoProject::prepare_workers();
    let handle_func = DemoProject::handle;
    let prepare_context_func = DemoProject::prepare_context;
    do_async_tasks!(eps, num_job, tables, handle_func, prepare_context_func);
}

/*
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
