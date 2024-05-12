use crossbeam::sync::Parker;
use futures_lite::pin;
use std::future::Future;
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
