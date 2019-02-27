use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

use std::fs::*;

#[test]
fn input_folder_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Path doesn't exist
    cmd.arg("tests/path/to/nowhere/42")
        .arg("tests/output/location/fails.json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn output_destination_is_a_directory() -> Result<(), Box<std::error::Error>> {

    create_dir("./input")?;
    create_dir("./dir")?;
    

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Path doesn't exist
    cmd.arg("./input")
        .arg("./dir");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Is a directory"));

    remove_dir("./input")?;
    remove_dir("./dir")?;

    Ok(())
}