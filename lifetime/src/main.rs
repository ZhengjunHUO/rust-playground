use std::fmt::Display;

fn main() {
    let s1 = String::from("Hello, Rust!");
    let rslt;

    {
        let s2 = "huo";
        rslt = max_str(s1.as_str(), s2);

        // won't work, rslt's lifetime == s2's lifetime, end at inner }
        //let s2 = String::from("huo");
        //rslt = max_str(s1.as_str(), s2.as_str());
    }

    println!("result is: {}", rslt);

    let rslt2 = new_str();
    println!("result2 is: {}", rslt2);

    let rslt3 = max_str_2(s1.as_str(), "foo", "result3 is");
    println!("{}", rslt3);
}

// the lifetime of the reference in result must be the smaller lifetime of the two arguments
fn max_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn max_str_2<'a, T>(s1: &'a str, s2: &'a str, obj: T) -> &'a str
where
    T: Display,
{
    println!("{}:", obj);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn new_str() -> &'static str {
    let rslt = "fufu";
    rslt
}
