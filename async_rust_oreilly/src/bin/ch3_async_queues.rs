use async_task::{Runnable, Task};
use log::info;
use std::sync::LazyLock;
use std::task::Poll;
use std::time::Duration;
use std::{future::Future, panic::catch_unwind};

fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static + FuturePrioShow,
    T: Send + 'static,
{
    static QUEUE: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        std::thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                info!("[RX normal thread] Accept runnable");
                let _ = catch_unwind(|| runnable.run());
            }
        });
        tx
    });

    static QUEUE_PREMIUM: LazyLock<flume::Sender<Runnable>> = LazyLock::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        for i in 0..2 {
            let rx_clone = rx.clone();
            std::thread::spawn(move || {
                while let Ok(runnable) = rx_clone.recv() {
                    info!("[RX premium thread {i}] Accept runnable");
                    let _ = catch_unwind(|| runnable.run());
                }
            });
        }
        tx
    });

    let schedule = match future.show_prio() {
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

trait FuturePrioShow: Future {
    fn show_prio(&self) -> FuturePrio;
}

struct Counter {
    count: u32,
    prio: FuturePrio,
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

impl FuturePrioShow for Counter {
    fn show_prio(&self) -> FuturePrio {
        self.prio
    }
}

fn main() {
    env_logger::init();

    let ctr = Counter {
        count: 0,
        prio: FuturePrio::High,
    };
    let another_ctr = Counter {
        count: 0,
        prio: FuturePrio::Low,
    };
    // 高优先级通道有两个worker，所以在处理单个高优先级的task时，每次被唤醒会round robin到其中一个worker上执行
    let task_ctr = spawn_task(ctr);
    let task_another_ctr = spawn_task(another_ctr);

    info!("[main] Before block on task_ctr");
    futures_lite::future::block_on(task_ctr);
    info!("[main] Before block on task_another_ctr");
    futures_lite::future::block_on(task_another_ctr);
    info!("[main] Done");
}
