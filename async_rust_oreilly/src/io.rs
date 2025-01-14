use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub struct ReaderWrapper {
    pub lines: io::Lines<BufReader<File>>,
}

impl ReaderWrapper {
    pub fn new(path: &str) -> io::Result<Self> {
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

pub struct WriterWrapper {
    pub fd: File,
}

impl WriterWrapper {
    pub fn new(path: &str) -> io::Result<Self> {
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
