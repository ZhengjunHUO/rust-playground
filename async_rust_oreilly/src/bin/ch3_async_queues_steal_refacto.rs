use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use log::info;
use std::sync::LazyLock;
use std::task::Poll;
use std::time::Duration;
use std::{future::Future, panic::catch_unwind};

macro_rules! task_spawn {
    ($future:expr) => {
        task_spawn!($future, FuturePrio::Low)
    };
    ($future:expr, $prio:expr) => {
        spawn_task($future, $prio)
    };
}

macro_rules! join {
    ($($future:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.push(futures_lite::future::block_on($future));
            )*
            result
        }
    };
}

macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.push(catch_unwind(|| futures_lite::future::block_on($future)));
            )*
            result
        }
    };
}

// basic async runtime: task-spawning func, convert future into task, put the task on the task queue
fn spawn_task<F, T>(future: F, prio: FuturePrio) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(|| flume::unbounded::<Runnable>());
    static CHANNEL_PREMIUM: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(|| flume::unbounded::<Runnable>());

    // 普通队列只处理普通task
    static QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        let rx_clone = CHANNEL.1.clone();
        std::thread::spawn(move || {
            while let Ok(runnable) = rx_clone.recv() {
                info!("[RX standard thread] Accept runnable");
                let _ = catch_unwind(|| runnable.run());
            }
        });
        CHANNEL.0.clone()
    });

    // 如果没有PREMIUM task，但有很多普通task堆积，premium队列会把普通的task偷走执行提升吞吐量
    static QUEUE_PREMIUM: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        for i in 0..2 {
            let rx_premium_clone = CHANNEL_PREMIUM.1.clone();
            let rx_clone = CHANNEL.1.clone();

            std::thread::spawn(move || loop {
                match rx_premium_clone.try_recv() {
                    Ok(runnable) => {
                        info!("[RX premium thread {i}] Accept runnable");
                        let _ = catch_unwind(|| runnable.run());
                    }
                    Err(_) => {
                        info!("[RX premium thread {i}] No runnable in channel, check standard chan ...");
                        match rx_clone.try_recv() {
                            Ok(runnable) => {
                                info!(
                                    "[RX premium thread {i}] Steal runnable from standard channel"
                                );
                                let _ = catch_unwind(|| runnable.run());
                            }
                            Err(_) => {
                                info!("[RX premium thread {i}] Nothing to do, sleep ...");
                                std::thread::sleep(Duration::from_millis(500));
                            }
                        }
                    }
                }
            });
        }
        CHANNEL_PREMIUM.0.clone()
    });

    let schedule = match prio {
        FuturePrio::High => |runnable| QUEUE_PREMIUM.send(runnable).unwrap(),
        FuturePrio::Low => |runnable| QUEUE.send(runnable).unwrap(),
    };

    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    return task;
}

#[derive(Debug, Clone, Copy)]
enum FuturePrio {
    High,
    Low,
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
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

    // 创建很多普通任务
    let mut counters = vec![];
    for _ in 0..5 {
        counters.push(Counter::new());
    }
    // 创建一个高级任务（如果没有这个任务来激活高级worker（lazy），高级worker就不会启动）
    let c_prem_1 = Counter::new();
    let c_prem_2 = Counter::new();

    let mut tasks = vec![];
    for c in counters {
        tasks.push(task_spawn!(c));
    }
    let t_prem_1 = task_spawn!(c_prem_1, FuturePrio::High);
    let t_prem_2 = task_spawn!(c_prem_2, FuturePrio::High);

    let task_fns = task_spawn!(async {
        fn_async().await;
        fn_async().await;
        fn_async().await;
        fn_async().await;
        fn_async().await;
    });

    info!("[main] block on tasks");
    for t in tasks {
        futures_lite::future::block_on(t);
    }
    // 不同的Output的future不能在同一个join!中
    try_join!(t_prem_1, t_prem_2);
    join!(task_fns);
    info!("[main] Done");
}
