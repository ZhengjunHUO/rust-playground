use anyhow::{bail, Result};
use std::env;
use std::fs::{self, File};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    dump_sql(
        "postgresql://user:pass@psql.psql-install.svc.cluster.local:5432/testdb",
        "test",
    )?;
    println!("Done");
    Ok(())
}

fn dump_sql(db_name: &str, dump_name: &str) -> Result<()> {
    let binary = "pg_dump";
    if !binary_exist_in_path(&binary) {
        bail!("Can't find binary {} in $PATH", binary);
    }

    let file_name = format!("{}.sql", dump_name);
    let file = File::create(&file_name).unwrap();
    let stdio = Stdio::from(file);

    let dump = Command::new(binary)
        .arg("--no-owner")
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
