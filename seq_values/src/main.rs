fn main() {
    // 关于Array
    // (1) 
    let mut b = [true; 3];
    b[1] = false;
    assert_eq!(b.len(), 3);
    assert_eq!(b, [true, false, true]);

    // (2) 
    let mut a1 = [1, 10, 6, 3, 9];
    // 很多有用的方法是在slice上实现，array可直接调用，因为
    // 隐式地将array转换为&mut a1[..], 如下r2所示
    a1.sort();
    assert_eq!(a1, [1, 3, 6, 9, 10]);

    let mut a2 = [24, 3, 11, 8, 6];
    let r2 = &mut a2[..];
    r2.sort();
    assert_eq!(a2, [3, 6, 8, 11, 24]);
}
