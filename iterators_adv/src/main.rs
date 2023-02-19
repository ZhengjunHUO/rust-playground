use std::collections::BTreeMap;
use std::iter::{from_fn, successors};
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
    let mut groups = BTreeMap::new();
    groups.insert("Rust", vec!["rustic", "rusty", "rustacean"]);
    groups.insert("Fufu", vec!["fuku", "neko"]);
    groups.insert("Huo", vec!["foo", "bar", "baz"]);

    let targets = vec!["Huo", "Fufu"];
    assert_eq!(
        targets.iter().flat_map(|s| &groups[s]).collect::<Vec<_>>(),
        vec![&"foo", &"bar", &"baz", &"fuku", &"neko"]
    );

    // flatten
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
}
