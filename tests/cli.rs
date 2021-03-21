use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::*;

use serde_json::Value;
use std::fs::*;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::Command;

#[test]
fn input_folder_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
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
fn output_folder_is_created_if_it_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-output-folder-is-created")
        .suffix(".md")
        .tempfile()?;
    let contents: &str = r#"+++
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
fn output_destination_is_a_directory() -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = Builder::new()
        .prefix("input-output_destination_is_a_directory")
        .tempdir()?;
    let output_dir = Builder::new()
        .prefix("output-output_destination_is_a_directory")
        .tempdir()?;

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg(input_dir.path()).arg("-o").arg(output_dir.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Is a directory"));

    input_dir.close()?;
    output_dir.close()?;

    Ok(())
}

#[test]
fn hidden_files_are_skipped() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new().prefix(".").suffix(".md").tempfile()?;
    let output_file_path = "./hidden_files_are_skipped.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Write to hidden file
    let contents = r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

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
fn skips_everything_except_md_files() -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = tempdir()?;
    let mut file_a = Builder::new()
        .prefix("not-a-dotfile-skip-other-extensins")
        .suffix(".txt")
        .tempfile_in(input_dir.path())?;
    let mut file_b = Builder::new()
        .prefix("not-a-dotfile-skip-other-extensins")
        .suffix(".webm")
        .tempfile_in(input_dir.path())?;

    let output_file_path = "./skips_everything_except_md_files.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // Write to hidden file
    let contents = r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file_a, "{}", contents)?;
    writeln!(file_b, "{}", contents)?;

    cmd.arg(input_dir.path()).arg("-o").arg(output_file_path);

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
fn skips_drafts_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-skips-drafts-by-default")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./skips_drafts_by_default.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"+++
draft = true
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
slug = "replacing-awk-sed-with-amber"
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().success().stderr(predicate::str::contains(
        "Skipping ".to_owned() + &file.path().to_string_lossy().into_owned() + ". Is draft",
    ));

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
fn malformed_toml_produces_warning_and_exit_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-malformed-toml")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./malformed_toml_produces_warning_and_exit_error.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"+++
draft = true
title = "Replacing Sed/Awk With Amber"
date / "2019-01-25T07:52:40Z"
slug: "replacing-awk-sed-with-amber"
m4lformed
+++
Contents here
"#;
    writeln!(file, "{}", contents)?;

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Could not parse TOML front matter",
    ));

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
fn malformed_yaml_produces_warning_and_exit_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-malformed-yaml")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./malformed_yaml_produces_warning_and_exit_error.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"---
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

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Could not parse YAML front matter",
    ));

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
fn correctly_produces_json_for_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-correct-yaml")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./correctly_produces_json_for_yaml.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"---
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

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = r#"[{"title":"What You Can Achieve In a Year","href":"/what-you-can-achieve-in-a-year/","date":"2019-02-15T20:01:39Z","content":"Jon Edmiston\n","tags":["Blog","RSS","Blogging"],"draft":false}]"#;
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_toml() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("not-a-dotfile-correct-toml")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./correctly_produces_json_for_toml.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"+++
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

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = r#"[{"title":"Replacing Sed/Awk With Amber","href":"/replacing-awk-sed-with-amber/","date":"2019-01-25T07:52:40Z","content":"Contents here","tags":["Rust","Bash","Awk","Sed","Amber","Code Search","Replace","Unix"],"draft":false}]"#;
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_toml_with_url_property() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = Builder::new()
        .prefix("correctly_produces_json_for_toml_with_url_property")
        .suffix(".md")
        .tempfile()?;
    let output_file_path = "./correctly_produces_json_for_toml_with_url_property.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"+++
draft = false
title = "Replacing Sed/Awk With Amber"
date = "2019-01-25T07:52:40Z"
url = "/en/post/2019/replacing-awk-sed-with-amber"
tags = ['Rust', 'Bash', 'Awk', 'Sed', 'Amber', 'Code Search', 'Replace', 'Unix']
banner = ""
aliases = []
+++
Contents here
"#;

    writeln!(file, "{}", contents)?;

    cmd.arg(file.path()).arg("-o").arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = r#"[{"title":"Replacing Sed/Awk With Amber","href":"/en/post/2019/replacing-awk-sed-with-amber","date":"2019-01-25T07:52:40Z","content":"Contents here","tags":["Rust","Bash","Awk","Sed","Amber","Code Search","Replace","Unix"],"draft":false}]"#;
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_yaml_with_slug_and_relative_path(
) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = Builder::new().prefix("content").tempdir()?;
    let nested_inside_content_dir = Builder::new()
        .prefix("nested")
        .tempdir_in(input_dir.path())?;

    let mut file = Builder::new()
        .prefix("correctly_produces_json_for_yaml_with_relative_path-yaml")
        .suffix(".md")
        .tempfile_in(nested_inside_content_dir.path())?;
    let output_file_path = "./correctly_produces_json_for_yaml_with_relative_path.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"---
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

    cmd.arg(input_dir.path()).arg("-o").arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = format!(
        r#"[{{"title":"What You Can Achieve In a Year","href":"/{0}/what-you-can-achieve-in-a-year/","date":"2019-02-15T20:01:39Z","content":"Jon Edmiston\n","tags":["Blog","RSS","Blogging"],"draft":false}}]"#,
        nested_inside_content_dir
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    );
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_produces_json_for_yaml_without_slug_or_url() -> Result<(), Box<dyn std::error::Error>>
{
    let input_dir = Builder::new().prefix("content").tempdir()?;
    let nested_inside_content_dir = Builder::new()
        .prefix("nested")
        .tempdir_in(input_dir.path())?;

    let mut file = Builder::new()
        .prefix("correctly_produces_json_for_yaml_without_slug_or_url-yaml")
        .suffix(".md")
        .tempfile_in(nested_inside_content_dir.path())?;
    let output_file_path = "./correctly_produces_json_for_yaml_without_slug_or_url.json";
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let contents = r#"---
draft: false
title: What You Can Achieve In a Year
date: "2019-02-15T20:01:39Z"
tags:
  - 'Blog'
  - 'RSS'
  - 'Blogging'
---
# Jon Edmiston
"#;

    writeln!(file, "{}", contents)?;

    cmd.arg(input_dir.path()).arg("-o").arg(output_file_path);

    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let mut buf_reader = BufReader::new(output_file);
    let mut read_back = String::new();
    buf_reader.read_to_string(&mut read_back)?;
    let expected = format!(
        r#"[{{"title":"What You Can Achieve In a Year","href":"/{0}/{1}/","date":"2019-02-15T20:01:39Z","content":"Jon Edmiston\n","tags":["Blog","RSS","Blogging"],"draft":false}}]"#,
        nested_inside_content_dir
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
        file.path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
    );
    assert_eq!(read_back, expected);

    remove_file(output_file_path)?;
    file.close()?;
    Ok(())
}

#[test]
fn correctly_handles_large_numbers_of_files() -> Result<(), Box<dyn std::error::Error>> {
    // Create a large amount of files in a directory
    let input_dir = Builder::new()
        .prefix("correctly_handles_large_numbers_of_files-directory")
        .tempdir()?;

    // We keep a vec to prevent the files from being dropped & cleaned up
    let mut files: Vec<NamedTempFile> = Vec::new();
    let total_files = 500;
    for i in 0..total_files {
        let mut file = Builder::new()
            .prefix(&format!(
                "{}{}",
                "correctly_handles_large_numbers_of_files", i
            ))
            .suffix(".md")
            .tempfile_in(input_dir.path())?;
        let contents = r#"+++
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
        files.push(file);
    }

    let output_file_path = "./correctly_handles_large_numbers_of_files.json";

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg(input_dir.path()).arg("-o").arg(output_file_path);
    cmd.assert().success();

    let output_file = File::open(output_file_path)?;
    let buf_reader = BufReader::new(output_file);
    let v: Value = serde_json::from_reader(buf_reader)?;
    let length = v.as_array().unwrap().len();
    assert_eq!(length, total_files);

    remove_file(output_file_path)?;
    input_dir.close()?;
    Ok(())
}
