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
    println!("Approach 1: {}", Alphanumeric.sample_string(&mut rng, size));
    // 2) based on Rng: RngCore trait implemented by ThreadRng
    println!(
        "Approach 2: {}",
        (0..size)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect::<String>()
    );
    // 3) also based on Rng: RngCore trait implemented by ThreadRng
    println!(
        "Approach 3: {}",
        rng.sample_iter(Alphanumeric)
            .take(size)
            .map(char::from)
            .collect::<String>()
    );
}
