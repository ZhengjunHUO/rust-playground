use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::successors;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let prime = vec![2, 3, 5, 7, 11];

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

    // Fork-join parallelism
    // multi-thread job traitment using Arc
    let dict = Arc::new(
        ["huo", "fufu", "foo", "bar"]
            .into_iter()
            .zip([1, 2, 3, 4].into_iter())
            .collect::<HashMap<&str, u32>>(),
    );
    let nthread = 8;
    //let tasks: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let tasks: Vec<u32> = successors(Some(1), |n| Some(n + 1)).take(18).collect();
    let mut handlers = vec![];

    //let chunks = tasks.chunks(tasks.len().div_ceil(nthread)).map(|v| v.to_vec()).collect::<Vec<Vec<u32>>>();
    let chunks = tasks
        .chunks(tasks.len() / nthread + 1)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<u32>>>();
    println!("[INFO] Running with thread::spawn ...");
    for ids in chunks {
        // use Arc to solve the ownership movement problem for threads
        let d = dict.clone();
        handlers.push(thread::spawn(move || handle_task(ids, &d)));
    }

    for h in handlers {
        h.join().unwrap();
    }
    println!("[INFO] Done.\n");

    // using Rayon
    println!("[INFO] Running with Rayon ...");
    let partitions = tasks
        .chunks(1)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<u32>>>();
    //let _ = partitions.par_iter().map(|task_id| handle_task(task_id.to_vec(), &dict)).collect::<Vec<_>>();
    partitions
        .par_iter()
        .for_each(|task_id| handle_task(task_id.to_vec(), &dict));
    println!("[INFO] Done.");
}

fn handle_task(task_ids: Vec<u32>, dict: &HashMap<&str, u32>) {
    println!("Handling tasks: {:?}", task_ids);
    println!("Looking into dictionary sized {}", dict.len());
    let mut rng = thread_rng();
    for task_id in task_ids {
        println!("[Task {}] Working in progress ...", task_id);
        thread::sleep(Duration::from_secs(rng.gen_range(0..=5)));
        println!("[Task {}] All done, quit ...", task_id);
    }
}
