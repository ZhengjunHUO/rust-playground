use anyhow::{bail, Result};
use glob::glob;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("coverage") => coverage()?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!("Available tasks:\n\ncoverage\t\t\t\tcalculate code coverage")
}

fn binary_exist_in_path(bin: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for path in path.split(":") {
            let path_to_bin = format!("{}/{}", path, bin);
            if fs::metadata(path_to_bin).is_ok() {
                return true;
            }
        }
    }
    false
}

fn remove_files(pattern: &str) -> Result<()> {
    for entry in glob(pattern).unwrap() {
        match entry {
            Ok(path) => {
                //println!("[DEBUG] Remove {:?}", path.display());
                fs::remove_file(path)?;
            }
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}

fn coverage() -> Result<()> {
    if !binary_exist_in_path("grcov") {
        bail!("Error: grcov not found in $PATH");
    }

    println!("[1] Preparing folder ...");
    let dest_dir = "coverage";
    if fs::metadata(&dest_dir).is_ok() {
        fs::remove_dir_all(&dest_dir)?;
    }
    fs::create_dir_all(&dest_dir)?;
    println!("[1] Done.");

    println!("[2] Generating test coverage profiles ...");
    let cargo_test = Command::new("cargo")
        .arg("test")
        .env("CARGO_INCREMENTAL", "0")
        .env("RUSTFLAGS", "-Cinstrument-coverage")
        .env("LLVM_PROFILE_FILE", "test-coverage-%p-%m.profraw")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run cargo test");

    let cargo_test_out = cargo_test.wait_with_output()?;
    println!("{}", String::from_utf8_lossy(&cargo_test_out.stdout));
    println!("[2] Done.");

    println!("[3] Generating report ...");
    let grcov_handle = Command::new("grcov")
        .arg(".")
        .arg("--binary-path")
        .arg("./target/debug/")
        .arg("-s")
        .arg(".")
        .arg("-t")
        .arg("html")
        .arg("--branch")
        .arg("--ignore-not-existing")
        //.arg("--ignore")
        //.arg("../*")
        //.arg("--ignore")
        //.arg("/*")
        .arg("--ignore")
        .arg("xtask/*")
        .arg("--ignore")
        .arg("*/src/tests/*")
        .arg("-o")
        .arg(format!("{}/html", dest_dir))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run grcov");
    let grcov_out = grcov_handle.wait_with_output()?;
    println!("Result: {}", String::from_utf8_lossy(&grcov_out.stdout));
    println!("[3] Done.");

    println!("[4] Cleaning up test coverage profiles ...");
    remove_files("**/*.profraw")?;
    println!("[4] Done.");
    Ok(())
}
