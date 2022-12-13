use generics::Inspect;
use objects::{Cat, Kube, Point, PointX};
use std::fmt::Display;

mod objects;

fn max_val<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i;
        }
    }

    max
}

fn notify<T: Inspect>(obj: &T) {
    println!("[INFO] notify: Find an object: {}", obj.info());
}

fn notify_with_content<T: Inspect + Display>(obj: &T) {
    println!(
        "[INFO] notify_with_content: Find an object: {}\n[DEBUG] Content: {}",
        obj.info(),
        obj
    );
}

fn notify_sugar(obj: &impl Inspect) {
    println!("[INFO] notify_sugar: Find an object: {}", obj.info());
}

fn notify_duo<T, U>(obj1: &T, obj2: &U)
where
    T: Inspect + Display,
    U: Inspect,
{
    println!("[INFO] notify_duo: Find 1st object: {}", obj1.info());
    println!("[INFO] notify_duo: Find 2nd object: {}", obj2.info());
}

// fn notify_sugar_duo<T: Inspect>(obj1: &T, obj2: &T) {  // Not work for different types
fn notify_sugar_duo(obj1: &impl Inspect, obj2: &impl Inspect) {
    println!("[INFO] notify_sugar_duo: Find 1st object: {}", obj1.info());
    println!("[INFO] notify_sugar_duo: Find 2nd object: {}", obj2.info());
}

fn main() {
    let l1 = vec![25, 17, 33, 69, 40];
    let max1 = max_val(&l1);
    println!("The max in {:?} is {}", l1, max1);

    let l2 = vec!['r', 'u', 's', 't'];
    let max2 = max_val(&l2);
    println!("The max in {:?} is {}", l2, max2);

    let p1 = Point { x: 6, y: 8 };
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("p1.x: {}; p2.y: {}", p1.get_x(), p2.get_y());
    println!(
        "the distance between zero point and p2: {}",
        p2.dist_from_zero_point()
    );

    let p3 = PointX { x: "Rust", y: 3.14 };
    let p4 = PointX { x: 1, y: 'x' };
    let p5 = p3.melange(p4);
    println!("after the melange p5.x = {}, p5.y = {}", p5.x, p5.y);

    let k1 = Kube::new(String::from("k8s"), String::from("cilium"), 8, false);
    println!("k8s: {:?}", k1);
    notify_sugar(&k1);

    let c1 = Cat::new(String::from("fufu"), 6);
    notify(&c1);

    notify_sugar_duo(&k1, &c1);
    notify_with_content(&k1);

    // Not work with Cat, cause the Display trait is not implemented
    // notify_with_content(&c1);

    notify_duo(&k1, &c1);
}
