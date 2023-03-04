pub mod mpmc {
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::sync::{Arc, Mutex};

    pub struct SharedReceiver<T>(pub Arc<Mutex<Receiver<T>>>);
    impl<T> Iterator for SharedReceiver<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let g = self.0.lock().unwrap();
            g.recv().ok()
        }
    }

    pub fn channel<T>() -> (Sender<T>, SharedReceiver<T>) {
        // Get tx, rx from mpsc
        let (tx, rx) = mpsc::channel();
        // Wrap rx with Mutex and Arc
        (tx, SharedReceiver(Arc::new(Mutex::new(rx))))
    }
}

pub mod paral {
    use std::sync::mpsc;
    use std::thread;

    pub trait OffThread: Iterator {
        // turn self iterator to a iterator in a thread
        fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
    }

    impl<T> OffThread for T
    where
        T: Iterator + Send + 'static,
        T::Item: Send + 'static,
    {
        fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
            let (tx, rx) = mpsc::sync_channel(4096);

            thread::spawn(move || {
                for i in self {
                    if tx.send(i).is_err() {
                        break;
                    }
                }
            });

            // Receiver implements IntoIterator trait
            rx.into_iter()
        }
    }
}
