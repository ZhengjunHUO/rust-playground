use std::thread;
use crate::role::Hero;
use crate::role::sort_roles;

mod role;

fn increment_func (x: u32) -> u32 { x + 1 }

fn main() {
    let mut x: u32 = 0;
    let increment_closure1 = |x: u32| -> u32 { x + 1 };
    let increment_closure2 = |x| { x + 1 };
    let increment_closure3 = |x| x + 1;

    x = increment_func(x);
    println!("Apply increment_func, get {}", x);

    x = increment_closure1(x);
    x = increment_closure2(x);
    x = increment_closure3(x);
    println!("Apply 3 closures, get {}", x);

    println!("=== Test immutable borrow ===");
    borrow();
    println!("=== Test mutable borrow ===");
    mut_borrow();
    println!("=== Test move ownership ===");
    move_ownership();

    println!("=== Test Fn trait ===");
    // array
    let mut roles = [
        Hero { attack: 56, defense: 77},
        Hero { attack: 78, defense: 89},
        Hero { attack: 61, defense: 63},
        Hero { attack: 78, defense: 59},
    ];
    // array to mutable slice
    sort_roles(&mut roles[..]);
}

fn borrow() {
    let prime = vec![2, 3, 5];
    println!("Before defining a closure: {:?}", prime);

    // immutable borrow => immutable binding
    let borrow_closure = || println!("[Inside closure] prime list: {:?}", prime);

    // immutable borrow occurs, multiple immu ref can coexist, will compile
    println!("[Before closure] prime list: {:?}", prime);
    borrow_closure();
    println!("[After  closure] prime list: {:?}", prime);
}

fn mut_borrow() {
    let mut prime = vec![2, 3, 5];
    println!("Before defining a closure: {:?}", prime);

    // mutable borrow => mutable binding
    let mut borrow_mut_closure = || prime.push(7);

    // immutable borrow occurs when there is already a mutable borrow, won't compile
    // println!("[Before closure] prime list: {:?}", prime);

    borrow_mut_closure();
    // mutable borrow end here, can print
    println!("[After  closure] prime list: {:?}", prime);
}

fn move_ownership() {
    let prime = vec![2, 3, 5];
    println!("Define a prime list: {:?}", prime);

    // value moved into closure, closure used as thread's arg
    thread::spawn(move || println!("[In thread] prime list: {:?}", prime)).join().unwrap();

    // value moved, won't compile
    //println!("[After closure] prime list: {:?}", prime);
}
