use anyhow::{bail, Result};
use std::env;
use std::fs::{self, File};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    // should be a superuser and its password
    //dump_sql(
    //restore_sql(
    //    "postgresql://user:password@127.0.0.1:5432/dbname",
    //    "dump.sql",
    //)?;
    sql_client_version()?;
    println!("Done");
    Ok(())
}

fn sql_client_version() -> Result<()> {
    let binary = "psql";
    if !binary_exist_in_path(binary) {
        bail!("Can't find binary {} in $PATH", binary);
    }

    let psql = Command::new(binary)
        .arg("--version")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Get psql version failed.");

    let err = Command::new("cat")
        .arg("-")
        .stdin(psql.stderr.unwrap())
        .output()
        .expect("Cat stderr failed.");

    let error = String::from_utf8_lossy(&err.stdout);
    if error.len() > 0 {
        bail!("Error getting psql client's version: \n{}", error);
    }

    let rslt = Command::new("cat")
        .arg("-")
        .stdin(psql.stdout.unwrap())
        .output()
        .expect("Cat stdout failed.");

    println!(
        "psql client version: {}",
        String::from_utf8_lossy(&rslt.stdout)
            .strip_suffix('\n')
            .unwrap()
    );
    Ok(())
}

#[allow(dead_code)]
fn dump_sql(db_name: &str, dump_name: &str) -> Result<()> {
    let binary = "pg_dump";
    if !binary_exist_in_path(binary) {
        bail!("Can't find binary {} in $PATH", binary);
    }

    let file_name = format!("{}.sql", dump_name);
    let file = File::create(&file_name).unwrap();
    let stdio = Stdio::from(file);

    let dump = Command::new(binary)
        .arg(format!("--dbname={}", db_name))
        .stderr(Stdio::piped())
        .stdout(stdio)
        .spawn()
        .expect("Dump db failed.");

    let cat = Command::new("cat")
        .arg("-")
        .stdin(dump.stderr.unwrap())
        .output()
        .expect("Cat error failed.");

    let error = String::from_utf8_lossy(&cat.stdout);
    if error.len() > 0 {
        fs::remove_file(&file_name)?;
        bail!("while dumping the db: {}", error);
    }

    Ok(())
}

#[allow(dead_code)]
fn restore_sql(db_name: &str, dump_name: &str) -> Result<()> {
    let binary = "psql";
    if !binary_exist_in_path(binary) {
        bail!("Can't find binary {} in $PATH", binary);
    }

    let file = File::open(dump_name).unwrap();
    let stdio = Stdio::from(file);

    let psql = Command::new(binary)
        .arg(format!("--dbname={}", db_name))
        .stdin(stdio)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Restore db failed.");

    let err = Command::new("cat")
        .arg("-")
        .stdin(psql.stderr.unwrap())
        .output()
        .expect("Cat error failed.");

    let error: String = String::from_utf8_lossy(&err.stdout)
        .split_inclusive('\n')
        .filter(|s| !s.contains("already exists"))
        .collect();
    if !error.is_empty() {
        bail!("while restoring the db: \n{}", error);
    }

    Ok(())
}

fn binary_exist_in_path(bin: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for path in path.split(':') {
            let path_to_bin = format!("{}/{}", path, bin);
            if fs::metadata(path_to_bin).is_ok() {
                return true;
            }
        }
    }
    false
}
