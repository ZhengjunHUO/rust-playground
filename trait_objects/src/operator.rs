use std::ops::{Mul, Add};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Binome {
    pub a: u32,
    pub b: u32,
}

pub struct Offset(pub u32);

impl Mul for Binome {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Self {
            a: self.a * o.a,
            b: self.b * o.b,
        }
    }
}

impl Add<Offset> for Binome {
    type Output = Self;

    fn add(self, o: Offset) -> Self {
        Self {
            a: self.a + o.0,
            b: self.b + o.0,
        }
    }
}
