use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn required_arg_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("md2html")?;

    cmd.arg("");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument \'\' found",
    ));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("md2html")?;

    cmd.arg("--path").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Application parsing error: No such file or directory (os error 2)",
    ));

    Ok(())
}
