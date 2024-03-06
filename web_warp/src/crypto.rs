use crate::models::InvalidToken;
use aes_gcm::{
    aead::{
        generic_array::{typenum::U12, GenericArray},
        Aead, KeyInit,
    },
    Aes256Gcm, Nonce,
};
use hex::FromHex;

pub(crate) fn decrypt_token(token: String) -> Result<String, warp::Rejection> {
    let key = "abcdefghijklmnopqrstuvwxyz123456".as_bytes();

    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let buf = Vec::<u8>::from_hex(token).unwrap();
    let buf_len = buf.len();

    let iv: &GenericArray<u8, U12> = Nonce::from_slice(&buf[(buf_len - 28)..(buf_len - 16)]);
    let ciphered = [&buf[..(buf_len - 28)], &buf[(buf_len - 16)..]].concat();

    match cipher.decrypt(iv, ciphered.as_slice()) {
        Ok(deciphered) => {
            let rslt = String::from_utf8(deciphered).unwrap();
            Ok(rslt)
        }
        Err(err) => {
            println!("[DEBUG] Error occurred during the decryption: {}", err);
            Err(warp::reject::custom(InvalidToken))
        }
    }
}
