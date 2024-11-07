use anyhow::{bail, Result};
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    // should be a superuser and its password
    //dump_sql(
    //restore_sql(
    //    "postgresql://postgres:mysecretpassword@172.19.0.2:5432/rafaldb",
    //    "dump.sql",
    //)?;

    sql_client_version()?;
    //println!("Done");

    let rslt = list_dbs("postgresql://postgres:mysecretpassword@172.19.0.2:5432/rafaldb")?;
    println!("result: {:?}", rslt);
    Ok(())
}

fn list_dbs(db_name: &str) -> Result<Vec<String>> {
    let dbs = exec_cmd(
        "psql",
        [db_name, "-t", "-c", "SELECT datname FROM pg_database;"],
    )?;
    Ok(dbs
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>())
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

    let file = File::create(dump_name).unwrap();
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
        fs::remove_file(dump_name)?;
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
