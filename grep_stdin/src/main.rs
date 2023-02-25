use std::io;
use std::io::prelude::*;

fn grep<B>(pattern: &str, br: B) -> io::Result<()>
where
    B: BufRead,
{
    let lines = br.lines();
    for line in lines {
        let l = line?;
        if l.contains(pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}

fn main() {
    println!("grep from stdin ...");
    let _ = grep("Rust", io::stdin().lock());
}
