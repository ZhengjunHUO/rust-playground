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

impl PartialEq for CatImplPartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

fn main() {
    let c1 = Cat { uuid: 1, name: String::from("Fufu"), weight: 6.4, };
    let c2 = Cat { uuid: 1, name: String::from("Fuku"), weight: 5.9, };
    assert_ne!(c1, c2);

    let cp1 = CatImplPartialEq { uuid: 1, name: String::from("Fufu") };
    let cp2 = CatImplPartialEq { uuid: 1, name: String::from("Fuku") };
    assert_eq!(cp1, cp2);
}
