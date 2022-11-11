const FIVE: i32 = 5;

fn add_i32(num: Option<i32>, delta: i32) -> Option<i32> {
    match num {
        Some(i) => Some(i + delta),
        None => None,
    }
}

fn main() {
    // test option enum
    let num_orig = Some(3);
    let num = add_i32(num_orig, FIVE);
    println!("{:?} adds {} equals to {:?}", num_orig, FIVE, num);

    let none_orig = None;
    let none = add_i32(none_orig, FIVE);
    println!("{:?} adds {} equals to {:?}", none_orig, FIVE, none);
}
