use lazy_static::lazy_static;
use rand::prelude::*;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time;

// test const func
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB: ({}-{}-{})", self.red, self.green, self.blue)
    }
}

const fn u8_to_rgb(n: u8) -> Color {
    Color {
        red: n,
        green: n,
        blue: n,
    }
}

const WHITE_C: Color = u8_to_rgb(255);
const BLACK_C: Color = u8_to_rgb(0);
// fin de test const func

static COUNTER: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    static ref SIGNIN: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    // #1 test atomic global usize
    println!(
        "[DEBUG] Initially, counter has value: {}",
        COUNTER.load(Ordering::SeqCst)
    );
    let mut hs = vec![];

    for i in 1..=5 {
        hs.push(thread::spawn(move || {
            // generate a random num between 1-3
            let n: usize = thread_rng().gen_range(1..=3);
            thread::sleep(time::Duration::from_secs(n as u64));
            // sign in
            SIGNIN
                .lock()
                .unwrap()
                .push_str(format!("[Thread {}]", i).as_str());
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

    // #2 test const fn
    println!("WHITE: {}", WHITE_C);
    println!("BLACK: {}", BLACK_C);

    // #3 test lazy_static
    println!("SIGNIN: {}", SIGNIN.lock().unwrap());
}
