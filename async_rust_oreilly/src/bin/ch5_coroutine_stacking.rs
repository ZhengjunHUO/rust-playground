#![feature(coroutines)]
#![feature(coroutine_trait)]
use std::io;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::time::Instant;

use async_rust_oreilly::io::{ReaderWrapper, WriterWrapper};

struct RWMgr {
    reader: ReaderWrapper,
    writer: WriterWrapper,
}

impl RWMgr {
    fn new(from: &str, to: &str) -> io::Result<Self> {
        let reader = ReaderWrapper::new(from)?;
        let writer = WriterWrapper::new(to)?;
        Ok(Self { reader, writer })
    }

    fn run(&mut self) {
        while let CoroutineState::Yielded(num) = Pin::new(&mut self.reader).resume(()) {
            Pin::new(&mut self.writer).resume(num);
        }
    }
}

fn main() -> io::Result<()> {
    let mut rw_manager = RWMgr::new("foo.txt", "out.txt")?;
    println!("Running job ...");
    let begin = Instant::now();
    rw_manager.run();
    println!("Job done in {:?}", begin.elapsed());
    Ok(())
}
