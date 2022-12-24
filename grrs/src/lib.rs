use anyhow::{Context, Result};
use std::{
    io::{self, BufRead, BufReader},
};

pub fn search_in<R>(buf: BufReader<R>, pattern: &str, mut writer: impl io::Write) -> Result<()>
where
    R: std::io::Read,
{
    for line in buf.lines() {
        if let Ok(l) = line {
            if l.contains(pattern) {
                writeln!(writer, "{}", l)
                    .with_context(|| format!("Error occurred during write"))?;
            }
        }
    }
    Ok(())
}

#[test]
fn search_in_test() {
    let mut rslt = Vec::new();
    assert!(matches!(search_in(BufReader::new("Huo is a rustacean now\nbut he need some rest.".as_bytes()), "rust", &mut rslt), Ok(())));
    assert_eq!(rslt, b"Huo is a rustacean now\n");
}
