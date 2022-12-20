use std::path;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 't', long = "pattern")]
    pattern: String,
    #[arg(short = 'p', long = "path")]
    path: path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
