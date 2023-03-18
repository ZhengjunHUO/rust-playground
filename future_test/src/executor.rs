use crossbeam::sync::Parker;
use futures_lite::pin;
use std::future::Future;
use std::task::{Context, Poll};
use waker_fn::waker_fn;

// executor to poll a future
pub fn block_on<F: Future>(f: F) -> F::Output {
    // init blocking primitive
    let p = Parker::new();
    let u = p.unparker().clone();

    // init Waker from a closure
    let waker = waker_fn(move || u.unpark());
    // passed to future, let it to unpark (unblock)
    let mut ctx = Context::from_waker(&waker);

    // takes ownership of the future f, pin f to the stack
    // rebind f as type Pin<&mut F>
    pin!(f);

    loop {
        match f.as_mut().poll(&mut ctx) {
            Poll::Ready(rslt) => {
                println!("[DEBUG] executor polled: The future is ready.");
                return rslt;
            }
            // block until the waker is called
            Poll::Pending => {
                println!("[DEBUG] executor polled: The future is not ready yet.");
                p.park();
            }
        }
    }
}
