use crate::traits::RunAsync;

pub async fn exec_async_tasks<T: RunAsync>(mut run_async: T) {
    let (tables, num_job) = run_async.prepare_shared_backlog();
    let eps = T::prepare_workers();

    // (1) Prepare mpsc channels
    let (tx, mut rx) = tokio::sync::mpsc::channel(std::cmp::min(num_job, 10));
    let mut senders = Vec::with_capacity(eps.len());
    (0..eps.len() - 1).for_each(|_| senders.push(tx.clone()));
    senders.push(tx);

    run_async.pre_dispatch_hook();

    // (2) Dispatch jobs to workers
    let mut tasks = Vec::with_capacity(eps.len());
    for ep in eps.into_iter() {
        let context = T::prepare_context(ep, tables.clone());
        tasks.push(tokio::spawn(T::handle(context, senders.pop().unwrap())));
    }

    // (3) Join all threads
    println!("[Main] Start exec ... ");
    while let Some(payload) = rx.recv().await {
        run_async.in_dispatch_hook(payload);
    }

    run_async.post_dispatch_hook();

    println!("[Main] All done!");
}
