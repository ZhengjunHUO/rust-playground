use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

fn grep<B>(pattern: &str, br: B) -> io::Result<()>
where
    B: BufRead,
{
    // use collect to turn a Vec<io::Result<String>> to io::Result<Vec<String>>
    // to avoid do let line = l?; in the for loop
    let lines = br.lines().collect::<io::Result<Vec<String>>>()?;
    for l in lines {
        if l.contains(pattern) {
            println!("{}", l);
        }
    }
    Ok(())
}

fn do_grep() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    // Try get the pattern at args[1]
    let pattern = match args.next() {
        Some(p) => p,
        None => Err("Can't parse the pattern ...")?,
    };

    let fs = args.map(PathBuf::from).collect::<Vec<PathBuf>>();
    // If no file is provided in args, try grep pattern from stdin
    if fs.is_empty() {
        grep(&pattern, io::stdin().lock())?;
    } else {
        for f in fs {
            let file = File::open(f)?;
            grep(&pattern, BufReader::new(file))?;
        }
    }

    Ok(())
}

fn main() {
    let rslt = do_grep();
    match rslt {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}
