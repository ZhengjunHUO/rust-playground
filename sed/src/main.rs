use regex::Regex;
use std::{
    env,
    fs::{read_to_string, write},
    process::exit,
};
use text_colorizer::*;

#[derive(Debug)]
struct Params {
    pattern: String,
    replace: String,
    file: String,
    result: String,
}

fn usage() {
    eprintln!("{}: replace a pattern in the file", "sed".red().bold());
    eprintln!(
        "{}: sed <pattern> <replace> <file> <result>",
        "Usage".green()
    );
}

fn parse_args() -> Params {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        usage();
        exit(1);
    };

    Params {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        file: args[2].clone(),
        result: args[3].clone(),
    }
}

fn replace(pattern: &str, replace: &str, content: &str) -> Result<String, regex::Error> {
    let reg = Regex::new(pattern)?;
    Ok(reg.replace_all(content, replace).to_string())
}

fn main() {
    let args = parse_args();
    println!("[Debug]: {:?}", args);

    let content = match read_to_string(&args.file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "{} Failed to read from file {}: {:?}",
                "[Error]".red().bold(),
                args.file,
                e
            );
            exit(1);
        }
    };

    let result = match replace(&args.pattern, &args.replace, &content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{} Failed to sed the file {}: {:?}",
                "[Error]".red().bold(),
                args.file,
                e
            );
            exit(1);
        }
    };

    match write(&args.result, &result) {
        Ok(_) => {
            println!(
                "{} Result wrote to {}.",
                "[OK]".green().bold(),
                &args.result
            );
        }
        Err(e) => {
            eprintln!(
                "{} Failed to write to file {}: {:?}",
                "[Error]".red().bold(),
                args.file,
                e
            );
            exit(1);
        }
    };
}
