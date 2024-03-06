#![feature(trace_macros)]
#![recursion_limit = "256"]

use std::env;
use test_macro::{
    capture_then_check_attribute, capture_then_check_tokens, capture_then_stringify,
    check_attribute, check_tokens, expr_len, json, pop_head, pop_tail, print_fibo,
    rendered_from_env, vec_string, Json,
};

#[allow(dead_code)]
struct Config {
    id: String,
    attr: Attribute,
}

struct Attribute {
    access_key: Option<String>,
    secret_key: Option<String>,
}

impl Config {
    pub fn render(&mut self) {
        trace_macros!(true);
        rendered_from_env!(self.attr.access_key, "GCS_ACCESS_KEY");
        rendered_from_env!(self.attr.secret_key, "GCS_SECRET_KEY");
        trace_macros!(false);
    }
}

fn main() {
    let mut config = Config {
        id: "Test".into(),
        attr: Attribute {
            access_key: None,
            secret_key: None,
        },
    };

    config.render();
    println!(
        "access_key: {:?}, secret_key: {:?}",
        config.attr.access_key, config.attr.secret_key
    );

    //trace_macros!(true);
    println!(pop_head!(foo bar baz));
    println!(pop_tail!(foo bar baz));
    //trace_macros!(false);

    let list = [4, 5, 6];
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

    println!("[DEBUG] Print fibonacci sequence:");
    let f = print_fibo![a[n]: u64 = 0, 1 => a[n-1] + a[n-2]];
    for elem in f.take(15) {
        println!("{}", elem)
    }

    let rslt = vec_string!["foo", "bar", "baz"];
    println!("{:?}", rslt);
}
