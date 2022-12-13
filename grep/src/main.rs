use grep::{exec, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Get an err when parsing the args: {}", err);
        process::exit(1);
    });
    println!(
        "[DEBUG] grep {} from {}",
        config.pattern, config.path_to_file
    );

    if let Err(e) = exec(config) {
        eprintln!("Get an err when read from the file: {}", e);
        process::exit(2);
    }
}
