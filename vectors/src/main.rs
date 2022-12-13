#[derive(Debug)]
struct Element {
    value: i32,
}

fn main() {
    // need to specify the type
    //let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3, 4];
    v.push(5);

    let i = &v[3];
    //v.push(6);  // i is a immu borrow, can't do a mutable borrow here

    // use get to return a Option<T>
    // won't panic if out of index
    let j = v.get(8);
    match j {
        Some(j) => println!("The value is: {}", j),
        None => println!("Out of range"),
    }

    // i32 implements the `Copy` trait, can use without &
    // let i = v[3];  // in this case a following v.push will pass

    println!("Vector v: ");
    for (idx, k) in v.iter().enumerate() {
        println!("    v[{}]: {}", idx, k);
    }

    println!("Get v[3]: {}", i);

    // Vector of struct
    let mut elements = vec![Element { value: 10 }, Element { value: 20 }];
    // struct doesn't implement the `Copy` trait by default, should use reference
    //let e: &Element = &elements[1];
    println!("Vector elements: {:?}", elements);

    let e = &mut elements[0];
    println!("e's value: {:?}", e.value);

    println!("Update elements[0] ...");
    e.value = 30;

    println!("Vector elements:");
    // use reference, or value will be moved
    for elem in &elements {
        println!("    {:?}", elem);
    }

    println!("Update elements by multiplied by 10 ...");
    // use reference, or value will be moved
    for elem in &mut elements {
        elem.value *= 10;
    }
    println!("Vector elements: {:?}", elements);
}
