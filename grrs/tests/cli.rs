use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("-t").arg("huo").arg("-p").arg("/foo/bar");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open file"));

    Ok(())
}

#[test]
fn file_exist_and_match() -> Result<(), Box<dyn std::error::Error>> {
    let f = assert_fs::NamedTempFile::new("temp.txt")?;
    f.write_str("A foo file\nSome bar content\nSome baz content\nSome foo content.")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("-t").arg("foo").arg("-p").arg(f.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A foo file\nSome foo content."));

    Ok(())
}
