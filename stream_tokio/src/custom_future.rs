use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Instant;

pub(crate) struct DelayFuture(pub(crate) Instant);

impl Future for DelayFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if Instant::now() >= self.0 {
            return Poll::Ready("From delay future".to_owned());
        }

        println!("[DEBUG] Not ready yet !");
        let waker = cx.waker().clone();
        let timestamp = self.0;

        thread::spawn(move || {
            let now = Instant::now();

            if now < timestamp {
                thread::sleep(timestamp - now);
            }

            waker.wake();
        });

        Poll::Pending
    }
}
