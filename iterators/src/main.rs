use std::any::type_name;
use std::fmt::Display;

fn type_of<T>(_: &T) {
    println!("    {}", type_name::<T>())
}

fn test_for() {
    //let vals = vec![1,2,3,4,5];
    let vals = vec![
        String::from("huo"),
        String::from("rust"),
        String::from("rocks"),
    ];
    for v in &vals {
        println!("{}", v);
    }

    println!("{:?}", vals);
}

fn collect_into_vec<I: Iterator>(source: I) -> Vec<I::Item> {
    let mut rslt = Vec::new();

    for v in source {
        rslt.push(v)
    }

    rslt
}

fn main() {
    // Test #1
    let prime = vec![2u32, 3, 5, 7, 11];
    println!("=> Type of prime is: ");
    type_of(&prime);
    //print_all(&prime);

    let num = increment(&prime);
    println!("incremented prime: {:?}", num);
    println!("original prime: {:?}", prime);

    println!("filtered num: {:?}", can_be_divided_by(num, 4));
    // can't use num because num is moved in can_be_divided_by
    //println!("{:?}", num);

    // 使用iter()在&T上循环
    // Test #2-1
    let cats = vec!["fufu", "lulu", "shoushou"];
    println!("=> Type of cats is: ");
    type_of(&cats);
    // iter()获取的是ref，之后cats依然有效
    // 注意iterator中的元素是&str的ref
    let sum = cats
        .iter()
        .map(|id: &&str| id.len())
        .fold(8, |acc, len| acc + len);
    assert_eq!(sum, 24);
    print_all(&cats);

    // Test #2-2
    let cat_with_point = [("fufu", 30), ("lulu", 50), ("shoushou", 25)];
    println!("=> Type of cat_with_point is: ");
    type_of(&cat_with_point);
    // 同理此处&(name, _)也是元组的ref
    let name_only = cat_with_point
        .iter()
        .map(|&(name, _)| name)
        .collect::<Vec<_>>();
    assert_eq!(name_only, cats);

    // Test #3 使用iter_mut()在&mut T上循环
    let mut cats_with_point = [
        [("fufu", 30), ("lulu", 50), ("shoushou", 25)],
        [("fuku", 63), ("luku", 10), ("naonao", 47)],
    ];
    println!("=> Type of cats_with_point is: ");
    type_of(&cats_with_point);
    let sorted_asc = cats_with_point
        .iter_mut()
        .map(|tp| {
            tp.sort_by(|&a, &b| a.1.cmp(&b.1));
            tp
        })
        .collect::<Vec<_>>();

    println!("After sort: {:?}", sorted_asc);

    // Test #4 使用into_iter()在T上循环 (move)
    let cat_with_credit = vec![
        (String::from("fufu"), 30),
        (String::from("lulu"), 50),
        (String::from("shoushou"), 25),
    ];
    println!("=> Type of cat_with_credit is: ");
    type_of(&cat_with_credit);
    //let name_moved = cat_with_credit.into_iter().map(|(name, _)| { name }).collect::<Vec<_>>();
    let name_moved = retrieve_name(cat_with_credit);
    println!("=> Type of name_moved is: ");
    type_of(&name_moved);
    assert_eq!(name_moved, cats);
    // Won't work because cat_with_credit is moved at into_iter() call
    //println!("{:?}", cat_with_credit);

    // Test #5 About for
    test_for();

    // Test #6
    let cats_bis = name_moved.clone().into_iter().collect::<Vec<_>>();
    println!("=> Type of cats_bis is: ");
    type_of(&cats_bis);

    // Vec<String>, Vec<String>
    assert_eq!(name_moved, cats_bis);
    // Vec<&str>, Vec<String> ?!
    assert_eq!(cats, cats_bis);

    let cat_first_two = name_moved.clone().into_iter().take(2).collect::<Vec<_>>();
    let cat_first_two_bis = name_moved.iter().take(2).cloned().collect::<Vec<_>>();
    let cat_first_two_ter = name_moved.iter().take(2).cloned().collect::<Vec<_>>();
    assert_eq!(cat_first_two, cat_first_two_bis);
    assert_eq!(cat_first_two, cat_first_two_ter);

    // Test #7 Zip
    let z1 = vec![1, 2, 3];
    let z2 = vec![10, 20, 30, 40];

    // z1, z2 consumed; element in rslt is (i32, i32)
    // use iter() to get ref (avoid move); element in rslt is (&i32, &i32)
    // z1.into_iter().zip(z2.iter()); => (i32, &i32)
    let rslt = z1.into_iter().zip(z2);
    // rslt consumed by for loop
    // for loop syntax is actually sugar for iterators
    /*
    for elem in rslt {
        type_of(elem);
        println!("{:?}", elem);
    }
    */
    assert_eq!(collect_into_vec(rslt), vec![(1, 10), (2, 20), (3, 30)]);
}

fn print_all<T: Display>(prime: &[T])
where
    T: Display,
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

fn increment(prime: &[u32]) -> Vec<u32> {
    // collect(): take anything iterable, and turn it into a relevant collection
    let positive = prime.iter().map(|x| x + 1).collect();
    positive
}

fn can_be_divided_by(num: Vec<u32>, divisor: u32) -> Vec<u32> {
    // into_iter() take the ownership
    num.into_iter().filter(|n| n % divisor == 0).collect()
}

fn retrieve_name(v: Vec<(String, usize)>) -> Vec<String> {
    v.into_iter().map(|(name, _)| name).collect()
}
