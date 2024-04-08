#[macro_export]
macro_rules! do_async_tasks {
    ($eps:ident, $num_job:ident, $tables:ident, $handle_func:ident, $prepare_context_func:ident) => {
        use $crate::traits::IsDone;
        // Prepare mpsc channels
        let (tx, mut rx) = tokio::sync::mpsc::channel(std::cmp::min($num_job, 10));
        let mut senders = Vec::with_capacity($eps.len());
        (0..$eps.len() - 1).for_each(|_| senders.push(tx.clone()));
        senders.push(tx);

        // Optional: init progress bar
        let ind = indicatif::ProgressBar::new($num_job as u64);

        // Dispatch jobs to workers
        let mut tasks = Vec::with_capacity($eps.len());
        for ep in $eps.into_iter() {
            let context = $prepare_context_func(ep, $tables.clone());
            tasks.push(tokio::spawn($handle_func(context, senders.pop().unwrap())));
        }

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
    };
}
