#![feature(coroutines)]
#![feature(coroutine_trait)]
use std::io;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

use async_rust_oreilly::io::ReaderWrapper;

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
