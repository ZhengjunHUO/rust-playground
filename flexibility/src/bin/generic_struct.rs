struct Struct<T> {
    data: T,
}

impl Struct<u8> {
    fn new(data: u8) -> Struct<u8> {
        Self { data }
    }
}

impl Struct<f64> {
    fn new(data: f64) -> Struct<f64> {
        Self { data }
    }
}

fn main() {
    /*
    Won't work, multiple applicable items in scope
        multiple `new` found
    */
    //let s1: Struct<u8> = Struct::new(0_u8);
    //let s2: Struct<f64> = Struct::new(0_f64);

    let s1 = Struct::<u8>::new(8_u8);
    let s2 = Struct::<f64>::new(64_f64);
    println!("s1<u8>: {}", s1.data);
    println!("s2<u64>: {}", s2.data);
}
