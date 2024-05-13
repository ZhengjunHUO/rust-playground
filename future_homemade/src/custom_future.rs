use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::spawn;
use std::time::Instant;

struct Inner<T> {
    data: Option<T>,
    callback: Option<Waker>,
}

pub(crate) struct CustomFuture<T>(Arc<Mutex<Inner<T>>>);

impl<T> Future for CustomFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();

        if let Some(data) = guard.data.take() {
            return Poll::Ready(data);
        }

        guard.callback = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub(crate) fn init_future<C, T>(closure: C) -> CustomFuture<T>
where
    C: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let fut = Arc::new(Mutex::new(Inner {
        data: None,
        callback: None,
    }));

    let my_fut = fut.clone();
    spawn(move || {
        let data = closure();
        let mut guard = my_fut.lock().unwrap();
        guard.data = Some(data);
        if let Some(waker) = guard.callback.take() {
            waker.wake();
        }
    });

    CustomFuture(fut)
}

pub(crate) struct DelayFuture(pub(crate) Instant);

impl Future for DelayFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if Instant::now() >= self.0 {
            return Poll::Ready("From delay future".to_owned());
        }

        println!("Not ready yet !");
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
