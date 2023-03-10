#![feature(trace_macros)]

fn main() {
    trace_macros!(true);
    let list = vec![4, 5, 6];
    println!("sum: {}", list.iter().sum::<u64>());
    trace_macros!(false);
}
