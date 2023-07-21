fn main() {
    let today = chrono::Utc::now().naive_utc().date();
    println!("Today is {}", today.format("%Y%m%d"));
}
