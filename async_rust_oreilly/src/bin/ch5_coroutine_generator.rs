#![feature(coroutines)]
#![feature(coroutine_trait)]
use std::io;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

use async_rust_oreilly::io::ReaderWrapper;

fn main() -> io::Result<()> {
    let mut coroutine = ReaderWrapper::new("foo.txt")?;

    while let CoroutineState::Yielded(num) = Pin::new(&mut coroutine).resume(()) {
        println!("Read from file: {num}");
    }

    Ok(())
}
