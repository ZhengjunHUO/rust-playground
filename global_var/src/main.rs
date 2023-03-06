use rand::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!(
        "[DEBUG] Initially, counter has value: {}",
        COUNTER.load(Ordering::SeqCst)
    );
    let mut hs = vec![];

    for i in 1..=5 {
        hs.push(thread::spawn(move || {
            let n: usize = thread_rng().gen_range(1..=3);
            println!("[Thread {}] will increment counter {} time(s).", i, n);
            for _j in 0..n {
                COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for h in hs {
        h.join().expect("Join thread failed!");
    }
    println!(
        "[DEBUG] counter's final value: {}",
        COUNTER.load(Ordering::SeqCst)
    );
}
