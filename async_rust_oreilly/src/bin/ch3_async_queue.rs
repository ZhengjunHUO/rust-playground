use std::{future::Future, panic::catch_unwind};
use std::sync::LazyLock;
use async_task::{Runnable, Task};

// task-spawning func: convert future into task, put the task on the task queue
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    // task queue, transmitting end of a channel
    static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        std::thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                println!("[RX thread] Accept runnable");
                let _ = catch_unwind(|| runnable.run());
            }
        });
        tx
    });
    
    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    // The returned Runnable is used to poll the future, and the Task is used to await its output.
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    println!("[spawn_task] Runnable scheduled, task queue size: {}", QUEUE.len());
    return task;
}

fn main() {}