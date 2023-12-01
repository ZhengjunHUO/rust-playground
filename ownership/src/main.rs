use std::rc::Rc;

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
    println!("{}'s length is {}", s3, len);

    // also works
    //let (s1, len) = get_len(s1);
    //println!("{}'s length is {}", s1, len)

    // use immutable ref, avoid move back the origin string from the func
    let lenth = calc_len(&s3);
    println!("{}'s lengthw is {}", s3, lenth);

    // modify mutable ref in func
    let mut s4 = String::from("你好");

    // can have several immut refs
    let r3 = &s4;
    let r4 = &s4;
    println!("{} {}", r3, r4); // r3, r4 scope end here, ok to have mut ref

    // not the case for mut refs (avoid race condition)
    let r1 = &mut s4;
    modif_str_ch(r1); // r1's scope end here, ok to have another mutable ref

    let r2 = &mut s4;
    modif_str_en(r2);
    println!("After change: {}", s4);

    // 比较ref
    let ai = 8;
    let bi = 8;
    let ra = &ai;
    let rb = &bi;
    // ref指向的内容在数值上相同
    assert!(ra == rb);
    // 但是存储在不同的地址上
    assert!(!std::ptr::eq(ra, rb));

    // Rc也可以进行类似的比较
    let rc_str_a = Rc::new("huo".to_string());
    let rc_str_b = rc_str_a.clone();
    assert!(rc_str_a == rc_str_b);
    // 指向的是同一片区域
    assert!(Rc::ptr_eq(&rc_str_a, &rc_str_b));

    let rc_str_c = Rc::new("huo".to_string());
    assert!(rc_str_a == rc_str_c);
    // 值相同但是地址不同
    assert!(!Rc::ptr_eq(&rc_str_a, &rc_str_c));

    test_option();
}

fn get_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn modif_str_ch(s: &mut String) {
    s.push_str(", 世界!");
}

fn modif_str_en(s: &mut String) {
    s.push_str(" world!");
}

fn print(v: Option<&str>) {
    match v {
        Some(s) => println!("Got some {}", s),
        None => println!("Got None"),
    }
}

fn test_option() {
    let s = String::from("Rustacean");
    let v = Some(s.as_str());
    print(v);
    print(v);
    print(v);
}
