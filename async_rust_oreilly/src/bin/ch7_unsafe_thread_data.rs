use std::{cell::UnsafeCell, collections::HashMap, time::Duration};

use tokio_util::task::LocalPoolHandle;

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

fn print_all() {
    COUNTER.with(|ctr| {
        let counter = unsafe { &*ctr.get() };
        println!("{:?}", counter);
    });
}

#[tokio::main]
async fn main() {
    let pool = LocalPoolHandle::new(1);
    let seq: Vec<_> = (1..6).cycle().take(5000).collect();

    let mut handlers = Vec::new();
    for i in seq {
        handlers.push(pool.spawn_pinned(move || async move {
            incr_counter(i).await;
            incr_counter(i).await
        }));
    }
    for h in handlers {
        h.await.unwrap();
    }

    // 直接调用print_all()返回空集合，必须在thread0的scope中调用
    pool.spawn_pinned(|| async {
        print_all();
    })
    .await
    .unwrap();
}
