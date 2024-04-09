use crate::traits::{IsDone, RunAsync};

pub async fn exec_async_tasks<T: RunAsync>(run_async: T) {
    let (tables, num_job) = T::prepare_shared_backlog();
    let eps = T::prepare_workers();

    // (1) Prepare mpsc channels
    let (tx, mut rx) = tokio::sync::mpsc::channel(std::cmp::min(num_job, 10));
    let mut senders = Vec::with_capacity(eps.len());
    (0..eps.len() - 1).for_each(|_| senders.push(tx.clone()));
    senders.push(tx);

    // Optional: init progress bar
    let ind = indicatif::ProgressBar::new(num_job as u64);

    // (2) Dispatch jobs to workers
    let mut tasks = Vec::with_capacity(eps.len());
    for ep in eps.into_iter() {
        let context = T::prepare_context(ep, tables.clone());
        tasks.push(tokio::spawn(T::handle(context, senders.pop().unwrap())));
    }

    // (3) Join all threads
    println!("[main] Receiving message !");
    while let Some(payload) = rx.recv().await {
        // Optional: update progress bar
        ind.println(format!("{payload:?}"));
        if payload.is_done() {
            ind.inc(1);
        }
    }

    // Optional: quit progress bar
    ind.finish_with_message("Complete");

    println!("[main] All done!");
}
