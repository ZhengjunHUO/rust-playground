#![feature(trace_macros)]
#![recursion_limit = "256"]

use test_macro::{json, Json};

fn main() {
    trace_macros!(true);
    let list = vec![4, 5, 6];
    println!("sum: {}", list.iter().sum::<u64>());
    assert_eq!(json!(null), Json::Null);
    trace_macros!(false);
}
