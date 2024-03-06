const FIVE: i32 = 5;

fn add_i32(num: Option<i32>, delta: i32) -> Option<i32> {
    num.map(|i| i + delta)
}

fn get_value_i32(num: Option<i32>) -> i32 {
    // if let syntax
    if let Some(value) = num {
        //println!("The value is {}", value);
        value
    } else {
        0
    }
}

fn main() {
    // test option enum
    let num_orig = Some(3);
    let num = add_i32(num_orig, FIVE);
    println!(
        "{} adds {} equals to {}",
        get_value_i32(num_orig),
        FIVE,
        get_value_i32(num)
    );

    let none_orig = None;
    let none = add_i32(none_orig, FIVE);
    println!("{:?} adds {} equals to {:?}", none_orig, FIVE, none);
}
