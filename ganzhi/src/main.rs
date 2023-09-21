use chrono::Datelike;
use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::error::CustomUserError;
use inquire::{DateSelect, Text};
use lunardate::LunarDate;
use rust_ephemeris::lunnar::SolorDate;

#[derive(Clone)]
pub struct HourCompleter {
    hours: Vec<String>,
}

impl HourCompleter {
    fn filter_candidates(&self, input: &str) -> Vec<String> {
        let pattern = input.to_lowercase();

        self.hours
            .clone()
            .into_iter()
            .filter(|s| s.starts_with(&pattern))
            .collect()
    }
}

impl Autocomplete for HourCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        Ok(self.filter_candidates(input))
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => {
                let list = self.filter_candidates(input);
                if list.len() == 0 {
                    Replacement::None
                } else {
                    Replacement::Some(list[0].clone())
                }
            }
        })
    }
}

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

    let ac = HourCompleter {
        hours: (1..25).into_iter().map(|i| i.to_string()).collect(),
    };
    let resp = Text::new("Choose the hour: ")
        .with_autocomplete(ac)
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
