use crate::waker::build_raw_waker;
use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{Arc, mpsc},
    task::{Context, Poll, Waker},
};

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    waker: Arc<Waker>,
}

pub struct Executor {
    pub polling: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            polling: VecDeque::new(),
        }
    }

    pub fn spawn<F, T>(&mut self, future: F) -> mpsc::Receiver<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let pinned_future = Box::pin(async move {
            let rslt = future.await;
            let _ = tx.send(rslt);
        });
        let task = Task {
            future: pinned_future,
            waker: self.create_waker(),
        };
        self.polling.push_back(task);
        rx
    }

    pub fn poll(&mut self) {
        let mut task = match self.polling.pop_front() {
            Some(task) => task,
            None => return,
        };
        let waker = task.waker.clone();
        let context = &mut Context::from_waker(&waker);

        match task.future.as_mut().poll(context) {
            Poll::Ready(()) => {}
            Poll::Pending => {
                self.polling.push_back(task);
            }
        }
    }

    pub fn create_waker(&self) -> Arc<Waker> {
        Arc::new(unsafe { Waker::from_raw(build_raw_waker()) })
    }
}
