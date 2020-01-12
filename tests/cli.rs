use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_version_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.arg("--version");

    cmd.assert().stdout(predicate::str::contains("git-mob 0.1"));

    Ok(())
}

#[test]
fn test_help_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("git-mob")?;
    cmd.arg("--help");

    cmd.assert()
        .stdout(predicate::str::contains("git-mob 0.1"))
        .stdout(predicate::str::contains(
            "James Thompson <james@thomps.onl>",
        ))
        .stdout(predicate::str::contains(
            "makes mob programming easier with Git",
        ));

    Ok(())
}
