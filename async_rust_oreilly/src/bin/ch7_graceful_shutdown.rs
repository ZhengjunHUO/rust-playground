use std::{cell::UnsafeCell, collections::HashMap, sync::LazyLock, time::Duration};

use tokio_util::task::LocalPoolHandle;

pub async fn destroy_daemon() {
    println!("Gracefully shutdown the program ...");
    let mut count = 0;
    loop {
        tokio::signal::ctrl_c().await.unwrap();
        println!("Ctrl-c captured");
        count += 1;
        if count > 2 {
            println!("Quit");
            std::process::exit(0);
        }
    }
}

static POOL: LazyLock<LocalPoolHandle> = LazyLock::new(|| LocalPoolHandle::new(4));

thread_local! {
    pub static COUNTER: UnsafeCell<HashMap<u32, u32>> = UnsafeCell::new(HashMap::new());
}

async fn incr_counter(num: u32) {
    tokio::time::sleep(Duration::from_secs(num as u64)).await;
    COUNTER.with(|ctr| {
        let counter = unsafe { &mut *ctr.get() };
        match counter.get_mut(&num) {
            Some(value) => {
                let temp = *value + 1;
                *value = temp;
            }
            None => {
                counter.insert(num, 1);
            }
        }
    });
}

fn get_thread_local_data() -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    COUNTER.with(|ctr| {
        let counter = unsafe { &*ctr.get() };
        result = counter.clone();
    });
    result
}

async fn collect_all_counts() -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    let mut handlers = Vec::new();
    for i in 0..4 {
        handlers.push(POOL.spawn_pinned_by_idx(|| async move { get_thread_local_data() }, i));
    }
    for h in handlers {
        let thread_counter = h.await.unwrap_or_default();
        for (idx, count) in thread_counter {
            *result.entry(idx).or_insert(0) += count;
        }
    }
    result
}

#[tokio::main]
async fn main() {
    //tokio::spawn(destroy_daemon());
    tokio::spawn(async {
        println!("Counting ...");
        let seq: Vec<_> = (1..6).cycle().take(5000).collect();

        let mut handlers = Vec::new();
        for i in seq {
            handlers.push(POOL.spawn_pinned(move || async move {
                incr_counter(i).await;
                incr_counter(i).await
            }));
        }
        for h in handlers {
            h.await.unwrap();
        }
        println!("Done!")
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Ctrl-c captured, shuting down gracefully ...");
    let result = collect_all_counts().await;
    println!("Counting result: {:?}", result);
}
