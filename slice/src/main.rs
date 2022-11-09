fn main() {
    // 1. string slice
    let s = String::from("Hello world!");

    // let s1 = &s[0..5];
    let s1 = &s[..5];

    // let s2 = &s[6..12];
    // let s2 = &s[6..s.len()];
    let s2 = &s[6..];

    println!("{} {}", s1, s2);
    println!("{}", &s[..]);

    let w = first_word(&s);
    println!("The first word is: {w}");

    // 2. int slice
    let int_slice = [1, 2, 3, 4, 5];
    let s3 = &int_slice[1..4];
    assert_eq!(s3, &[2, 3, 4])
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
