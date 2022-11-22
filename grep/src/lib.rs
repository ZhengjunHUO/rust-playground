use std::fs;
use std::error::Error;

pub struct Config {
    pub pattern: String,
    pub path_to_file: String,
}

impl Config {
    // &'static str: string literal that have the 'static lifetime
    pub fn build(args :&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("[Usage] cargo run -- <pattern> <path_to_file>");
        }
        Ok(Config { pattern: args[1].clone(), path_to_file: args[2].clone() })
    }
}

pub fn exec(config: Config) -> Result<(), Box<dyn Error>> {
    let cont = fs::read_to_string(&config.path_to_file)?;
    println!("The content of {}:\n{}", config.path_to_file, cont);
    Ok(())
}
