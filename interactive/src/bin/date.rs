fn main() {
    let today = chrono::Utc::now().naive_utc().date();
    let yesterday = today - chrono::Duration::days(1);
    println!("Today is {}", today.format("%Y%m%d"));
    println!("Yesterday is {}", yesterday.format("%Y%m%d"));
}
