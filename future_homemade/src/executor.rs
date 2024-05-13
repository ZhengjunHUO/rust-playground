use crossbeam::sync::Parker;
use futures_lite::pin;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use waker_fn::waker_fn;

pub(crate) fn block_on<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    let p = Parker::new();
    let u = p.unparker().clone();

    let waker = waker_fn(move || u.unpark());
    let mut cx = Context::from_waker(&waker);

    pin!(fut);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(data) => return data,
            Poll::Pending => p.park(),
        }
    }
}

pub(crate) struct MiniTokio {
    tasks: VecDeque<Pin<Box<dyn Future<Output = String>>>>,
}

impl MiniTokio {
    pub(crate) fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    pub(crate) fn spawn<F>(&mut self, f: F)
    where
        F: Future<Output = String> + 'static,
    {
        self.tasks.push_back(Box::pin(f));
    }

    pub(crate) fn exec(&mut self) {
        let p = Parker::new();
        let u = p.unparker().clone();

        let waker = waker_fn(move || u.unpark());
        let mut cx = Context::from_waker(&waker);

        while let Some(mut f) = self.tasks.pop_front() {
            match f.as_mut().poll(&mut cx) {
                Poll::Ready(data) => println!("{data}"),
                Poll::Pending => self.tasks.push_back(f),
            }
        }
    }
}
