use clap::Parser;
use serde_json::json;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Params {
    file: PathBuf,

    /// Output in json format
    #[arg(short = 'j', long = "json")]
    json: bool,
}

fn main() {
    let args = Params::parse();
    let mut c = 0;
    let f = args.file;

    for l in fs::read_to_string(&f).unwrap().lines() {
        c += l.split(' ').count();
    }

    let rslt = format!("`{}`'s word count: {}", f.to_str().unwrap(), c);

    if args.json {
        println!(
            "{}",
            json!({
                "type": "info",
                "message": rslt,
            })
        );
    } else {
        println!("{}", rslt);
    }
}
