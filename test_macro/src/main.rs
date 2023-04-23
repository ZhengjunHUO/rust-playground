#![feature(trace_macros)]
#![recursion_limit = "256"]

use test_macro::{
    capture_then_check_attribute, capture_then_check_tokens, capture_then_stringify,
    check_attribute, check_tokens, fetch_head_recusive, json, Json,
};

fn main() {
    trace_macros!(true);
    println!(fetch_head_recusive!(foo bar baz));
    trace_macros!(false);

    let list = vec![4, 5, 6];
    println!("sum: {}", list.iter().sum::<u64>());
    assert_eq!(json!(null), Json::Null);

    println!(
        "Directly stringify seq of token trees: {:?}",
        stringify!(rusty(2 ^ (3 * 2 + (4))))
    );
    println!(
        "Capture input to a expr and stringify: {:?}\n",
        capture_then_stringify!(rusty(2 ^ (3 * 2 + (4))))
    );

    println!("[DEBUG] Be able to examine the content, and match correctly.");
    println!(
        "check_tokens!((rustacean)): {}\ncheck_tokens!(2 ^ 10): {}\ncheck_tokens!(1024): {}\n",
        check_tokens!((rustacean)),
        check_tokens!(2 ^ 10),
        check_tokens!(1024)
    );
    println!(
        "[DEBUG] Parsed the input to an AST node first, then impossible to examine the content."
    );
    println!(
        "capture_then_check_tokens!((rustacean)): {}\ncapture_then_check_tokens!(2 ^ 10): {}\ncapture_then_check_tokens!(1024): {}\n",
        capture_then_check_tokens!((rustacean)),
        capture_then_check_tokens!(2 ^ 10),
        capture_then_check_tokens!(1024)
    );

    println!("[DEBUG] Except using tt to capture that content.");
    println!(
        "check_attribute!(#[macro_export]): {}\ncheck_attribute!(#[test]): {}\n\ncapture_then_check_attribute!(#[macro_export]): {}\ncapture_then_check_attribute!(#[test]): {}",
        check_attribute!(#[macro_export]),
        check_attribute!(#[test]),
        capture_then_check_attribute!(#[macro_export]),
        capture_then_check_attribute!(#[test]),
    );
}
