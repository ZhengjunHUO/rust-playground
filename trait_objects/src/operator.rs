use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Binome {
    pub a: u32,
    pub b: u32,
}

impl Mul for Binome {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Self {
            a: self.a * o.a,
            b: self.b * o.b,
        }
    }
}
