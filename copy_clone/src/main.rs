// Clone是Copy的supertrait: pub trait Copy: Clone { }
// 是Copy的东西必定是Clone

#[derive(Debug, Copy, Clone)]
struct DeriveCopyClone {
    a: u32,
    b: u32,
    //s并没有implement Copy所以本struct不能derive Copy
    //s: String,
}

#[derive(Debug, Clone)]
struct DeriveClone {
    a: u32,
    b: u32,
    s: String,
}

#[derive(Debug)]
struct Couple {
    a: u32,
    b: u32,
    s: String,
}

fn main() {
    let dcc1 = DeriveCopyClone { a: 6, b: 6 };
    // 因为derive了Copy, dcc1被隐式地复制到dcc2
    // 没有产生move
    let dcc2 = dcc1;
    println!("dcc1: {:?}, dcc2: {:?}", dcc1, dcc2);

    let dc1 = DeriveClone { a: 3, b: 4, s: String::from("Rust rocks!"), };
    // 只derive了Clone，所以需要显示地复制到dc2
    let dc2 = dc1.clone();
    println!("dc1: {:?}, dc2: {:?}", dc1, dc2);

    let c1 = Couple { a: 7, b: 8, s: String::from("Rust rocks!"), };
    // 没有derive，所以是move
    let c2 = c1;
    println!("c2: {:?}", c2);


    let s1 = String::from("Rust rocks!");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}
