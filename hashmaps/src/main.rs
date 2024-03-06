use std::collections::HashMap;

fn main() {
    let mut dict = HashMap::new();
    dict.insert(String::from("foo"), 5);
    dict.insert(String::from("bar"), 8);
    // overwrite bar
    dict.insert(String::from("bar"), 18);
    // foo exist, do nothing
    dict.entry(String::from("foo")).or_insert(15);
    // huo not exist, insert entry
    dict.entry(String::from("huo")).or_insert(15);

    for (k, v) in &dict {
        println!("{}: {}", k, v);
    }

    let val1 = String::from("val1");
    let val2 = String::from("val2");
    // val1 exist, return val1's value
    let val1_val = dict.get(&val1).copied().unwrap_or(0);
    // val2 not exist, return 0
    let val2_val = dict.get(&val2).copied().unwrap_or(0);

    println!("val1's value: {}", val1_val);
    println!("val2's value: {}", val2_val);

    let mut counter = HashMap::new();
    let text = "what is fufu ? cat ! fufu is a domestic cat .";

    for w in text.split_whitespace() {
        // if key w is not exist yet, create an entry with value = 0
        //let i: &mut i32 = counter.entry(w).or_insert(0);
        let i = counter.entry(w).or_insert(0);
        *i += 1;
    }

    println!("counter: {:?}", counter);

    let mut dict_test: HashMap<String, (u64, u64)> = HashMap::new();
    dict_test.insert("Rustacean".to_owned(), (6, 9));
    assert_eq!(dict_test.get("rusty"), None);
    assert_eq!(dict_test.get("Rustacean"), Some(&(6, 9)));
    match dict_test.get("Rustacean") {
        None => unreachable!(),
        Some(&t) => assert_eq!(t, (6, 9)),
    }
}
