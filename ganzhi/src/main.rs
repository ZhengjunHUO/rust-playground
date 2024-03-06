use chrono::Datelike;
use ganzhi::interactive::HourCompleter;
use inquire::{DateSelect, Text};
use lunardate::LunarDate;
use rust_ephemeris::lunnar::SolorDate;

fn main() {
    let start = chrono::NaiveDate::from_ymd_opt(1990, 6, 1).unwrap();
    let chosen = DateSelect::new("Your birthday in gregorian calendar: ")
        .with_default(start)
        .prompt()
        .unwrap();

    let year = chosen.year();
    let month = chosen.month();
    let day = chosen.day();
    let mut hour: f64 = 10.0;

    let hc = HourCompleter {
        hours: (1..25).map(|i| i.to_string()).collect(),
    };
    let resp = Text::new("Choose the hour: ")
        .with_autocomplete(hc)
        .prompt();

    match resp {
        Ok(h) => hour = h.parse::<f64>().unwrap(),
        Err(err) => println!("Error retrieving your response: {}", err),
    }

    println!("公曆: {}年{}月{}日{}時", year, month, day, hour);

    let d = SolorDate(year, month as i32, day as i32);
    let sz = d.sizhu(hour / 24.0);
    let l = LunarDate::from_solar_date(year, month, day).unwrap();
    println!(
        "農曆: {}年{}{}月{}日{}時",
        l.year(),
        if l.is_leap_month() { "闰" } else { "" },
        l.month(),
        l.day(),
        sz.3.zhi()
    );
    println!("八字: {} {} {} {}", sz.0, sz.1, sz.2, sz.3);
}
