use crate::traits::{IsDone, RunAsync};
use std::collections::VecDeque;
use std::env;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use tokio::sync::mpsc::Sender;

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

pub struct DemoProject {}

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
                        format!(
                            "[Worker {}] Todo list is empty, mission complete",
                            client_id
                        ),
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

pub struct DemoContext {
    client_id: String,
    task_list: Arc<Mutex<VecDeque<String>>>,
}

pub struct DemoPayload {
    message: String,
    done: bool,
}

impl DemoPayload {
    fn new(message: String, done: bool) -> Self {
        Self { message, done }
    }
}

impl Debug for DemoPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IsDone for DemoPayload {
    fn is_done(&self) -> bool {
        self.done
    }
}
