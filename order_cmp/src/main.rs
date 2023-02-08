use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq)]
struct Interval<T> {
    left_boundary: T,
    right_boundary: T,
}

impl<T: PartialOrd> PartialOrd<Self> for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.right_boundary <= other.left_boundary {
            Some(Ordering::Less)
        } else if self.left_boundary >= other.right_boundary {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

fn main() {
    let itv1 = Interval::<i32> {
        left_boundary: 20,
        right_boundary: 35,
    };
    let itv2 = Interval::<i32> {
        left_boundary: 12,
        right_boundary: 17,
    };
    let itv3 = Interval::<i32> {
        left_boundary: 20,
        right_boundary: 35,
    };
    let itv4 = Interval::<i32> {
        left_boundary: 29,
        right_boundary: 47,
    };
    assert!(itv2 < itv1);
    assert!(itv1 == itv3);
    assert!(!(itv4 < itv1));
    assert!(!(itv4 > itv1));
    assert!(!(itv4 == itv1));
}
