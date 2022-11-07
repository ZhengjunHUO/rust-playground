fn main() {
    // str pointer on stack, content on heap
    let s1 = String::from("Hello");
 
    // move: s1 is not valid any more
    // try to print s1 return error
    //let s2 = s1;

    // deep copy
    let s2 = s1.clone();

    println!("{s1}, world!");
    println!("{s2} again!");

    // stack-only data: fixed size, deep copy by default
    let a = 3;
    let b = a;
    println!("{} == {}", a, b);

    // s1 moved into func, not valid any more
    let (s3, len) = get_len(s1);
    println!("{}'s length is {}", s3, len)

    // also works
    //let (s1, len) = get_len(s1);
    //println!("{}'s length is {}", s1, len)
}

fn get_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
