use rand::prelude::*;
use std::iter::successors;
use std::thread;
use std::time;
use threads_adv::{mpmc, paral::OffThread};

fn main() {
    // #1 run task in concurrent pipeline
    let l = successors(Some(0), |n| Some(n + 1))
        .take(20)
        .map(|n| n * n)
        .off_thread()
        .map(|n| n * 100)
        .off_thread()
        .collect::<Vec<u32>>();
    println!("{:?}", l);

    // #2 use case of home-made multiple receiver
    let mut hs = vec![];
    let (tx, rx) = mpmc::channel();

    for i in 0..=4 {
        let rx_cloned = rx.0.clone();
        let h = thread::spawn(move || {
            let mut rg = thread_rng();
            thread::sleep(time::Duration::from_secs(rg.gen_range(1..=5)));
            let v = rx_cloned.lock().unwrap().recv().unwrap();
            println!("[Thread {}] Get value: {}", i, v);
        });

        hs.push(h);
    }

    for i in 0..=4 {
        tx.send(i).unwrap();
    }

    for h in hs {
        h.join().unwrap();
    }
}
