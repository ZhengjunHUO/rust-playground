use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct WorkerPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Task>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
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

        WorkerPool { workers, tx: Some(tx) }
    }

    pub fn schedule<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        // send the handler(with conn) through the channel to the workerpool
        self.tx.as_ref().unwrap().send(task).unwrap();
    }
}

// hook called when program exit
impl Drop for WorkerPool {
    fn drop(&mut self) {
        // close the sender first to close the channel
        // if not the join() method below will not return
        // because of the workers loop forever
        drop(self.tx.take());

        for w in &mut self.workers {
            // move occurs
            // take() on "Option" to move value out of "Some" variant and leave a "None" variant
            // clean up done
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
            println!("[Worker {}] Stopped.", w.id);
        }
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        // Worker's thread take a closure with infinite loop, listening on the receiver
        let thread = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();
            match msg {
                Ok(task) => {
                    println!("[Worker {}] Receive task.", id);
                    task();
                }
                Err(_) => {
                    println!("[Worker {}] Gracefully shutting down ...", id);
                    break;
                }
            }
        });

	Worker { id, thread: Some(thread) }
    }
}
