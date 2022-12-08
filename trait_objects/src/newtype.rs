use std::fmt;

// wrap the external type
pub struct Envelope(pub Vec<String>);

// impl the external trait
impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<[{}]>", self.0.join(" => "))
    }
}
