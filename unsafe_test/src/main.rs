use std::slice;
use unsafe_test::foreign::print_env_vars;

static mut COUNTER: u32 = 99;

extern "C" {
    fn abs(input: i32) -> i32;
    fn rand() -> u32;
    fn srand(seed: u32);
}

fn main() {
    deref_raw_pointer();
    split_ref();
    use_c_lib();
    increm_counter(1);
    raw_ptr_distance();

    print_env_vars();
    // Only dereferencing a raw pointer is unsafe.
    assert!(!option_to_raw_ptr(Some(&8)).is_null());
    assert!(option_to_raw_ptr::<usize>(None).is_null());
}

fn increm_counter(n: u32) {
    unsafe {
        println!("COUNTER before operation: {COUNTER}");
        COUNTER += n;
        println!("COUNTER updated: {COUNTER}");
    }
}

fn use_c_lib() {
    let num = -1;
    unsafe {
        println!("Abs of {} is {}", num, abs(num));
        srand(179);
        println!("Generate a random: {}", rand());
    }
}

// safe abstraction to the unsafe code
fn split_at_mut(list: &mut [i32], offset: usize) -> (&mut [i32], &mut [i32]) {
    let l = list.len();
    // Get list's raw pointer
    let pos = list.as_mut_ptr();
    assert!(offset <= l);

    unsafe {
        (
            slice::from_raw_parts_mut(pos, offset),
            slice::from_raw_parts_mut(pos.add(offset), l - offset),
        )
    }
}

fn split_ref() {
    let mut list = vec![1, 2, 3, 4];
    // Get its mutable slice
    let r = &mut list[..];

    // Use slice's safe method (include an unsafe func)
    //let (half1, half2) = r.split_at_mut(2);
    let (half1, half2) = split_at_mut(r, 2);

    assert_eq!(half1, &mut [1, 2]);
    assert_eq!(half2, &mut [3, 4]);
}

fn deref_raw_pointer() {
    let mut n = 10;
    let rp = &n as *const i32;
    let rp_m = &mut n as *mut i32;

    unsafe {
        *rp_m += *rp;
        assert_eq!(*rp, 20);
        assert_eq!(*rp_m, 20);
    }
}

fn option_to_raw_ptr<T>(o: Option<&T>) -> *const T {
    match o {
        Some(v) => v as *const T,
        None => std::ptr::null(),
    }
}

fn raw_ptr_distance() {
    let list = vec!["huo", "rustacean", "fufu"];
    let head: *const &str = &list[0];
    let tail: *const &str = &list[list.len() - 1];
    unsafe {
        assert_eq!(tail.offset_from(head), 2);
        assert_eq!(head.offset_from(tail), -2);
        assert_eq!(head.add(2), tail);
    }
}
