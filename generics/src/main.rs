fn max_val<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i;
        }
    }

    max
}

fn main() {
    let l1 = vec![25, 17, 33, 69, 40];
    let max1 = max_val(&l1);
    println!("The max in {:?} is {}", l1, max1);

    let l2 = vec!['r', 'u', 's', 't'];
    let max2 = max_val(&l2);
    println!("The max in {:?} is {}", l2, max2);
}
