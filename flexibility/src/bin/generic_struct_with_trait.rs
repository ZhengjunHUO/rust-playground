struct Struct<T> {
    data: T,
}

// Introduce a trait here
trait Num {}

// Instead of impl Struct<_> for all possible types
// Implment once for the trait
impl<N: Num> Struct<N> {
    fn new(data: N) -> Self {
        Self { data }
    }
}

impl Num for u8 {}
impl Num for f64 {}

fn main() {
    // No need turbo fish any more
    let s1 = Struct::new(8_u8);
    println!("s1<u8>: {}", s1.data);
    let s2 = Struct::new(64_f64);
    println!("s2<u64>: {}", s2.data);
}
