use std::collections::{BTreeMap, BTreeSet};
use std::iter::{from_fn, once, repeat, successors, Iterator, Peekable};
use std::str::FromStr;

fn main() {
    // #1 Generate a vec with random value
    let rand_pool: Vec<u8> = from_fn(|| Some(rand::random::<u8>())).take(5).collect();
    println!("rand_pool: {:?}", rand_pool);

    // #2 Generate a vec, initialized with 0-9
    let incr_pool: Vec<u32> = successors(Some(0), |&n| Some(n + 1)).take(10).collect();
    assert_eq!(incr_pool, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // #3 drain moved a part out of a string and build an iterator
    let mut sayit = "RustRocks".to_string();
    let part = String::from_iter(sayit.drain(1..5));
    assert_eq!(sayit, "Rocks");
    assert_eq!(part, "ustR");

    // #4 map & filter
    let text1 = "   foo     \n     Rust \n  fufu  \nbar ".to_string();
    let trimed: Vec<&str> = text1
        .lines()
        .map(str::trim)
        .filter(|s| *s != "Rust")
        .collect();
    assert_eq!(trimed, vec!["foo", "fufu", "bar"]);
    println!("Ownership not taken: {:?}", text1);

    // #5 filter_map
    let text2 = "foo123 \n  -45 fufu\n bar 67 baz";
    let fm: Vec<i32> = text2
        .split_whitespace()
        .filter_map(|n| i32::from_str(n).ok())
        .collect();
    assert_eq!(fm, vec![-45, 67]);

    // #6 flat_map & flatten
    // flat_map #1
    let mut groups = BTreeMap::new();
    groups.insert("Rust", vec!["rustic", "rusty", "rustacean"]);
    groups.insert("Fufu", vec!["fuku", "neko"]);
    groups.insert("Huo", vec!["foo", "bar", "baz"]);

    let targets = vec!["Huo", "Fufu"];
    assert_eq!(
        targets.iter().flat_map(|s| &groups[s]).collect::<Vec<_>>(),
        vec![&"foo", &"bar", &"baz", &"fuku", &"neko"]
    );

    // flatten #1
    assert_eq!(
        groups.values().cloned().flatten().collect::<Vec<_>>(),
        vec![
            "fuku",
            "neko",
            "foo",
            "bar",
            "baz",
            "rustic",
            "rusty",
            "rustacean"
        ]
    );

    // flatten #2
    assert_eq!(
        vec![None, None, Some("Rust"), Some("Huo"), None, Some("Rocks")]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        vec!["Rust", "Huo", "Rocks"]
    );

    // flat_map #2
    assert_eq!(
        "Learn Rust"
            .chars()
            .flat_map(char::to_uppercase)
            .collect::<String>(),
        "LEARN RUST".to_string()
    );

    // #7 take_while & by_ref & skip_while
    let mail = "To: Huo\r\nFrom: Fufu\r\n\r\nCoucou!";
    let mut lines = mail.lines();
    assert_eq!(
        lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>(),
        vec!["To: Huo", "From: Fufu"]
    );

    for l in lines {
        println!("Continue reading the mail's body: {}", l);
    }

    assert_eq!(
        mail.lines()
            .skip_while(|l| !l.is_empty())
            .skip(1)
            .collect::<Vec<_>>(),
        vec!["Coucou!"]
    );

    // #8 Peekable
    let mut p = "3840x2160".chars().peekable();
    assert_eq!(parse_num(&mut p), 3840);
    // Peekable上的其他方法（如next）知道调用peek方法的当前位置
    assert_eq!(p.next(), Some('x'));
    assert_eq!(parse_num(&mut p), 2160);
    assert_eq!(p.next(), None);

    // #9 DoubleEndedIterator & Reversible
    // 首尾有双指针l和r，如果l>r则开始返回None
    let mut dei = groups.iter();
    assert_eq!(dei.next().unwrap().0, &"Fufu");
    assert_eq!(dei.next_back().unwrap().0, &"Rust");
    assert_eq!(dei.next().unwrap().0, &"Huo");
    assert_eq!(dei.next_back(), None);
    assert_eq!(dei.next(), None);

    let mut targets_rev = targets.iter().rev();
    assert_eq!(targets_rev.next(), Some(&"Fufu"));

    // #10 chain
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
    assert_eq!(
        (2..4)
            .chain(BTreeSet::from([5, 7, 11, 13]))
            .chain(vec![17, 19, 23])
            .collect::<Vec<i32>>(),
        primes
    );

    // #11 Enumerate && zip
    // zip是enumerate的通用形式
    let indexed_primes = primes.iter().enumerate().collect::<Vec<_>>();
    let zipped_primes = (0..9).zip(primes.iter()).collect::<Vec<_>>();
    assert_eq!(indexed_primes, zipped_primes);

    // #12 repeat, once, cycle, etc.
    // 用m3替换3的倍数，用m5替换5的倍数
    let m3s_m5s = repeat("")
        .take(2)
        .chain(once("m3"))
        .cycle()
        .zip(repeat("").take(4).chain(once("m5")).cycle());

    let m3_m5 = (1..50).zip(m3s_m5s).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (m3, m5)) => format!("{}{}", m3, m5),
    });

    for m in m3_m5.take(10) {
        println!("{}", m);
    }

    // #13 Implement Iterator
    let rg = RangeU32 { begin: 3, end: 6 };

    // the standard library provides a blanket implementation of IntoIterator for every type that implements Iterator
    for i in rg {
        print!("{} ", i);
    }
    println!();
}

struct RangeU32 {
    begin: u32,
    end: u32,
}

impl Iterator for RangeU32 {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.begin >= self.end {
            return None;
        }

        let result = Some(self.begin);
        self.begin += 1;
        result
    }
}

fn parse_num<I>(p: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut result = 0;
    loop {
        match p.peek() {
            Some(v) if v.is_digit(10) => {
                result = result * 10 + v.to_digit(10).unwrap();
            }
            _ => return result,
        }
        p.next();
    }
}
