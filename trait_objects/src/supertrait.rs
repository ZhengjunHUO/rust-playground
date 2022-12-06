use std::fmt;

// supertrait is fmt::Display
// the one who want to implement WrappedPrint trait should have already implemented fmt::Display
pub trait WrappedPrint: fmt::Display {
    fn wrapped_print(&self) {
        let text = self.to_string();
        let len = text.len();
        println!("{}", "#".repeat(len + 4));
        println!("#{}#", " ".repeat(len + 2));
        println!("# {} #", text);
        println!("#{}#", " ".repeat(len + 2));
        println!("{}", "#".repeat(len + 4));
    }
}

pub struct Couple {
    pub a: u32,
    pub b: u32,
}

impl fmt::Display for Couple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

impl WrappedPrint for Couple {}
