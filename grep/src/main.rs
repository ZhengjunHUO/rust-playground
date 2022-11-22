use std::{env, process};
use grep::{Config, exec};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Get an err when parsing the args: {}", err);
        process::exit(1);
    });
    println!("grep {} from {}", config.pattern, config.path_to_file);

    if let Err(e) = exec(config) {
        println!("Get an err when read from the file: {}", e);
        process::exit(2);
    }
}
