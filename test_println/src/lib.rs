pub fn say_hi() {
    println!("Hello Huo !");
}

#[cfg(test)]
mod tests {
    use super::*;
    use gag::BufferRedirect;
    use std::io::Read;

    /// Should run with $ cargo test -- --nocapture
    #[test]
    fn test_stdout() {
        let mut stdout = BufferRedirect::stdout().unwrap();
        let mut buf = String::new();

        say_hi();

        stdout.read_to_string(&mut buf).unwrap();
        assert_eq!(&buf[..], "Hello Huo !\n");
    }
}
