use async_rust_oreilly::{join, task_spawn};
use async_rust_oreilly::tasks::{spawn_task, FuturePrio};
use log::info;
use std::task::Poll;
use std::time::Duration;
use std::future::Future;

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
        if self.count < 3 {
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

struct Runtime {
    std_chan_num: usize,
    premium_chan_num: usize,
}

impl Runtime {
    fn new() -> Self {
        let paral = std::thread::available_parallelism().unwrap().get();
        Runtime {
            std_chan_num: 1,
            premium_chan_num: paral - 2,
        }
    }

    fn with_std_chan_num(mut self, num: usize) -> Self {
        self.std_chan_num = num;
        self
    }

    fn with_premium_chan_num(mut self, num: usize) -> Self {
        self.premium_chan_num = num;
        self
    }

    fn run(&self) {
        std::env::set_var("STD_CHAN_NUM", self.std_chan_num.to_string());
        std::env::set_var("PREMIUM_CHAN_NUM", self.premium_chan_num.to_string());

        // 激活lazy的workers，在正式接受任务前进入待命模式
        join!(
            task_spawn!(async {}, FuturePrio::High),
            task_spawn!(async {}, FuturePrio::Low)
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct Daemon;

impl Future for Daemon {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        info!("Daemon is running ...");
        std::thread::sleep(Duration::from_secs(1));
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {
    env_logger::init();
    // 设定两个队列的长度，并激活队列
    Runtime::new()
        .with_std_chan_num(2)
        .with_premium_chan_num(3)
        .run();

    // 把任务放到后台，生命周期贯穿整个程序
    task_spawn!(Daemon {}).detach();

    // 创建很多普通任务
    let mut counters = vec![];
    for _ in 0..8 {
        counters.push(Counter::new());
    }
    // 创建高级任务（没有引入Runtime前，如果没有这个任务来激活高级worker（lazy），高级worker就不会启动）
    //let c_prem_1 = Counter::new();
    //let c_prem_2 = Counter::new();

    let mut tasks = vec![];
    for c in counters {
        tasks.push(task_spawn!(c));
    }
    //let t_prem_1 = task_spawn!(c_prem_1, FuturePrio::High);
    //let t_prem_2 = task_spawn!(c_prem_2, FuturePrio::High);

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
    //try_join!(t_prem_1, t_prem_2);
    join!(task_fns);
    info!("[main] Done");
}
