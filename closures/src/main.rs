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
}
