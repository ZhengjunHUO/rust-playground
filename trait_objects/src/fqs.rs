// test #1
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

// test #2
pub trait Neko {
    fn nickname() -> String;
}

pub struct Mao;

impl Mao {
    pub fn nickname() -> String {
        String::from("FufuMao")
    }
}

impl Neko for Mao {
    fn nickname() -> String {
        String::from("FukuNeko")
    }
}
