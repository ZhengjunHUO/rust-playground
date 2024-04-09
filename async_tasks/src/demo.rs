use crate::traits::{IsDone, RunAsync};
use indicatif::ProgressBar;
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

pub struct DemoProject {
    progress_bar: ProgressBar,
    backlog_size: usize,
}

impl DemoProject {
    pub fn new() -> Self {
        DemoProject {
            progress_bar: ProgressBar::new(1),
            backlog_size: 0,
        }
    }
}

impl RunAsync for DemoProject {
    type Backlog = VecDeque<String>;
    type Context = DemoContext;
    type Payload = DemoPayload;
    type Worker = usize;

    fn prepare_shared_backlog(&mut self) -> (Arc<Mutex<Self::Backlog>>, usize) {
        let path = env::args()
            .nth(1)
            .expect("Expect a path to file containing list of tables to be dealt with.");

        let mut task_list = VecDeque::new();
        for line in read_to_string(path).unwrap().lines() {
            task_list.push_back(line.to_string())
        }
        let num_job = task_list.len();

        self.backlog_size = num_job;

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

            let secs = 2 * (client_id + 1) as u64;

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
        (0..4).collect()
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

    fn pre_dispatch_hook(&self) {
        self.progress_bar.set_length(self.backlog_size as u64);
    }

    fn in_dispatch_hook(&self, payload: Self::Payload) {
        self.progress_bar.println(format!("{payload:?}"));
        if payload.is_done() {
            self.progress_bar.inc(1);
        }
    }

    fn post_dispatch_hook(&self) {
        self.progress_bar.finish_with_message("Complete");
    }
}

pub struct DemoContext {
    client_id: usize,
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
