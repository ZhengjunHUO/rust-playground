use std::fmt::Debug;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

pub trait IsDone {
    fn is_done(&self) -> bool;
}

pub trait RunAsync {
    type Backlog;
    type Context;
    type Payload: Debug + IsDone;
    type Worker;

    fn prepare_shared_backlog() -> (Arc<Mutex<Self::Backlog>>, usize);
    fn handle(
        ctx: Self::Context,
        tx: Sender<Self::Payload>,
    ) -> impl Future<Output = ()> + Send + 'static;
    fn prepare_workers() -> Vec<Self::Worker>;
    fn prepare_context(worker: Self::Worker, task_list: Arc<Mutex<Self::Backlog>>)
        -> Self::Context;
}
