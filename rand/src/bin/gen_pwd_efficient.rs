use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

fn main() {
    let size = std::env::args()
        .nth(1)
        .expect("Need to specify the password length.");
    println!(
        "{}",
        Alphanumeric.sample_string(&mut thread_rng(), size.parse::<usize>().unwrap())
    );
}
