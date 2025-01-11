use async_rust_oreilly::tasks::Runtime;
use async_rust_oreilly::{join, task_spawn};
use log::info;
use std::future::Future;
use std::task::Poll;
use std::time::Duration;

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
