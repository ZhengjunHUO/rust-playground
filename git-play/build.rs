fn main() {
    let crate_home = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(r"cargo:rustc-link-search=native={}/libs", crate_home);
}
