#![feature(trace_macros)]
#![recursion_limit = "256"]

use test_macro::{
    capture_then_check_attribute, capture_then_check_tokens, capture_then_stringify,
    check_attribute, check_tokens, json, Json,
};

fn main() {
    trace_macros!(true);
    let list = vec![4, 5, 6];
    println!("sum: {}", list.iter().sum::<u64>());
    assert_eq!(json!(null), Json::Null);
    trace_macros!(false);

    println!(
        "Directly stringify seq of token trees: {:?}",
        stringify!(rusty(2 ^ (3 * 2 + (4))))
    );
    println!(
        "Capture input to a expr and stringify: {:?}",
        capture_then_stringify!(rusty(2 ^ (3 * 2 + (4))))
    );

    println!(
        "check_tokens!((rustacean)): {}\ncheck_tokens!(2 ^ 10): {}\ncheck_tokens!(1024): {}\n",
        check_tokens!((rustacean)),
        check_tokens!(2 ^ 10),
        check_tokens!(1024)
    );
    println!(
        "capture_then_check_tokens!((rustacean)): {}\ncapture_then_check_tokens!(2 ^ 10): {}\ncapture_then_check_tokens!(1024): {}\n",
        capture_then_check_tokens!((rustacean)),
        capture_then_check_tokens!(2 ^ 10),
        capture_then_check_tokens!(1024)
    );

    println!(
        "check_attribute!(#[macro_export]): {}\ncheck_attribute!(#[test]): {}\ncapture_then_check_attribute!(#[macro_export]): {}\ncapture_then_check_attribute!(#[test]): {}",
        check_attribute!(#[macro_export]),
        check_attribute!(#[test]),
        capture_then_check_attribute!(#[macro_export]),
        capture_then_check_attribute!(#[test]),
    );
}
