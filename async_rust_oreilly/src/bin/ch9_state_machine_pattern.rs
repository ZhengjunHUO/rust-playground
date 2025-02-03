use std::{future::Future, pin::Pin, task::Poll};

enum State {
    On,
    Off,
}

enum Event {
    SwitchOn,
    SwitchOff,
}

impl State {
    fn trigger(self, event: Event) -> Self {
        match (&self, event) {
            (State::On, Event::SwitchOff) => {
                println!("State [ON] => [OFF]");
                State::Off
            }
            (State::Off, Event::SwitchOn) => {
                println!("State [OFF] => [ON]");
                State::On
            }
            _ => {
                println!("Nothing todo, skip.");
                self
            }
        }
    }
}

struct StateFuture<F: Future, G: Future> {
    state: State,
    on_future: Pin<Box<F>>,
    off_future: Pin<Box<G>>,
}

impl<F: Future, G: Future> Future for StateFuture<F, G> {
    type Output = State;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.state {
            State::On => {
                let inner = self.on_future.as_mut();
                let _ = inner.poll(cx);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            State::Off => {
                let inner = self.off_future.as_mut();
                let _ = inner.poll(cx);
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

fn main() {
    let mut state = State::On;
    state = state
        .trigger(Event::SwitchOff)
        .trigger(Event::SwitchOn)
        .trigger(Event::SwitchOn);

    match state {
        State::On => println!("State machine is ON"),
        _ => unreachable!(),
    }
}
