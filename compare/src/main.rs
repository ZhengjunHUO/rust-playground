use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Cat {
    uuid: u32,
    name: String,
    // 因为f32没有impl Eq所以struct不能derive Eq
    weight: f32,
}

#[derive(Debug)]
struct CatImplPartialEq {
    uuid: u32,
    name: String,
}

// 被Eq和PartialOrd需要
impl PartialEq for CatImplPartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

// Eq trait has no method
impl Eq for CatImplPartialEq {}


// 需要Eq和PartialOrd
impl Ord for CatImplPartialEq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

// If type is Ord, can implement partial_cmp by using cmp
// 被Ord需要，却可以调用Ord中的cmp来实现
impl PartialOrd for CatImplPartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let c1 = Cat { uuid: 1, name: String::from("Fufu"), weight: 6.4, };
    let c2 = Cat { uuid: 1, name: String::from("Fuku"), weight: 5.9, };
    assert_ne!(c1, c2);

    let cp1 = CatImplPartialEq { uuid: 1, name: String::from("Fufu") };
    let cp2 = CatImplPartialEq { uuid: 1, name: String::from("Fuku") };
    let cp3 = CatImplPartialEq { uuid: 8, name: String::from("FukuNeko") };
    assert_eq!(cp1, cp2);
    assert_eq!(cp1 < cp3, true);
}
