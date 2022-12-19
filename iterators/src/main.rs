use std::fmt::Display;

fn main() {
    // Test #1
    let prime = vec![2u32, 3, 5, 7, 11];
    print_all(&prime);

    let num = increment(&prime);
    println!("incremented prime: {:?}", num);
    println!("original prime: {:?}", prime);

    println!("filtered num: {:?}", can_be_divided_by(num, 4));
    // can't use num because num is moved in can_be_divided_by
    //println!("{:?}", num);

    // 使用iter()在&T上循环
    // Test #2-1
    let cats = vec!["fufu", "lulu", "shoushou"];
    // iter()获取的是ref，之后cats依然有效
    // 注意iterator中的元素是&str的ref
    let sum = cats.iter().map(|id: &&str| id.len()).fold(8, |acc, len| acc + len);
    assert_eq!(sum, 24);
    print_all(&cats);

    // Test #2-2
    let cat_with_point = [("fufu", 30), ("lulu", 50), ("shoushou", 25)];
    // 同理此处&(name, _)也是元组的ref
    let name_only = cat_with_point.iter().map(|&(name, _)| { name }).collect::<Vec<_>>();
    assert_eq!(name_only, cats);

    // Test #3 使用iter_mut()在&mut T上循环
    let mut cats_with_point = [
       [("fufu", 30), ("lulu", 50), ("shoushou", 25),],
       [("fuku", 63), ("luku", 10), ("naonao", 47),],
    ];
    let sorted_asc = cats_with_point.iter_mut().map(|tp| {
        tp.sort_by(|&a, &b| a.1.cmp(&b.1));
        tp
    }).collect::<Vec<_>>();

    println!("After sort: {:?}", sorted_asc);

    // Test #4 使用into_iter()在T上循环 (move)
    let cat_with_credit = vec![(String::from("fufu"), 30), (String::from("lulu"), 50), (String::from("shoushou"), 25)];
    //let name_moved = cat_with_credit.into_iter().map(|(name, _)| { name }).collect::<Vec<_>>();
    let name_moved = retrieve_name(cat_with_credit);
    assert_eq!(name_moved, cats);
    // Won't work because cat_with_credit is moved at into_iter() call
    //println!("{:?}", cat_with_credit);
}

fn print_all<T: Display>(prime: &Vec<T>)
where T: Display
{
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
    // collect(): take anything iterable, and turn it into a relevant collection
    let positive = prime.iter().map(|x| x + 1).collect();
    positive
}

fn can_be_divided_by(num: Vec<u32>, divisor: u32) -> Vec<u32> {
    // into_iter() take the ownership
    num.into_iter().filter(|n| n % divisor == 0).collect()
}

fn retrieve_name(v: Vec<(String, usize)>) -> Vec<String> {
    v.into_iter().map(|(name, _)| { name }).collect()
}
