use crate::custom_future::DelayFuture;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio_stream::Stream;

pub(crate) struct CustomStream {
    count: usize,
    timer: DelayFuture,
}

impl CustomStream {
    pub(crate) fn new() -> Self {
        Self {
            count: 5,
            timer: DelayFuture(Instant::now()),
        }
    }
}

impl Stream for CustomStream {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if self.count == 0 {
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.timer).poll(cx) {
            Poll::Ready(_) => {
                let next = self.timer.0 + Duration::from_millis(100);
                self.timer = DelayFuture(next);
                self.count -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
