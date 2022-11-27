use std::ops::Deref;
use std::rc::Rc;
use crate::List::*;

// tuple struct with 1 param
#[derive(Debug)]
struct FakeBox<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> FakeBox<T> {
    fn new(x: T) -> FakeBox<T> {
        FakeBox(x)
    }
}

impl<T: std::fmt::Debug> Deref for FakeBox<T> {
    type Target = T;

    // return a ref and allow compiler to know how to deref
    fn deref(&self) -> &Self::Target {
        // ref of the tuple's first value
        &self.0
    }
}

impl<T: std::fmt::Debug> Drop for FakeBox<T> {
    fn drop(&mut self) {
        println!("[{:?}]: drop called", self);
    }
}


// Test ref counter
enum List {
    Node(u32, Rc<List>),
    Nil,
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


    let l = Rc::new(Node(10, Rc::new(Node(100, Rc::new(Nil)))));
    println!("After let l, l's counter = {}", Rc::strong_count(&l));
    // Rc::clone don't do deep copy, increse ref count only
    let l1 = Node(0, Rc::clone(&l));
    println!("After let l1, l's counter = {}", Rc::strong_count(&l));
    {
        let l2 = Node(1, Rc::clone(&l));
        println!("After let l2, l's counter = {}", Rc::strong_count(&l));
    }
    // the implementation of the Drop trait decreases the ref count automatically
    // when an Rc<T> value goes out of scope.
    println!("After l2 out of scope, l's counter = {}", Rc::strong_count(&l));
}
