use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;

pub struct MyFuture<T>(Arc<Mutex<Partage<T>>>);

struct Partage<T> {
    payload: Option<T>,
    waker: Option<Waker>,
}

impl<T: Send> Future for MyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut g = self.0.lock().unwrap();
        if let Some(content) = g.payload.take() {
            return Poll::Ready(content);
        }

        g.waker = Some(cx.waker().clone());
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
    let rdv = Arc::new(Mutex::new(Partage {
        payload: None,
        waker: None,
    }));

    thread::spawn({
        // ref counter plus 1
        let rdv_cloned = rdv.clone();
        move || {
            // returns a T (Partage)
            let rslt = f();
            let waker_or_none = {
                let mut g = rdv_cloned.lock().unwrap();
                g.payload = Some(rslt);
                g.waker.take()
            };

            if let Some(waker) = waker_or_none {
                waker.wake();
            }
        }
    });

    MyFuture(rdv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future() {}
}
