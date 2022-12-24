use assert_cmd::prelude::*;
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
