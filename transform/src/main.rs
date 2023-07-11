fn main() {
    // Option<String> to Option<&str>
    let test = Some("Rusty".to_owned());
    assert_eq!(test.as_deref(), Some("Rusty"));

    // String to Vec<u8> (String consumed)
    let buf: Vec<u8> = String::from("TrustRust").into_bytes();
    let expected: Vec<u8> = vec![84, 114, 117, 115, 116, 82, 117, 115, 116];
    assert_eq!(&buf, &expected);

    // Vec<u8> to String
    let restored = String::from_utf8_lossy(&expected).to_string();
    assert_eq!(String::from("TrustRust"), restored);

    let mut string_with_ctrl_char = String::from("Hello\u{1}\0Huo");
    string_with_ctrl_char.retain(|c| c.is_ascii() && !c.is_control());
    let wanted = "HelloHuo";
    assert_eq!(&string_with_ctrl_char, &wanted);
}
