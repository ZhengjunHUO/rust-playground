use boringauth::oath::{HashFunction, TOTPBuilder};

fn main() {
    let key_base = std::env::args()
        .skip(1)
        .next()
        .expect("Wait for a key to generate the password !");
    let result_base_1 = TOTPBuilder::new().base32_key(&key_base).hash_function(HashFunction::Sha1).finalize().unwrap().generate();
    let result_base_256 = TOTPBuilder::new().base32_key(&key_base).hash_function(HashFunction::Sha256).finalize().unwrap().generate();
    let result_base_512 = TOTPBuilder::new().base32_key(&key_base).hash_function(HashFunction::Sha512).finalize().unwrap().generate();
    println!("* base32 1: {}\nbase32 256: {}\nbase32 512: {}\n", result_base_1, result_base_256, result_base_512);
    //let key: &[u8] = b"5YKMFQL7SXCINTCS";
    //let result = TOTPBuilder::new().key(key).hash_function(HashFunction::Sha256).finalize().unwrap().generate();
}
