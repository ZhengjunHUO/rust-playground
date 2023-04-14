extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // indicate rustc to link the system bzip2 shared library
    println!("cargo:rustc-link-lib=bz2");

    // invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // bindgen's entry point, build up options for the resulting bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Error occurred while generating bindings");

    // write out the bindings to $OUT_DIR/bindings.rs
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Error writing bindings to file");
}
