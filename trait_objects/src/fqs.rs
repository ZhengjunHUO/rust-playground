pub trait Cat {
    fn talk(&self);
}

pub trait Tiger {
    fn talk(&self);
}


pub struct Feline;
impl Feline {
    pub fn talk(&self) {
        println!("[undefined]");
    }
}

impl Cat for Feline {
    fn talk(&self) {
        println!("Miao miao miao ?");
    }
}

impl Tiger for Feline {
    fn talk(&self) {
        println!("Aowu ~");
    }
}
