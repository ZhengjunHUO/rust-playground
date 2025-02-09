use async_runtime::executor;
use std::task::Poll;

struct Counter {
    count: u32,
}

impl Future for Counter {
    type Output = u32;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.count += 1;
        if self.count < 5 {
            cx.waker().wake_by_ref();
            println!("[Counter_poll] Get count: {}", self.count);
            Poll::Pending
        } else {
            println!("[Counter_poll] Done");
            Poll::Ready(self.count)
        }
    }
}

fn main() {
    let mut executor = executor::Executor::new();
    let ctr1 = Counter { count: 0 };
    let ctr2 = Counter { count: 0 };
    let h1 = executor.spawn(ctr1);
    let h2 = executor.spawn(ctr2);

    std::thread::spawn(move || {
        loop {
            executor.poll();
        }
    });
    println!("Result: [{} {}]", h1.recv().unwrap(), h2.recv().unwrap());
}
