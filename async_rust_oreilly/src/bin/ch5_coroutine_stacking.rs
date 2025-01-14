#![feature(coroutines)]
#![feature(coroutine_trait)]
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::time::Instant;

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

struct WriterWrapper {
    fd: File,
}

impl WriterWrapper {
    fn new(path: &str) -> io::Result<Self> {
        let fd = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { fd })
    }
}

impl Coroutine<i32> for WriterWrapper {
    type Yield = ();
    type Return = ();

    fn resume(mut self: Pin<&mut Self>, arg: i32) -> CoroutineState<Self::Yield, Self::Return> {
        writeln!(self.fd, "{}", arg).unwrap();
        CoroutineState::Yielded(())
    }
}

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
        loop {
            match Pin::new(&mut self.reader).resume(()) {
                CoroutineState::Yielded(num) => {
                    Pin::new(&mut self.writer).resume(num);
                }
                CoroutineState::Complete(_) => break,
            }
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
