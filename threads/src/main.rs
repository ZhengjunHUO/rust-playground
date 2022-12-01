use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

    // test channel

    // multi producer single consumer
    let (tx, rx) = mpsc::channel();
    let tx_cloned = tx.clone();

    thread::spawn(move || {
        let ss = vec![
            String::from("Greeting"),
            String::from("from"),
            String::from("the"),
            String::from("upstream!"),
        ];

        for s in ss {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let ss = vec![
            String::from("Hello"),
            String::from("it's"),
            String::from("another"),
            String::from("producer!"),
        ];

        for s in ss {
            tx_cloned.send(s).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for r in rx {
        println!("[Main] Receive msg from thread: {}", r);
    }
}
