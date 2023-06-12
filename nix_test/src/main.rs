use nix::net::if_;

fn main() {
    let ifs = if_::if_nameindex().unwrap();
    for f in &ifs {
        println!("Found interface [#{}] {}", f.index(), f.name().to_string_lossy());
    }
}
