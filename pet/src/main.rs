use crate::feline::cat::Fufu;

pub mod feline;

fn main() {
    let pet = Fufu {};
    println!("I have a cat named {:?}.", pet);
}
