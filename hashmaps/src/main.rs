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

    let bar = String::from("bar");
    let baz = String::from("baz");
    // bar exist, return bar's value
    let bar_val = dict.get(&bar).copied().unwrap_or(0);
    // baz not exist, return 0
    let baz_val = dict.get(&baz).copied().unwrap_or(0);

    println!("bar's value: {}", bar_val);
    println!("baz's value: {}", baz_val);

    let mut counter = HashMap::new();
    let text = "what is fufu ? cat ! fufu is a domestic cat .";

    for w in text.split_whitespace() {
        // if key w is not exist yet, create an entry with value = 0
        //let i: &mut i32 = counter.entry(w).or_insert(0);
        let i = counter.entry(w).or_insert(0);
        *i += 1;
    }

    println!("counter: {:?}", counter);
}
