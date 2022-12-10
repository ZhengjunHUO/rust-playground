use std::{
    thread,
    sync::{mpsc, Arc, Mutex},
};

pub struct WorkerPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Task>,
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

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(s);

        for id in 0..s {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        WorkerPool { workers, tx }
    }

    pub fn schedule<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        self.tx.send(task).unwrap();
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let task = rx.lock().unwrap().recv().unwrap();
            println!("Task scheduled to worker {id}");
            task();
        });

	Worker { id, thread }
    }
}

type Task = Box<dyn FnOnce() + Send + 'static>;
