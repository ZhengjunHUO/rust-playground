use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, AtomicI16, Ordering},
        Arc, LazyLock,
    },
    task::Poll,
    time::{Duration, Instant},
};

// Subjects
static TEMPERATURE: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| Arc::new(AtomicI16::new(1700)));
static WANTED: LazyLock<Arc<AtomicI16>> = LazyLock::new(|| Arc::new(AtomicI16::new(1800)));
static SWITCH_ON: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));

// Display observer
pub struct Display {
    temperature: i16,
}

impl Display {
    pub fn new() -> Self {
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
            if SWITCH_ON.load(Ordering::SeqCst) {
                "[ON]"
            } else {
                "[OFF]"
            }
        );

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// Heater observer
pub struct Heater {
    last_update: Instant,
}

impl Heater {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }
}

impl Future for Heater {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        if SWITCH_ON.load(Ordering::SeqCst) {
            let current = Instant::now();
            if current.duration_since(self.last_update) < Duration::from_secs(3) {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }

            TEMPERATURE.fetch_add(30, Ordering::SeqCst);
        }

        self.last_update = Instant::now();
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// HeatLoss observer
pub struct HeatLoss {
    last_update: Instant,
}

impl HeatLoss {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }
}

impl Future for HeatLoss {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        let current = Instant::now();
        if current.duration_since(self.last_update) > Duration::from_secs(3) {
            TEMPERATURE.fetch_sub(10, Ordering::SeqCst);
            self.last_update = Instant::now();
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}