use std::thread;
use std::time::Duration;

fn main() {
    let prime = vec![2, 3, 5 ,7, 11];

    // prime's ownership moved to thread
    let h1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("[Thread #1] Moved a vec from the main: {:?}", prime);
    });

    let h2 = thread::spawn(|| {
        for i in 1..11 {
            println!("[Thread #2] Iteration {}.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..4 {
        println!("[Main] Iteration {}.", i);
        thread::sleep(Duration::from_millis(2));
    }
    println!("[Main] All jobs done.");

    h1.join().unwrap();
    println!("[Thread #1] Jobs done, quit.");
    h2.join().unwrap();
    println!("[Thread #2] Jobs done, quit.");
}
