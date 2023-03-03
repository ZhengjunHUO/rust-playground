// map Gregorian calendar to Chinese calendar approximately, accepted value 1900-2100
pub fn g2c(year: usize) -> Option<String> {
    if year < 1900 || year > 2100 {
        return None;
    }

    ["庚", "辛", "壬", "癸", "甲", "乙", "丙", "丁", "戊", "己"]
        .iter()
        .cycle()
        .zip(
            [
                "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
            ]
            .iter()
            .cycle(),
        )
        .take(year - 1900 + 1)
        .map(|(&t, &d)| t.to_owned() + d)
        .last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g2c() {
        assert_eq!(g2c(1870), None);
        assert_eq!(g2c(1963), Some(String::from("癸卯")));
        assert_eq!(g2c(1990), Some(String::from("庚午")));
        assert_eq!(g2c(1994), Some(String::from("甲戌")));
    }
}
