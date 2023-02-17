use std::iter::{from_fn, successors};

fn main() {
    // #1 Generate a vec with random value
    let rand_pool: Vec<u8> = from_fn(|| Some(rand::random::<u8>())).take(5).collect();
    println!("rand_pool: {:?}", rand_pool);

    // #2 Generate a vec, initialized with 0-9
    let incr_pool: Vec<u32> = successors(Some(0), |&n| Some(n + 1)).take(10).collect();
    assert_eq!(incr_pool, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // #3 drain moved a part out of a string and build an iterator
    let mut sayit = "RustRocks".to_string();
    let part = String::from_iter(sayit.drain(1..5));
    assert_eq!(sayit, "Rocks");
    assert_eq!(part, "ustR");
}
