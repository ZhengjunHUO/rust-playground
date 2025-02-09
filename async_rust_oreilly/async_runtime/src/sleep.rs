use std::{
    task::Poll,
    time::{Duration, Instant},
};

pub struct Sleep {
    until: Instant,
}

impl Sleep {
    pub fn new(d: Duration) -> Self {
        Self {
            until: Instant::now() + d,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let now = Instant::now();
        if now < self.until {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
