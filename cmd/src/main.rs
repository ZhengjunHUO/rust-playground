use anyhow::{bail, Result};
use regex::Regex;
use std::cmp::{Ordering, PartialOrd};
use std::ffi::OsStr;
use std::fmt;
use std::process::{Command, Stdio};
use std::str::FromStr;
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

fn grep_psql_version(raw: &str) -> Result<BinVersion> {
    let rx = Regex::new(r" \d?\d\.\d\d? ").unwrap();
    match rx.find(raw) {
        Some(r) => {
            let mut it = r.as_str().trim_start().trim_end().split('.');
            let maj = u8::from_str(it.next().unwrap()).unwrap();
            let min = u8::from_str(it.next().unwrap()).unwrap();
            Ok(BinVersion::new(maj, min))
        }
        None => bail!("Failed to grab psql client version from {} !", raw),
    }
}

#[derive(PartialEq)]
pub(crate) struct BinVersion {
    major: u8,
    minor: u8,
}

impl BinVersion {
    fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for BinVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl PartialOrd<Self> for BinVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major < other.major {
            return Some(Ordering::Less);
        }

        if self.major > other.major {
            return Some(Ordering::Greater);
        }

        if self.minor < other.minor {
            return Some(Ordering::Less);
        }

        if self.minor > other.minor {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        let v1 = BinVersion::new(12, 9);
        let v2 = BinVersion::new(12, 15);
        let v3 = BinVersion::new(9, 18);
        let v4 = BinVersion::new(12, 9);

        assert!(v1 < v2);
        assert!(v1 > v3);
        assert!(v2 > v3);
        assert!(v1 == v4);
        assert!(v4 >= v3);
    }
}
