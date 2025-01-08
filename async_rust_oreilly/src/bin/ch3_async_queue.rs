use async_task::{Runnable, Task};
use log::info;
use std::sync::LazyLock;
use std::task::Poll;
use std::time::Duration;
use std::{future::Future, panic::catch_unwind};

// basic async runtime: task-spawning func, convert future into task, put the task on the task queue
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    // task queue, transmitting end of a channel
    static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        /*
        // single worker; 独立的thread来处理runnable, 会被task中的std::thread::sleep中断
        std::thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                info!("[RX thread] Accept runnable");
                let _ = catch_unwind(|| runnable.run());
            }
        });
        */

        // multiple workers; msg不是以广播形式而是被分配给各rx
        for i in 0..3 {
            let rx_clone = rx.clone();
            std::thread::spawn(move || {
                while let Ok(runnable) = rx_clone.recv() {
                    info!("[RX thread {i}] Accept runnable");
                    let _ = catch_unwind(|| runnable.run());
                }
            });
        }
        tx
    });

    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    // The returned Runnable is used to poll the future, and the Task is used to await its output.
    // 本自定async runtime的核心为async_task::spawn
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    info!(
        "[spawn_task] Runnable scheduled, task queue size: {}",
        QUEUE.len()
    );
    return task;
}

struct Counter {
    count: u32,
}

impl Future for Counter {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.count += 1;
        info!("[Counter_poll] Get count: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

async fn fn_async() {
    info!("Enter async func");
    std::thread::sleep(Duration::from_secs(1));
    info!("Quit async func");
}

fn main() {
    env_logger::init();

    let ctr = Counter { count: 0 };
    let another_ctr = Counter { count: 0 };
    // future被送到一个持有rx的独立进程中，poll被激活
    let task_ctr = spawn_task(ctr);
    let task_another_ctr = spawn_task(another_ctr);
    let task_fns = spawn_task(async {
        fn_async().await;
        fn_async().await;
        fn_async().await;
        fn_async().await;
        fn_async().await;
    });

    // 对于task的运作无影响
    info!("[main] Enter sleep");
    std::thread::sleep(Duration::from_secs(3));
    info!("[main] Quit sleep");

    // block_on作用仅为防止主进程过早退出
    futures_lite::future::block_on(task_ctr);
    info!("[main] After block on task_ctr");
    futures_lite::future::block_on(task_another_ctr);
    info!("[main] After block on task_another_ctr");
    futures_lite::future::block_on(task_fns);
    info!("[main] After block on task_fns");
}
