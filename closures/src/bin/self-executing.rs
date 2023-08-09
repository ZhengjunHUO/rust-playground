fn main() {
    let rusty = true;
    let num = 3;

    (|| {
        match rusty {
            true => {
                if num % 2 == 1 {
                    println!("num {} is odd", num);
                    // early break in mathc branch
                    return;
                }
                println!("num {} is even", num);
            }
            false => println!("Nothing todo here."),
        }
    })();

    println!("Done");
}
