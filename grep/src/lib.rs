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
    let text = fs::read_to_string(&config.path_to_file)?;
    //println!("The content of {}:\n{}", config.path_to_file, text);
    for l in find(&config.pattern, &text) {
        println!("{}", l);
    }

    Ok(())
}

pub fn find<'a>(pattern: &str, text: &'a str) -> Vec<&'a str> {
    let mut rslt = Vec::new();

    for l in text.lines() {
        if l.contains(pattern) {
            rslt.push(l);
        }
    }

    rslt
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

}
