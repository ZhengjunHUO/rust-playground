use crate::{join, task_spawn};
use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use log::info;
use std::sync::LazyLock;
use std::time::Duration;
use std::{future::Future, panic::catch_unwind};

// basic async runtime: task-spawning func, convert future into task, put the task on the task queue
pub fn spawn_task<F, T>(future: F, prio: FuturePrio) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static CHANNEL: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(flume::unbounded::<Runnable>);
    static CHANNEL_PREMIUM: LazyLock<(Sender<Runnable>, Receiver<Runnable>)> =
        LazyLock::new(flume::unbounded::<Runnable>);

    // 普通队列只处理普通task
    static QUEUE: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        let num = std::env::var("STD_CHAN_NUM")
            .unwrap_or(String::from("1"))
            .parse::<usize>()
            .unwrap();
        for i in 0..num {
            let rx_clone = CHANNEL.1.clone();
            std::thread::spawn(move || {
                while let Ok(runnable) = rx_clone.recv() {
                    info!("[RX standard thread {i}] Accept runnable");
                    let _ = catch_unwind(|| runnable.run());
                }
            });
        }
        CHANNEL.0.clone()
    });

    // 如果没有PREMIUM task，但有很多普通task堆积，premium队列会把普通的task偷走执行提升吞吐量
    static QUEUE_PREMIUM: LazyLock<Sender<Runnable>> = LazyLock::new(|| {
        let num = std::env::var("PREMIUM_CHAN_NUM")
            .unwrap_or(String::from("1"))
            .parse::<usize>()
            .unwrap();
        for i in 0..num {
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
    task
}

#[derive(Debug, Clone, Copy)]
pub enum FuturePrio {
    High,
    Low,
}

pub struct Runtime {
    std_chan_num: usize,
    premium_chan_num: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let paral = std::thread::available_parallelism().unwrap().get();
        Runtime {
            std_chan_num: 1,
            premium_chan_num: paral - 2,
        }
    }

    pub fn with_std_chan_num(mut self, num: usize) -> Self {
        self.std_chan_num = num;
        self
    }

    pub fn with_premium_chan_num(mut self, num: usize) -> Self {
        self.premium_chan_num = num;
        self
    }

    pub fn run(&self) {
        std::env::set_var("STD_CHAN_NUM", self.std_chan_num.to_string());
        std::env::set_var("PREMIUM_CHAN_NUM", self.premium_chan_num.to_string());

        // 激活lazy的workers，在正式接受任务前进入待命模式
        join!(
            task_spawn!(async {}, FuturePrio::High),
            task_spawn!(async {}, FuturePrio::Low)
        );
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
