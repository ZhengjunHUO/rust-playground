use deref::Pool;
use std::fmt::Display;

fn print_all(v: &str) {
    println!("{:?}", v);
}

fn print_generic<T: Display>(v: T) {
    println!("{}", v);
}

fn main() {
    let pool_str = Pool::new(vec!["foo", "bar", "fufu", "baz"], 2);
    print_all(&pool_str);
    print_generic(&*pool_str);
    print_generic(&pool_str as &str);
}
