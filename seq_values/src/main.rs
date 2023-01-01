fn main() {
    // 关于Array
    // (1)
    let mut b1 = [true; 3];
    b1[1] = false;
    assert_eq!(b1.len(), 3);
    assert_eq!(b1, [true, false, true]);

    // (2)
    let mut a1 = [1, 10, 6, 3, 9];
    // 很多有用的方法是在slice上实现，array可直接调用，因为
    // 隐式地将array转换为&mut a1[..], 如下r2所示
    a1.sort();
    assert_eq!(a1, [1, 3, 6, 9, 10]);

    let mut a2 = [24, 3, 11, 8, 6];
    let r2: &mut [i32] = &mut a2[..];
    //let r2 = &mut a2;
    r2.sort();
    assert_eq!(a2, [3, 6, 8, 11, 24]);

    // 关于Vector
    // (1)
    let mut v1 = vec![2, 3, 5];
    assert_eq!(v1.pop(), Some(5));
    v1.push(7);
    assert_eq!(v1.iter().product::<u32>(), 42);

    // (2)
    // 类似array的声明法
    let mut b2 = vec![true; 3];
    b2[0] = false;
    assert_eq!(b2, vec![false, true, true]);

    // (3)
    // collect()需要显式指出类型
    let mut v2: Vec<u32> = (0..6).collect();
    // 类似array的隐式转换，变成slice后调用方法
    v2.reverse();
    assert_eq!(v2, vec![5, 4, 3, 2, 1, 0]);

    let mut v3: Vec<u32> = (0..6).collect();
    let r3 = &mut v3[..];
    r3.reverse();
    assert_eq!(v3, vec![5, 4, 3, 2, 1, 0]);

    // 关于slice
    // 隐式地转换&[T; N]为&[T]
    let sa = &a2;
    // 隐式地转换&Vec<T>为&[T]
    let sv = &v3;
    // Array和Vec都可以作为参数传递给print_all
    print_all(sa);
    print_all(sv);
}

// 接受一个slice ref
fn print_all<T>(s: &[T])
where
    T: std::fmt::Display,
{
    for i in s {
        println!("{}", i);
    }
}
