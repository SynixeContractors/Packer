use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn build() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packer")?;

    std::env::set_current_dir("tests")?;
    cmd.arg("./source").arg("./sync");
    cmd.assert().success();

    Ok(())
}
