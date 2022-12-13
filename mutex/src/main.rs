use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = 5;
    let mx = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..count {
        let c = Arc::clone(&mx);
        let h = thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }

    assert_eq!(count, *mx.lock().unwrap());
    println!("The mutex is {:?}", mx);
}
