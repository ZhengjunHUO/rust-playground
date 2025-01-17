#![feature(coroutines)]
#![feature(coroutine_trait)]

use std::{
    collections::VecDeque,
    ops::{Coroutine, CoroutineState},
    pin::Pin,
    time::{Duration, Instant},
};

struct Sleeper {
    begin: Instant,
    duree: Duration,
}

impl Sleeper {
    fn new(duree: Duration) -> Self {
        Sleeper {
            begin: Instant::now(),
            duree,
        }
    }
}

impl Coroutine<()> for Sleeper {
    type Yield = ();
    type Return = ();

    fn resume(
        self: std::pin::Pin<&mut Self>,
        _arg: (),
    ) -> std::ops::CoroutineState<Self::Yield, Self::Return> {
        if self.begin.elapsed() > self.duree {
            CoroutineState::Complete(())
        } else {
            CoroutineState::Yielded(())
        }
    }
}

type CoroutineQueue = VecDeque<Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>>;

struct Executor {
    queue: CoroutineQueue,
}

impl Executor {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn append(&mut self, coroutine: Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>) {
        self.queue.push_back(coroutine);
    }

    fn poll(&mut self) {
        println!("Current queue size: {}", self.queue.len());
        let mut coroutine = self.queue.pop_front().unwrap();
        match coroutine.as_mut().resume(()) {
            CoroutineState::Yielded(_) => self.queue.push_back(coroutine),
            CoroutineState::Complete(_) => {}
        }
    }
}

fn main() {
    let num = 5;
    let mut executor = Executor::new();
    for _ in 0..num {
        executor.append(Box::pin(Sleeper::new(Duration::from_millis(1))));
    }

    let begin = Instant::now();

    while !executor.queue.is_empty() {
        executor.poll();
    }

    println!("Done, cost {:?}", begin.elapsed());
}
