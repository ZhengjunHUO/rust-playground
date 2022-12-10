use std::thread;

pub struct WorkerPool {
//    workers: Vec<
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerPool {
    /// Create a new WorkerPool, whose size should be a positive number
    ///
    /// # Panics
    ///
    /// The `new` func will panic if s is zero
    pub fn new(s: usize) -> WorkerPool {
        assert!(s > 0);

        let mut workers = Vec::with_capacity(s);

        for id in 0..s {
            workers.push(Worker::new(id));
        }

        WorkerPool { workers }
    }

    pub fn schedule<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

	Worker { id, thread }
    }
}
