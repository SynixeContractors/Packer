use assert_cmd::prelude::*; // Add methods on commands
// use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn build() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("packer")?;

    std::env::set_current_dir("tests")?;
    cmd.arg("./source").arg("./sync");
    cmd.assert().success();

    Ok(())
}
