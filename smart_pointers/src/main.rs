use std::ops::Deref;

// tuple struct with 1 param
struct FakeBox<T>(T);

impl<T> FakeBox<T> {
    fn new(x: T) -> FakeBox<T> {
        FakeBox(x)
    }
}

impl<T> Deref for FakeBox<T> {
    type Target = T;

    // return a ref and allow compiler to know how to deref
    fn deref(&self) -> &Self::Target {
        // ref of the tuple's first value
        &self.0
    }
}

fn echo(s: &str) {
    println!("{s}");
}

fn main() {
    let a = 10;

    // reference pointing to the value of a
    let r = &a;
    // an instance of box pointing to a copied value of a
    let sp = Box::new(a);
    // use self defined box
    let b = FakeBox::new(a);

    assert_eq!(10, a);
    assert_eq!(10, *sp);
    assert_eq!(10, *r);
    assert_eq!(10, *b);
    // under the hood, *b is replaced by the following
    assert_eq!(10, *(b.deref()));

    let s = FakeBox::new(String::from("Rust rocks!"));
    // &FakeBox<String> =(deref)=> &String =(deref_coercion)=> &str
    echo(&s);
    // *s => *(s.deref()) => *(&String) => String
    // &String[..] => &str
    echo(&(*s)[..]);
}
