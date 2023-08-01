use ansi_term::{Colour, Style};
use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};

fn main() {
    let size = std::env::args()
        .nth(1)
        .expect("Need to specify the password length.")
        .parse::<usize>()
        .unwrap();

    let mut rng = thread_rng();

    // 1) based on DistString trait implemented by Alphanumeric
    println!(
        "{}: {}",
        Colour::Blue.paint("Approach 1"),
        Alphanumeric.sample_string(&mut rng, size)
    );
    // 2) based on Rng: RngCore trait implemented by ThreadRng
    println!(
        "{}: {}",
        Colour::Red.bold().paint("Approach 2"),
        (0..size)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>()
    );
    // 3) also based on Rng: RngCore trait implemented by ThreadRng
    println!(
        "{}: {}",
        Colour::Cyan.italic().paint("Approach 3"),
        Style::new().blink().paint(
            rng.sample_iter(Alphanumeric)
                .take(size)
                .map(char::from)
                .collect::<String>()
        )
    );
}
