use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    //print_filtered_envs();
    //piped_cmd();
    save_to_file();
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
