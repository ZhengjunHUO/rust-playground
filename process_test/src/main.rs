#![allow(dead_code)]

use glob::glob;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    //print_filtered_envs();
    //piped_cmd();
    //save_to_file();
    //append_path_env();

    // Attention: the pattern arg should be quoted: "/path/to/**/*.suffix"
    let arg = env::args().nth(1);
    match arg.as_deref() {
        //Some(p) => list_files_wildcard(p),
        Some(p) => {
            for entry in glob(p).expect("Failed to parse pattern") {
                match entry {
                    Ok(path) => println!("Found: {}", path.display()),
                    Err(e) => println!("Error occurred: {}", e),
                }
            }
        }

        _ => eprintln!("Wait for a path"),
    }
}

fn print_filtered_envs() {
    let filtered_env: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "USER" || k == "HOME" || k == "PATH")
        .collect();

    Command::new("printenv")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .env_clear()
        .envs(&filtered_env)
        .spawn()
        .expect("printenv failed to start");
}

fn append_path_env() {
    let env_dict: HashMap<String, String> = env::vars()
        .filter(|&(ref k, _)| k == "PATH")
        .collect();

    let path = env_dict.get("PATH").unwrap();

    Command::new("printenv")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .env_clear()
        .env("PATH", format!("{}:/tmp/bin", path))
        .spawn()
        .expect("printenv failed to start");
}

fn piped_cmd() {
    println!("{}", env::current_dir().unwrap().display());
    let find = Command::new("find")
        .arg("./src")
        .arg("-type")
        .arg("f")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec find");

    let find_out = find.stdout.expect("Failed to get find's stdout");

    let grep = Command::new("grep")
        .arg(".*rs")
        .stdin(Stdio::from(find_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec grep process");

    let out = grep
        .wait_with_output()
        .expect("Failed to wait grep's output");
    println!("Result: {}", String::from_utf8_lossy(&out.stdout));
}

fn save_to_file() {
    // $ echo -n hello > /tmp/hello.tmp
    let file = File::create("/tmp/hello.tmp").unwrap();
    let stdio = Stdio::from(file);

    Command::new("echo")
        .arg("-n")
        .arg("hello")
        .stdout(stdio)
        .spawn()
        .expect("echo failed to exec");
}

fn list_files_wildcard(wildcard: &str) {
    let ls_cmd = Command::new("ls")
        .arg(wildcard)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to exec ls");
    let out = ls_cmd.wait_with_output().expect("Failed to wait ls output");
    // print only the first match ??
    println!("Result: {}", String::from_utf8_lossy(&out.stdout));
}
