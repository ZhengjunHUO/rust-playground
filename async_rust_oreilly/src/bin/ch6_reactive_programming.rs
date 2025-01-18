use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, AtomicI16, Ordering},
        Arc, LazyLock,
    },
    task::Poll,
};

static TEMPERATURE: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| Arc::new(AtomicI16::new(1000)));
static WANTED: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| Arc::new(AtomicI16::new(1800)));
static SWITCH_ON: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));

struct Display {
    temperature: i16,
}

impl Display {
    fn new() -> Self {
        Self {
            temperature: TEMPERATURE.load(Ordering::SeqCst),
        }
    }
}

impl Future for Display {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let current = TEMPERATURE.load(Ordering::SeqCst);
        let wanted = WANTED.load(Ordering::SeqCst);
        let on = SWITCH_ON.load(Ordering::SeqCst);

        if self.temperature == current {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        if current < wanted && !on {
            SWITCH_ON.store(true, Ordering::SeqCst);
        } else if current > wanted && on {
            SWITCH_ON.store(false, Ordering::SeqCst);
        }

        self.temperature = current;

        clearscreen::clear().unwrap();
        println!(
            "Acutal temperature: {} [Wanted: {}]; HEATER IS {}",
            current as f32 / 100.0,
            wanted as f32 / 100.0,
            if on { "[ON]" } else { "[OFF]" }
        );

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() {}
