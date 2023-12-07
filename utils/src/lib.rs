const UNITS: [&str; 5] = ["byte(s)", "KB", "MB", "GB", "TB"];

pub fn size_to_human_readable(size: usize) -> String {
    if size < 1024 {
        return format!("{} byte(s)", size);
    }

    let mut idx = 0;
    let mut rslt = size as f64;
    while rslt >= 1024.0 {
        rslt /= 1024.0;
        idx += 1;
    }

    format!("{:.2} {}", rslt, UNITS[idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_to_human_readable() {
        assert_eq!(size_to_human_readable(1024), "1.00 KB".to_owned());
        assert_eq!(size_to_human_readable(892), "892 byte(s)".to_owned());
        assert_eq!(size_to_human_readable(93111441), "88.80 MB".to_owned());
        assert_eq!(size_to_human_readable(4927586304), "4.59 GB".to_owned());
    }
}
