use aes_gcm::aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::Aes256Gcm;
use hex::ToHex;

fn main() {
    let msg = b"Hello AES";

    let key = GenericArray::from_slice(b"abcdefghijklmnopqrstuvwxyz123456");
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    println!("key length: {}\nnonce length: {}", key.len(), nonce.len());
    let cipher = Aes256Gcm::new(key);

    // Encrypt
    let ciphered = cipher.encrypt(&nonce, msg.as_ref()).unwrap();
    println!("ciphered length: {}", ciphered.len());
    println!("ciphered hex: {}", ciphered.encode_hex::<String>());

    // Decrypt
    let plaintext = cipher.decrypt(&nonce, ciphered.as_ref()).unwrap();
    println!(
        "plantext: {:?}",
        String::from_utf8_lossy(plaintext.as_ref())
    );
    assert_eq!(&plaintext, msg);
}
