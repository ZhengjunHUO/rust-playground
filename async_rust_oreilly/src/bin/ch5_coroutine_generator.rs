#![feature(coroutines)]
#![feature(coroutine_trait)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

struct ReaderWrapper {
    lines: io::Lines<BufReader<File>>,
}

impl ReaderWrapper {
    fn new(path: &str) -> io::Result<Self> {
        let fd = File::open(path)?;
        Ok(Self {
            lines: BufReader::new(fd).lines(),
        })
    }
}

impl Coroutine<()> for ReaderWrapper {
    type Yield = i32;
    type Return = ();

    fn resume(mut self: Pin<&mut Self>, _arg: ()) -> CoroutineState<Self::Yield, Self::Return> {
        match self.lines.next() {
            Some(Ok(line)) => {
                if let Ok(num) = line.parse::<i32>() {
                    CoroutineState::Yielded(num)
                } else {
                    CoroutineState::Complete(())
                }
            }
            _ => CoroutineState::Complete(()),
        }
    }
}

fn main() -> io::Result<()> {
    let mut coroutine = ReaderWrapper::new("foo.txt")?;

    loop {
        match Pin::new(&mut coroutine).resume(()) {
            CoroutineState::Yielded(num) => println!("Read from file: {num}"),
            CoroutineState::Complete(_) => break,
        }
    }

    Ok(())
}
