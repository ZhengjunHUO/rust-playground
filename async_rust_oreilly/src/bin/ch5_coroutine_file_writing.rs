#![feature(coroutines)]
#![feature(coroutine_trait)]
use rand::Rng;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::Instant;

fn add_num_to_file(n: i32) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("foo.txt")?;
    writeln!(file, "{}", n)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let nums = (0..100000).map(|_| rng.gen()).collect::<Vec<i32>>();
    let begin = Instant::now();
    for num in nums {
        if let Err(e) = add_num_to_file(num) {
            eprintln!("Error occurred writing to file: {e}");
        }
    }
    let elapsed = begin.elapsed();
    println!("Task took {:?}", elapsed);
    Ok(())
}
