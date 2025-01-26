use log::info;
use std::{cell::RefCell, time::Duration};
use tokio_util::task::LocalPoolHandle;

thread_local! {
    pub static COUNTER: RefCell<usize> = RefCell::new(0);
}

async fn incr_counter(num: usize) -> usize {
    //std::thread::sleep(Duration::from_secs(2));
    tokio::time::sleep(Duration::from_secs(2)).await;
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        info!("[{}] counter: {}", num, *counter.borrow());
    });
    num
}

// executes all tasks on the current thread
#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    /*
    // (1) local pool has one thread in the pool: tasks will be processed sequentially; COUNTER will be 3
    //let local_pool = LocalPoolHandle::new(1);

    // (2) All three tasks started processing as soon as they were spawned
    // Each task has COUNTER == 1
    let local_pool = LocalPoolHandle::new(3);
    let foo = local_pool.spawn_pinned(|| async {
        info!("[foo] Task spawned.");
        incr_counter(1).await
    });
    let bar = local_pool.spawn_pinned(|| async {
        info!("[bar] Task spawned.");
        incr_counter(2).await
    });
    let baz = local_pool.spawn_pinned(|| async {
        info!("[baz] Task spawned.");
        incr_counter(3).await
    });
    */

    // (3) Spawn all tasks on the thread 0: same as LocalPoolHandle::new(1)
    let local_pool: LocalPoolHandle = LocalPoolHandle::new(3);
    let foo = local_pool.spawn_pinned_by_idx(
        || async {
            info!("[foo] Task spawned.");
            incr_counter(1).await
        },
        0,
    );
    let bar = local_pool.spawn_pinned_by_idx(
        || async {
            info!("[bar] Task spawned.");
            incr_counter(2).await
        },
        0,
    );
    let baz = local_pool.spawn_pinned_by_idx(
        || async {
            info!("[baz] Task spawned.");
            incr_counter(3).await
        },
        0,
    );

    let sum = async { foo.await.unwrap() + bar.await.unwrap() + baz.await.unwrap() };
    info!("Sum: {}", sum.await);
}
