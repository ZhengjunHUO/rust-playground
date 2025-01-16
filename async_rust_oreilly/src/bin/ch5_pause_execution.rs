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

fn main() {
    let num = 5;
    let mut queue = VecDeque::new();
    for _ in 0..num {
        queue.push_back(Sleeper::new(Duration::from_secs(1)));
    }

    let mut counter = 0;
    let begin = Instant::now();

    while counter < num {
        let mut sleeper = queue.pop_front().unwrap();
        match Pin::new(&mut sleeper).resume(()) {
            CoroutineState::Yielded(_) => queue.push_back(sleeper),
            CoroutineState::Complete(_) => counter += 1,
        }
    }

    println!("Done, cost {:?}", begin.elapsed());
}
