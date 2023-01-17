#![feature(trace_macros)]

fn main() {
    trace_macros!(true);
    assert_eq!(10*10*10 + 9*9*9, 12*12*12 + 1*1*1);
    println!("Hello Rust!");
    trace_macros!(false);
}
