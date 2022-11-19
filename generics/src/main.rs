use objects::{Kube, Cat};
use generics::Inspect;

mod objects;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

struct PointX<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointX<X1, Y1> {
    fn melange<X2, Y2>(self, other: PointX<X2, Y2>) -> PointX<X1, Y2> {
        PointX {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    fn dist_from_zero_point(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

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

    let p1 = Point { x: 6, y: 8 };
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("p1.x: {}; p2.y: {}", p1.get_x(), p2.get_y());
    println!("the distance between zero point and p2: {}", p2.dist_from_zero_point());

    let p3 = PointX { x: "Rust", y: 3.14 };
    let p4 = PointX { x: 1, y: 'x'};
    let p5 = p3.melange(p4);
    println!("after the melange p5.x = {}, p5.y = {}", p5.x, p5.y);

    let k1 = Kube::new(
        String::from("foo"),
        String::from("cilium"),
        8,
        false,
    );
    println!("[INFO] {}", k1.info());

    let c1 = Cat::new(
        String::from("fufu"),
        6,
    );
    println!("[INFO] {}", c1.info());
}
