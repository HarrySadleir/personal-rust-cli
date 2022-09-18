use assert_cmd::prelude::*;
use predicates::prelude::*; 
use std::process::Command;

#[test]
fn minigrep_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cli")?;

    cmd.arg("minigrep").arg("to").arg("tests/resources/poem.txt");
    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("Are you nobody, too?\nHow dreary to be somebody!")
        );
    Ok(())
}