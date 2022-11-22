use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let pattern = &args[1];
    let path_to_file = &args[2];

    println!("grep {} from {}", pattern, path_to_file);

    let cont = fs::read_to_string(path_to_file).expect("Need to read from a exist file.");
    println!("The content of {}:\n{}", path_to_file, cont);
}
