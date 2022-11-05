use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    //println!("The secret is: {}", secret);

    loop {
        println!("Input you guess number here [1-100]:");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Sorry too small !"),
            Ordering::Greater => println!("Sorry too big !"),
            Ordering::Equal => {
                println!("Yes that's it !");
                break;
            },
        }
    }
}
