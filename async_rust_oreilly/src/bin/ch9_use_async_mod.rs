use async_rust_oreilly::async_mod;
use std::time::Duration;

fn main() {
    println!("[main] Submit add ...");
    let key = async_mod::submit_add(4, 6).unwrap();
    println!("[main] Got key: {}", key);
    std::thread::sleep(Duration::from_secs(3));
    println!("[main] Retrieve add ...");
    let rslt = async_mod::retrieve_add(key).unwrap();
    println!("[main] Result is {}", rslt);
}
