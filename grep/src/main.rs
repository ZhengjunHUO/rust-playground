use std::{env, fs, process};

struct Config {
    pattern: String,
    path_to_file: String,
}

impl Config {
    // &'static str: string literal that have the 'static lifetime
    fn build(args :&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("[Usage] cargo run -- <pattern> <path_to_file>");
        }
        Ok(Config { pattern: args[1].clone(), path_to_file: args[2].clone() })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Get an err when parsing the args: {}", err);
        process::exit(10);
    });
    println!("grep {} from {}", config.pattern, config.path_to_file);

    let cont = fs::read_to_string(&config.path_to_file).expect("Need to read from a exist file.");
    println!("The content of {}:\n{}", &config.path_to_file, cont);
}
