use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

use std::fs::*;

#[test]
fn input_folder_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Path doesn't exist
    cmd.arg("tests/path/to/nowhere/42")
        .arg("-o tests/output/location/fails.json");

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

    cmd.arg("./input")
        .arg("-o ./dir");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    remove_dir("./input")?;
    remove_dir("./dir")?;

    Ok(())
}

// It skips hidden files
// It correctly finds a deeply nested file
// It skips everything except .md files
// The output of a malformed TOML front matter is skipped and a warning occurs
// The output of a malformed YAML front matter is skipped and a warning occurs
// It correctly produces JSON for a YAML file
// It correctly produces JSON for a TOML file