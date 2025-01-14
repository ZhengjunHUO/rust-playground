use std::io::Write;
use std::pin::Pin;

use async_rust_oreilly::io::{ReaderWrapper, WriterWrapper};

trait SymmetricCoroutine {
    type Input;
    type Output;

    fn resume_symm(self: Pin<&mut Self>, input: Self::Input) -> Self::Output;
}

impl SymmetricCoroutine for ReaderWrapper {
    type Input = ();
    type Output = Option<i32>;

    fn resume_symm(mut self: Pin<&mut Self>, _input: Self::Input) -> Self::Output {
        if let Some(Ok(num)) = self.lines.next() {
            num.parse::<i32>().ok()
        } else {
            None
        }
    }
}

impl SymmetricCoroutine for WriterWrapper {
    type Input = i32;
    type Output = ();

    fn resume_symm(mut self: Pin<&mut Self>, input: Self::Input) -> Self::Output {
        writeln!(self.fd, "{}", input).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let mut reader = ReaderWrapper::new("foo.txt")?;
    let mut writer = WriterWrapper::new("out.txt")?;

    // A true symmetrical coroutine would pass control from the reader to the writer
    // without having to return to the main function
    while let Some(num) = Pin::new(&mut reader).resume_symm(()) {
        Pin::new(&mut writer).resume_symm(num);
    }

    Ok(())
}
