const MILLION: u32 = 1000 * 1000;

fn main() {
    // constant
    println!("const MILLION's value: {}", MILLION);

    const BILLION: u32 = MILLION * 1000;
    println!("const BILLION's value: {}", BILLION);


    // mutable variable
    let mut m = 5;
    println!("mutable var x's value: {}", m);

    m = 6;
    println!("  => x's value changed to: {}", m);


    // shadowed immutable variable
    let im = 2;
    let im = im + 1;

    {
        let im = "I'm a string";
        println!("inner scope's immutable value: {}", im);
    } 

    println!("outer scope's immutable value: {}", im);
}
