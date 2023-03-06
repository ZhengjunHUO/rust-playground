use std::sync::{Arc, Condvar, Mutex};
use std::thread;
//use std::time;

fn main() {
    // Condvar need a related Mutex
    let cv_tup = Arc::new((Mutex::new(false), Condvar::new()));
    let cv_tup_cloned = cv_tup.clone();

    thread::spawn(move || {
        let (m, cv) = &*cv_tup_cloned;
        let mut g = m.lock().unwrap();
        // thread does the job
        *g = true;
        cv.notify_one();
    });

    // Force the main thread to yield the lock to the worker thread
    // then the main thread will not enter the while loop
    //thread::sleep(time::Duration::from_secs(3));

    let (m, cv) = &*cv_tup;
    let mut g = m.lock().unwrap();
    while !*g {
        println!("Waiting for the worker doing its job ...");
        // enter here if the main thread acquaire the lock first
        // cv.wait(g) release the guard and block, so the thread can
        // obtain the Mutex, do the job, and call notify.
        // the main thread will reacquire the guard and unblock
        g = cv.wait(g).unwrap();
    }
    println!("Job done, get the result {}", g);
}
