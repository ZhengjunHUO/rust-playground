use aes_gcm::{
    aead::{
        generic_array::{typenum::U12, GenericArray},
        Aead, KeyInit,
    },
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use hex::FromHex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Wait for a params: <ciphered_text>");
    }

    // decoding by default with `hex`, support `base64`
    let mut encoding = "hex";
    if args.len() > 2 && args[2] == "base64" {
        encoding = "base64";
    }

    // Hardcode secret key, should be the same key on other side
    let key = "abcdefghijklmnopqrstuvwxyz123456".as_bytes();
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();

    // decoded ciphered_text
    let buf: Vec<u8>;
    match encoding {
        "hex" => buf = Vec::<u8>::from_hex(&args[1]).unwrap(),
        "base64" => buf = general_purpose::STANDARD.decode(&args[1]).unwrap(),
        _ => unreachable!(),
    }
    let buf_len = buf.len();

    // Split iv and payload (the rest part)
    let iv: &GenericArray<u8, U12> = Nonce::from_slice(&buf[(buf_len - 28)..(buf_len - 16)]);
    let ciphered = [&buf[..(buf_len - 28)], &buf[(buf_len - 16)..]].concat();

    // decrypt the payload with iv
    match cipher.decrypt(iv, ciphered.as_slice()) {
        Ok(deciphered) => {
            let rslt = String::from_utf8(deciphered).unwrap();
            println!("Result: {}", rslt);
        }
        Err(err) => println!("{}", err),
    };
}
