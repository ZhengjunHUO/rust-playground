fn main() {
    let prime = vec![2u32, 3, 5, 7, 11];
    print_all(&prime);

    let num = increment(&prime);
    println!("incremented prime: {:?}", num);
    println!("original prime: {:?}", prime);

    println!("filtered num: {:?}", can_be_divided_by(num, 4));
}

fn print_all(prime: &Vec<u32>) {
    let mut prime_itr = prime.iter();

    // next() change iterator's state, so prime_itr should be mut
    // value returned by next() is immu ref
    println!("next get: {}", prime_itr.next().unwrap());

    // prime_itr don't need to be mut because for loop take ownership
    for v in prime_itr {
        println!("for loop get: {}", v);
    }

    // won't compile, prime_itr moved in for loop (ownership is taken)
    //println!("next get: {}", prime_itr.next().unwrap());
}

fn increment(prime: &Vec<u32>) -> Vec<u32> {
    let positive = prime.iter().map(|x| x + 1).collect();
    positive
}

fn can_be_divided_by(num: Vec<u32>, divisor: u32) -> Vec<u32> {
    // into_iter() take the ownership
    num.into_iter().filter(|n| n%divisor == 0).collect()
}
