use ganzhi::g2c;
use std::io;

fn main() {
    loop {
        // read input
        println!("Input a year [1900-2100]:");
        let mut year = String::new();
        io::stdin()
            .read_line(&mut year)
            .expect("Failed to read from user's input");

        // parse input
        let year: usize = match year.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input, try again with number between 1900 and 2100");
                continue;
            }
        };

        match g2c(year) {
            Some(s) => println!("{}", s),
            None => {
                println!("Input should between 1900 and 2100!");
                break;
            }
        }
    }
}
