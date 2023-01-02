fn main() {
    println!("Hello, \
        world!");

    // raw string
    println!(r##"
        Hello "Rust"!
        You can trust "Rust"!
    "##);

    // byte string (b"POST"类型为&[u8; 4])
    let method_bs = b"POST";
    let method_a = [b'P', b'O', b'S', b'T'];
    assert_eq!(method_bs, &method_a);

    // new string (UTF-8的列)
    let emjs = "😊😂😄".to_string();
    assert_eq!(emjs.len(), 12);
    assert_eq!(emjs.chars().count(), 3);
}
