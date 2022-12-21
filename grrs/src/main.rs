use anyhow::{Context, Result};
use clap::Parser;
use std::{
    io::{BufRead, BufReader},
    {fs, path},
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 't', long = "pattern")]
    pattern: String,
    #[arg(short = 'p', long = "path")]
    path: path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    /*
    let buf = fs::read_to_string(&args.path).expect("failed to read target file");
    for l in buf.lines() {
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }
    */

    //let f = fs::File::open(&args.path).expect("failed to open target file");
    let f = fs::File::open(&args.path).with_context(|| format!("Failed to open file `{}`", &args.path.display()))?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        if let Ok(l) = line {
            if l.contains(&args.pattern) {
                println!("{}", l);
            }
        }
    }
    Ok(())
}
