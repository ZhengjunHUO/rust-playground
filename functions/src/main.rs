fn main() {
    let mut max = 0;
    let list: [u32; 5] = [12, 7, 88, 36, 1];

    for v in list {
        max = max_positive(max, v);
    }

    println!("Result: Array's max is {max}. Quiting...");

    for countdown_num in (1..4).rev() {
        println!("{countdown_num}");
    }

    println!("Bye.");
}

fn max_positive(x: u32, y: u32) -> u32 {
    if x > y {
        return x;
    }

    y
}
