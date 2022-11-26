use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub pattern: String,
    pub path_to_file: String,
    pub ignore_case: bool,
}

impl Config {
    // &'static str: string literal that have the 'static lifetime
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip args[0]: the name of func itself
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("No pattern found !"),
        };

        let path_to_file = match args.next() {
            Some(arg) => arg,
            None => return Err("No path to target file found !"),
        };

        Ok(Config {
            pattern,
            path_to_file,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn exec(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.path_to_file)?;
    //println!("The content of {}:\n{}", config.path_to_file, text);

    let rslt = if config.ignore_case {
        find_ignore_case(&config.pattern, &text)
    }else{
        find(&config.pattern, &text)
    };

    for l in rslt {
        println!("{}", l);
    }

    Ok(())
}

pub fn find<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|l| l.contains(pattern)).collect()
}

pub fn find_ignore_case<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    // shadowed variable, return a String (create new data)
    let pattern = pattern.to_lowercase();
    text.lines().filter(|l| l.to_lowercase().contains(&pattern)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grep_pattern() {
        let pattern = "fu";
        let text = "\
Hello Rust,
fufu is a cat,
foo bar.";

        assert_eq!(vec!["fufu is a cat,"], find(pattern, text));
    }

    #[test]
    fn grep_pattern_ignore_case() {
        let pattern = "FufU";
        let text = "\
Hello fufu,
fufu is a cat,
foo bar.";

        assert_eq!(vec!["Hello fufu,", "fufu is a cat,"], find_ignore_case(pattern, text));
    }
}
