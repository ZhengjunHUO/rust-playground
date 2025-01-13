#![feature(coroutines)]
#![feature(coroutine_trait)]
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::time::Instant;

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

struct FdWrapper {
    fd: File,
}

impl FdWrapper {
    fn new(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { fd: file })
    }
}

impl Coroutine<i32> for FdWrapper {
    type Yield = ();
    type Return = ();

    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.fd, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

// fn add_num_to_file(n: i32) -> io::Result<()> {
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("foo.txt")?;
//     writeln!(file, "{}", n)?;
//     Ok(())
// }

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let nums = (0..100000).map(|_| rng.gen()).collect::<Vec<i32>>();
    let begin = Instant::now();

    let mut coroutine = FdWrapper::new("foo.txt")?;
    for num in nums {
        Pin::new(&mut coroutine).resume(num);
        // if let Err(e) = add_num_to_file(num) {
        //     eprintln!("Error occurred writing to file: {e}");
        // }
    }
    let elapsed = begin.elapsed();
    println!("Task took {:?}", elapsed);
    Ok(())
}
