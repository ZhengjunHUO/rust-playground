#[allow(clippy::precedence)]
fn main() {
    // (1)
    // 需要明确指出type(i32)
    println!("{}", (-4i32).abs());
    // 或者
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

    // 需要加上括号，否则
    println!("{}", -4i32.abs());


    // (2)
    assert_eq!(254_u8.checked_add(1), Some(255));
    assert_eq!(255_u8.checked_add(1), None);
    assert_eq!((-128_i8).checked_div(-1), None);

    let mut i: u32 = 1;
    loop {
        //i *= 100;
        i = i.checked_mul(100).expect("mul overflowed!");
    }
}
