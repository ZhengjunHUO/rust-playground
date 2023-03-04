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
