#![feature(trace_macros)]

macro_rules! block {
    ($xs:block) => {
        loop {
            break $xs;
        }
    };
}

fn main() {
    let rusty = true;
    let num = 3;

    /*
        loop {
            match rusty {
                true => {
                    if num % 2 == 1 {
                        println!("num {} is odd", num);
                        // early break in mathc branch
                        break;
                    }
                    println!("num {} is even", num);
                }
                false => println!("Nothing todo here."),
            }

            break;
        }
    */

    trace_macros!(true);
    match rusty {
        true => block!({
            if num % 2 == 1 {
                println!("num {} is odd", num);
                // early break in mathc branch
                break;
            }
            println!("num {} is even", num);
        }),
        false => println!("Nothing todo here."),
    }
    trace_macros!(false);

    println!("Done");
}
