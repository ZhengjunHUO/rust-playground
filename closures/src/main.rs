use crate::functraits::give_five_to;
use crate::functraits::huo_say;
use crate::functraits::repeat;
use crate::role::sort_roles;
use crate::role::Hero;
use std::collections::HashMap;
use std::thread;

mod functraits;
mod role;

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
}

impl City {
    fn get_stat(&self, stat: &Statistics) -> i64 {
        let result = match stat.hdi.get(&self.name) {
            Some(&n) => n,
            _ => 0,
        };
        result
    }
}

struct Statistics {
    hdi: HashMap<String, i64>,
}

fn city_sort_incr(cs: &mut Vec<City>) {
    cs.sort_by_key(|city| city.population);
}

fn city_sort_stat_desc(cs: &mut Vec<City>, stat: &Statistics) {
    cs.sort_by_key(|city| -city.get_stat(stat));
}

fn increment_func(x: u32) -> u32 {
    x + 1
}

fn main() {
    let mut x: u32 = 0;
    // 三种声明closure的方式
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
        Hero {
            attack: 56,
            defense: 77,
        },
        Hero {
            attack: 78,
            defense: 89,
        },
        Hero {
            attack: 61,
            defense: 63,
        },
        Hero {
            attack: 78,
            defense: 59,
        },
    ];
    // array to mutable slice
    sort_roles(&mut roles[..]);

    // a "fn" example
    let double_c = |x| x * 2;
    assert_eq!(give_five_to(double_c), 10);
    assert_eq!(give_five_to(double_c), 10);

    // a "fnMut" example
    let mut val: usize = 256;
    //let double_val_c = || val *= 2;
    repeat(|| val *= 2);
    assert_eq!(val, 1024);
    repeat(|| val *= 2);
    assert_eq!(val, 4096);

    // a "fnOnce" example
    let s = String::from("Rust rocks");

    let move_s = move || s;
    //huo_say(move || s);
    huo_say(move_s);
    // Can't call it again because closure move_s is moved
    //huo_say(move_s);

    // sort cities by populations
    let mut cs = vec![
        City {
            name: "foo".to_string(),
            population: 20000,
        },
        City {
            name: "bar".to_string(),
            population: 5000,
        },
        City {
            name: "fufu".to_string(),
            population: 12000,
        },
    ];
    city_sort_incr(&mut cs);
    println!("Cities after sort: {:?}", cs);

    let dict = Statistics {
        hdi: HashMap::from([
            ("foo".to_string(), 75),
            ("bar".to_string(), 87),
            ("baz".to_string(), 62),
        ]),
    };
    city_sort_stat_desc(&mut cs, &dict);
    println!("Cities after sort: {:?}", cs);
}

// 一个fn的例子
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

// 一个fnMut的例子
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

// 一个fnOnce的例子
fn move_ownership() {
    let prime = vec![2, 3, 5];
    println!("Define a prime list: {:?}", prime);

    // value moved into closure, closure used as thread's arg
    thread::spawn(move || println!("[In thread] prime list: {:?}", prime))
        .join()
        .unwrap();

    // value moved, won't compile
    //println!("[After closure] prime list: {:?}", prime);
}
