use rand::{thread_rng, Rng};

const DICT: &[u8] = b"*&^%$#@!~\
                      ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                      abcdefghijklmnopqrstuvwxyz\
                      0123456789)(*&^%$#@!~";

fn main() {
    let size = std::env::args()
        .nth(1)
        .expect("Need to specify the password length.")
        .parse::<usize>()
        .unwrap();

    let mut rng = thread_rng();

    // based on Rng: RngCore trait implemented by ThreadRng
    println!(
        "{}",
        (0..size)
            .map(|_| { DICT[rng.gen_range(0..size)] as char })
            .collect::<String>()
    );
}
