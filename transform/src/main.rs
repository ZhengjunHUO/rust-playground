use std::net::Ipv4Addr;
use std::ops::Deref;
use std::str::FromStr;

fn main() {
    // 1) Option<String> to Option<&str>
    let test = Some("Rusty".to_owned());
    // as_ref把Option<String>转变为Option<&String>, as_str()再将&String转为&str
    assert_eq!(test.as_ref().map(|s| s.as_str()), Some("Rusty"));
    // 也可用as_deref()一步到位
    assert_eq!(test.as_deref(), Some("Rusty"));

    let test_deref = "Rusty".to_owned();
    // x.deref()返回Target = str的&
    assert_eq!(test_deref.deref(), "Rusty");
    // *x返回Target = str, 需要再取&
    assert_eq!(&*test_deref, "Rusty");

    // 1b) Option<&T> to Option<T>
    assert_eq!(
        Some("rustacean").map(|s| s.to_string()),
        Some(String::from("rustacean"))
    );
    assert_eq!(Some(&88).cloned(), Some(88));

    // 2) String to Vec<u8> (String consumed)
    //let buf: Vec<u8> = String::from("TrustRust").into_bytes();
    let buf: Vec<u8> = "TrustRust".to_string().into();
    //let expected: Vec<u8> = vec![84, 114, 117, 115, 116, 82, 117, 115, 116];
    //let expected: Vec<u8> = b"TrustRust".to_vec();
    let expected: Vec<u8> = Vec::from("TrustRust");
    assert_eq!(&buf, &expected);

    // 2b) str to &[u8]
    let origin_str = "Rusty";
    assert_eq!(b"Rusty", origin_str.as_bytes());

    // 3) Vec<u8> to String
    let restored = String::from_utf8_lossy(&expected).to_string();
    assert_eq!(String::from("TrustRust"), restored);

    let mut string_with_ctrl_char = String::from("Hello\u{1}\0Huo");
    string_with_ctrl_char.retain(|c| c.is_ascii() && !c.is_control());
    let wanted = "HelloHuo";
    assert_eq!(&string_with_ctrl_char, &wanted);

    // 4) bool to int
    assert_eq!(i8::from(true), 1);

    // 5) Vec<u8> to Ipv4Addr
    let addr = Ipv4Addr::new(192, 168, 0, 10);
    assert_eq!(addr, vec_to_ipv4(vec![192, 168, 0, 10]).unwrap());
    assert_eq!(addr, Ipv4Addr::from_str("192.168.0.10").unwrap());

    // 6) [u8; 2] to u16
    let bytes: [u8; 2] = [10, 20];
    // 0x140A
    assert_eq!(u16::from_le_bytes(bytes), 5130);
    // 0x0A14
    assert_eq!(u16::from_be_bytes(bytes), 2580);

    // 7) [u8; 4] + 2 * u16 => [u8; 8]
    assert_eq!(build_array(), [10, 20, 30, 40, 0, 2, 0, 4]);

    // 8) String to int
    assert_eq!(String::from("18").parse::<usize>().unwrap(), 18);
}

// Vec<u8> to Ipv4Addr
fn vec_to_ipv4(vec: Vec<u8>) -> Option<Ipv4Addr> {
    if vec.len() == 4 {
        Some(Ipv4Addr::from([vec[0], vec[1], vec[2], vec[3]]))
    } else {
        None
    }
}

// build a [u8; 8] from a [u8; 4] plus two u16
fn build_array() -> [u8; 8] {
    let array1: [u8; 4] = [10, 20, 30, 40];
    let value1: u16 = 512;
    let value2: u16 = 1024;

    let bytes1: [u8; 2] = value1.to_le_bytes();
    let bytes2: [u8; 2] = value2.to_le_bytes();

    let mut result: [u8; 8] = [0; 8];
    result[..4].copy_from_slice(&array1);
    result[4..6].copy_from_slice(&bytes1);
    result[6..8].copy_from_slice(&bytes2);
    result
}
