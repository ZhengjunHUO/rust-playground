use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;

pub mod executor;

pub struct MyFuture<T>(Arc<Mutex<Partage<T>>>);

// shared beteen future and the thread running the closure in fn spawn_blocking
struct Partage<T> {
    payload: Option<T>,
    // used to receive the callback passed by the executor (eg. block_on)
    waker: Option<Waker>,
}

impl<T: Send> Future for MyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut g = self.0.lock().unwrap();

        // check if future's payload is ready, return the Partage's payload
        if let Some(content) = g.payload.take() {
            return Poll::Ready(content);
        }

        // if not yet ready, save the callback waker passed by context
        g.waker = Some(cx.waker().clone());
        // tell the executor that it's not ready, need to poll again later
        Poll::Pending
    }
}

// 返回一个future
// 因为f和T都会被发送到另一个thread上，且该thread的生命周期未知
// 所以trait需要Send和'static
pub fn spawn_blocking<T, F>(f: F) -> MyFuture<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // [I] create a rdv point between future and thread
    let rdv = Arc::new(Mutex::new(Partage {
        payload: None,
        waker: None,
    }));

    // [II] spawn a thread dedicated to blocking operations (the closure f)
    thread::spawn({
        // ref counter plus 1
        let rdv_cloned = rdv.clone();
        move || {
            // returns a T (Partage)
            let rslt = f();
            let waker_or_none = {
                let mut g = rdv_cloned.lock().unwrap();
                // (1) save the result returned by closure f
                g.payload = Some(rslt);
                g.waker.take()
            };

            // (2) the future now is worth polling again, notify the executor
            if let Some(waker) = waker_or_none {
                // waker is consumed
                waker.wake();
            }
        }
    });

    // [III] return the future immediately
    MyFuture(rdv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future() {}
}
