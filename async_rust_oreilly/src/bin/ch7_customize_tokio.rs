use std::{future::Future, sync::LazyLock, time::Duration};
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

static CUSTOM_RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .thread_name("custom runtime foo")
        .worker_threads(3)
        .max_blocking_threads(1)
        .thread_keep_alive(Duration::from_secs(60))
        .thread_stack_size(5 * 1024 * 1024)
        .global_queue_interval(61)
        .on_thread_start(|| println!("[Rt foo]: thread start ..."))
        .on_thread_stop(|| println!("[Rt foo]: thread stop ..."))
        .on_thread_park(|| println!("[Rt foo]: thread park ..."))
        .on_thread_unpark(|| println!("[Rt foo]: thread park ..."))
        .enable_time()
        .build()
        .unwrap()
});

fn spawn_task<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    CUSTOM_RUNTIME.spawn(future)
}

async fn do_sth() -> usize {
    println!("[Inside do_sth] Do something ...");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("[Inside do_sth] Done.");
    88
}

// - async task could start before all the worker threads are created
// - the idle worker threads are being parked
fn main() {
    let h = spawn_task(do_sth());
    println!("[main] Task spawned.");
    println!("[main] Task done ? {}", h.is_finished());
    std::thread::sleep(Duration::from_secs(2));
    println!("[main] Task done ? {}", h.is_finished());
    println!("[main] Got result: {}", CUSTOM_RUNTIME.block_on(h).unwrap());
}
