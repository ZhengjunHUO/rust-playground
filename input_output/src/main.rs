use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal;
use serde_json::json;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
    process::exit,
};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Params {
    file: PathBuf,

    /// Output in json format
    #[arg(short = 'j', long = "json")]
    json: bool,
}

fn count_words<R: BufRead>(r: R) -> usize {
    let mut c = 0;

    for l in r.lines() {
        c += l.unwrap().split(' ').count();
    }
    c

    // when f is PathBuf
    //for l in std::fs::read_to_string(&f).unwrap().lines() {
    //    c += l.split(' ').count();
    //}
}

fn main() {
    let args = Params::parse();

    let mut f = args.file;
    let c;

    if f == PathBuf::from("-") {
        if stdin().is_terminal() {
            Params::command().print_help().unwrap();
            exit(1);
        }

        f = PathBuf::from("<stdin>");
        c = count_words(BufReader::new(stdin().lock()));
    } else {
        c = count_words(BufReader::new(File::open(&f).unwrap()));
    }

    let rslt = format!("`{}`'s word count: {}", f.to_string_lossy(), c);

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
