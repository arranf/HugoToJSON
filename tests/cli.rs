use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::*;

use std::process::Command;
use std::fs::*;
use std::path::Path;
use std::io::{Write, BufReader};
use std::io::prelude::*;

#[test]
fn input_folder_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Path doesn't exist
    cmd.arg("tests/path/to/nowhere/42")
        .arg("-o")
        .arg("tests/output/location/fails.json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn output_folder_is_created_if_it_doesnt_exist() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-output-folder-is-created").suffix(".md").tempfile()?;
    let contents: &str = 
r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;
    println!("{:?}", file.path());
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let output_location = "./output_folder_is_created_if_it_doesnt_exist.json";

    assert!(!Path::new(output_location).exists());

    cmd.arg(file.path())
        .arg("-o")
        .arg(Path::new(output_location));

    cmd.assert().success();

    assert!(Path::new(output_location).exists());
    remove_file(output_location)?;
    file.close()?;
    Ok(())
}

#[test]
fn output_destination_is_a_directory() -> Result<(), Box<std::error::Error>> {
    let input_dir = Builder::new().prefix("input-output_destination_is_a_directory").tempdir()?;
    let output_dir = Builder::new().prefix("output-output_destination_is_a_directory").tempdir()?;
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg(input_dir.path())
        .arg("-o")
        .arg(output_dir.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Is a directory"));
    
    input_dir.close()?;
    output_dir.close()?;

    Ok(())
}

#[test]
fn hidden_files_are_skipped() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix(".").suffix(".md").tempfile()?;
    let output_file_path = "./hidden_files_are_skipped.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Write to hidden file
    let contents = 
r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    assert_eq!(read_back, "[]");
    remove_file(output_file_path)?;
    Ok(())
}

#[test]
fn skips_everything_except_md_files() -> Result<(), Box<std::error::Error>> {
    let input_dir = tempdir()?;
    let mut file_a = Builder::new().prefix("not-a-dotfile-skip-other-extensins").suffix(".txt").tempfile_in(input_dir.path())?;
    let mut file_b = Builder::new().prefix("not-a-dotfile-skip-other-extensins").suffix(".webm").tempfile_in(input_dir.path())?;

    let output_file_path = "./skips_everything_except_md_files.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Write to hidden file
    let contents = 
r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file_a, "{}", contents)?;
    writeln!(file_b, "{}", contents)?;

    cmd.arg(input_dir.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    assert_eq!(read_back, "[]");

    remove_file(output_file_path)?;
    input_dir.close()?;
    Ok(())
}

#[test]
fn skips_drafts_by_default() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-skips-drafts-by-default").suffix(".md").tempfile()?;
    let output_file_path = "./skips_drafts_by_default.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = 
r#"+++
draft = true
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert().success().stderr(predicate::str::contains("Skipping ".to_owned() + &file.path().to_string_lossy().into_owned() + ". Is draft"));

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    assert_eq!(read_back, "[]");

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn malformed_toml_produces_warning_and_exit_error() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-malformed-toml").suffix(".md").tempfile()?;
    let output_file_path = "./malformed_toml_produces_warning_and_exit_error.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = 
r#"+++
draft = true
title = "Replacing Sed/Awk With Amber"
date / "2019-01-25T07:52:40Z"
slug: "replacing-awk-sed-with-amber"
m4lformed
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not parse TOML front matter"));

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    assert_eq!(read_back, "[]");

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}


#[test]
fn malformed_yaml_produces_warning_and_exit_error() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-malformed-yaml").suffix(".md").tempfile()?;
    let output_file_path = "./malformed_yaml_produces_warning_and_exit_error.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = 
r#"---
draft: false
title = What You Can Achieve In a Year
date: "2019-02-15T20:01:39Z"
slug: what-you-can-achieve-in-a-year
tags:
m4lf0rm3zf
  - 'Blog'
  - 'RSS'
  - 'Blogging'
---
Jon Edmiston
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not parse YAML front matter"));

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    println!("{}", read_back);
    buf_reader.read_to_string(&mut read_back)?;
    assert_eq!(read_back, "[]");

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_yaml() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-correct-yaml").suffix(".md").tempfile()?;
    let output_file_path = "./correctly_produces_json_for_yaml.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = 
r#"---
draft: false
title: What You Can Achieve In a Year
date: "2019-02-15T20:01:39Z"
slug: what-you-can-achieve-in-a-year
tags:
  - 'Blog'
  - 'RSS'
  - 'Blogging'
---
# Jon Edmiston
"#;

    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert()
        .success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = r#"[{"title":"What You Can Achieve In a Year","href":"//what-you-can-achieve-in-a-year","date":"2019-02-15T20:01:39Z","content":"Jon Edmiston\n","tags":["Blog","RSS","Blogging"]}]"#;
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_toml() -> Result<(), Box<std::error::Error>> {
    let mut file = Builder::new().prefix("not-a-dotfile-correct-toml").suffix(".md").tempfile()?;
    let output_file_path = "./correctly_produces_json_for_toml.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = 
r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
tags = ['Rust', 'Bash', 'Awk', 'Sed', 'Amber', 'Code Search', 'Replace', 'Unix']
banner = ""
aliases = []
+++
Contents here
"#;

    writeln!(file, "{}", contents)?;

    cmd.arg(file.path())
        .arg("-o")
        .arg(output_file_path);

    cmd.assert()
        .success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = r#"[{"title":"Replacing Sed/Awk With Amber","href":"//replacing-awk-sed-with-amber","date":"2019-01-25T07:52:40Z","content":"Contents here","tags":["Rust","Bash","Awk","Sed","Amber","Code Search","Replace","Unix"]}]"#;
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}