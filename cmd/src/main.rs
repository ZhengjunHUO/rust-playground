use anyhow::{bail, Result};
use regex::Regex;
use std::ffi::OsStr;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> Result<()> {
    let raw_client = exec_cmd("psql", ["--version"])?;
    println!("psql client verion: [{}]", grep_psql_version(&raw_client)?);

    let raw_server = exec_cmd(
        "psql",
        [
            "--dbname=postgresql://postgres:pwd@127.0.0.1:31000/dbname",
            "-t",
            "-c",
            "SELECT version();",
        ],
    )?;
    println!("psql server verion: [{}]", grep_psql_version(&raw_server)?);
    Ok(())
}

fn exec_cmd<T, S>(bin: &str, args: T) -> Result<String>
where
    T: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    if !binary_exist_in_path(bin) {
        bail!("Can't find binary {} in $PATH", bin);
    }

    let cmd = Command::new(bin)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Exec cmd failed.");

    let err_out = Command::new("cat")
        .arg("-")
        .stdin(cmd.stderr.unwrap())
        .output()
        .expect("Cat stderr failed.");
    let error = String::from_utf8_lossy(&err_out.stdout);
    if error.len() > 0 {
        bail!("Error occurred while executing {}: \n{}", bin, error);
    }

    let out = Command::new("cat")
        .arg("-")
        .stdin(cmd.stdout.unwrap())
        .output()
        .expect("Cat stdout failed.");

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Check if binary exists in environment variable $PATH
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

fn grep_psql_version(raw: &str) -> Result<String> {
    let rx = Regex::new(r" \d{2}\.\d{2} ").unwrap();
    match rx.find(raw) {
        Some(r) => Ok(r.as_str().trim_start().trim_end().to_owned()),
        None => bail!("Failed to grab psql client version from {} !", raw),
    }
}
