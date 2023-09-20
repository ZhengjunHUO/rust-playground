use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let today = chrono::Utc::now().naive_utc().date();
    let yesterday = today - chrono::Duration::days(1);
    println!("Today is {}", today.format("%Y%m%d"));
    println!("Yesterday is {}", yesterday.format("%Y%m%d"));

    let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2023, 9, 20)
        .unwrap()
        .and_hms_opt(9, 57, 11)
        .unwrap();
    let delta = chrono::Utc::now().naive_utc() - dt;

    if delta.num_minutes() > 0 {
        println!("delta is {} mins", delta.num_minutes());
    } else {
        println!("delta is {} secs", delta.num_seconds());
    }
}
