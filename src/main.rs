extern crate yaml_rust;
#[macro_use] extern crate serde_derive;

mod page_index;
mod config;
mod constants;
mod operation_result;

use std::fs;
use std::env;
use std::path::{Path,Component};

use crate::operation_result::*;
use crate::page_index::*;
use crate::config::*;

use toml::Value;
use walkdir::{DirEntry, WalkDir};

use yaml_rust::{YamlLoader};




fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    println!("Scanning ${0}", &config.scan_path);
    let index = traverse_files(&Path::new(&config.scan_path));
    let index = serde_json::to_string(&index).expect("Unable to serialize page index");
    
    println!("Writing index to {0}", &config.index_path);
    fs::create_dir_all(Path::new(&config.index_path).with_file_name(constants::EMPTY_STRING)).expect("Error writing index");
    
    fs::write(config.index_path, index)
}


fn traverse_files(content_dir_path: &Path) -> Vec<PageIndex> {
    let mut index = Vec::new();
    for entry in WalkDir::new(content_dir_path)
                        .into_iter()
                        .filter_entry(|e| !is_hidden(e))
                        .filter(|e| e.is_ok()) {
        if let Ok(file) = entry {
            match process_file(content_dir_path, file) {
                Ok(page) => index.push(page),
                Err(OperationResult::Parse(ref err)) => println!("{}", err),
                Err(OperationResult::Io(ref err)) => println!("{}", err),
                _ => ()
            }
        } else if let Some(io_error) = entry.unwrap_err().into_io_error() {
            println!("Failed {}", io_error);
        } else {
            println!("Error reading unknown file");
        }
    }
    index
}

fn process_file(root_dir: &Path, file: walkdir::DirEntry) -> Result<page_index::PageIndex, OperationResult> {
    if file.file_type().is_dir() {
        return Err(OperationResult::Parse(ParseError::new(file.path().to_str().unwrap_or_default().to_owned(), "Not a file")));
    }

    let path = file.path();
    let extension = path.extension().and_then(|e| e.to_str());

    match extension {
        Some("md") => process_md_file(root_dir, path),
        // TODO: .html files
        _ => Err(OperationResult::Parse(ParseError::new(file.path().to_str().unwrap_or_default().to_owned(), "Not a compatible file extension.")))?,
        // TODO: Handle None
    }
}

fn process_md_file(root_dir: &Path, abs_path: &Path) -> Result<page_index::PageIndex, OperationResult> {
    let contents = fs::read_to_string(abs_path)?;
    let abs_path_file_name = abs_path.file_name().unwrap();
    
    // Get the subdirectory path. Given ./blog/content/sub/post/example.md and a root_dir of ./blog/content produce sub/post 
    let directory: String = abs_path.strip_prefix(root_dir).expect("Error fetching subdir")
        .components().take_while(|comp: &Component| comp.as_os_str() != abs_path_file_name)
        .map(|comp: Component| comp.as_os_str().to_str().expect("Error fetching subdir"))
        .collect::<Vec<&str>>()
        .join(constants::FORWARD_SLASH);

    // TODO: Refactor this 
    let first_line = contents.lines().next();
    if first_line.is_none() || first_line.unwrap().trim().is_empty() {
        return Err(OperationResult::Parse(ParseError::new(directory, "Could not read first line of file")));
    }
    match first_line.unwrap().chars().next() {
        Some('+') => process_toml_front_matter(contents, directory),
        Some('-') => process_yaml_front_matter(contents, directory),
        // TODO: JSON frontmatter '{' => process_json_frontmatter()
        _ => Err(OperationResult::Parse(ParseError::new(directory,"Could not determine file front matter type.")))
    }
}

fn process_toml_front_matter(contents: String, directory: String) -> Result<page_index::PageIndex, OperationResult> {
    let split_content: Vec<&str> = contents.trim().split(constants::TOML_FENCE).collect();

    let length = split_content.len();
    if  length <= 1 {
        return Err(OperationResult::Parse(ParseError::new(directory, "Could not split on TOML fence.")));
    }

    let front_matter = split_content[length - 2].trim().parse::<Value>().map_err(|_| ParseError::new(directory.to_string(), "Could parse TOML front matter."))?;   
    let is_draft =  front_matter.get(constants::DRAFT).and_then(|v| v.as_bool()).unwrap_or(false);

    // TODO: Add a flag to allow indexing drafts
    if is_draft {
        return Err(OperationResult::Skip(Skip::new(directory, "Is draft.")));
    }
    
    let title = front_matter.get(constants::TITLE).and_then(|v| v.as_str());
    let slug = front_matter.get(constants::SLUG).and_then(|v| v.as_str());
    let date = front_matter.get(constants::DATE).and_then(|v| v.as_str());
    let description = front_matter.get(constants::DESCRIPTION).and_then(|v| v.as_str());

    let categories: Vec<String> = front_matter.get(constants::CATEGORIES).and_then(|v| v.as_array())
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();

    let series: Vec<String> = front_matter.get(constants::SERIES).and_then(|v| v.as_array())
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();

    let tags: Vec<String> = front_matter.get(constants::TAGS).and_then(|v| v.as_array())
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();
    
    let keywords: Vec<String> = front_matter.get(constants::KEYWORDS).and_then(|v| v.as_array())
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();
    
    let content = split_content[length - 1].trim().to_owned();

    page_index::PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory)
}

fn process_yaml_front_matter(contents: String, directory: String) -> Result<page_index::PageIndex, OperationResult> {
    let split_content: Vec<&str> = contents.trim().split(constants::YAML_FENCE).collect();
    let length = split_content.len();
    if length <= 1 {
        return Err(OperationResult::Parse(ParseError::new(directory, "Could not split on YAML fence.")))
    }

    let front_matter = split_content[length - 2].trim();
    let front_matter = YamlLoader::load_from_str(front_matter).map_err(|_| ParseError::new(directory.to_string(), "Failed to get front matter."))?;
    let front_matter = front_matter.first().ok_or(ParseError::new(directory.to_string(), "Failed to get front matter."))?;

    let is_draft =  front_matter[constants::DRAFT].as_bool().unwrap_or(false);

    // TODO: Add a flag to allow indexing drafts
    if is_draft {
        return Err(OperationResult::Skip(Skip::new(directory, "Is draft.")));
    }
    
    let title = front_matter[constants::TITLE].as_str();
    let slug = front_matter[constants::SLUG].as_str();
    let description = front_matter[constants::DESCRIPTION].as_str();
    let date = front_matter[constants::DATE].as_str();

    let series: Vec<String> = front_matter[constants::SERIES].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();

    let categories: Vec<String> = front_matter[constants::CATEGORIES].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();

    let tags: Vec<String> = front_matter[constants::TAGS].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();

    let keywords: Vec<String> = front_matter[constants::KEYWORDS].as_vec().unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| v.as_str())
        .map(|s| s.trim().to_owned())
        .collect();
    
    let content = split_content[length - 1].trim().to_owned();

    PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}



mod test {
    use super::*;

    #[test]
    fn page_index_from_yaml() {
        let contents = String::from(r#"
---
draft: false
title: Responsive Blog Images
date: "2019-01-20T23:11:28Z"
slug: responsive-blog-images
tags:
  - Hugo
  - Images
  - Responsive
  - Blog
---
The state of images on the web is pretty rough. What should be an easy goal, showing a user a picture, is...
"#);
        let page_index = process_yaml_front_matter(contents, String::from("post"));
        assert!(page_index.is_ok());
        let page_index = page_index.unwrap();
        assert_eq!(page_index.title, "Responsive Blog Images");
        assert_eq!(page_index.content, "The state of images on the web is pretty rough. What should be an easy goal, showing a user a picture, is...");
        assert_eq!(page_index.date, "2019-01-20T23:11:28Z");

        // Should be empty as not provided
        assert!(page_index.series.is_empty());
        assert!(page_index.keywords.is_empty());
        assert!(page_index.description.is_empty());
        assert!(page_index.categories.is_empty());
    }

    #[test]
    fn page_index_from_yaml_returns_none_when_draft() {
        let contents = String::from(r#"
---
draft: true
title: Responsive Blog Images
date: "2019-01-20T23:11:28Z"
slug: responsive-blog-images
tags:
  - Hugo
  - Images
  - Responsive
  - Blog
---
The state of images on the web is pretty rough. What should be an easy goal, showing a user a picture, is...
"#);
        let page_index = process_yaml_front_matter(contents, String::from("post"));
        assert!(page_index.is_err());
        // Pattern match error
        match page_index.unwrap_err() {
            OperationResult::Skip(_) => assert!(true), // The case where the result is a Skip result
            _ => assert!(false) // All other cases
        }
    }

    #[test]
    fn page_index_from_yaml_returns_err_if_fence_not_closed() {
        let contents = String::from(r#"
---
draft: true
title: Responsive Blog Images
date: "2019-01-20T23:11:28Z"
slug: responsive-blog-images
tags:
  - Hugo
  - Images
"#);
        let page_index = process_yaml_front_matter(contents, String::from("post"));
        assert!(page_index.is_err());
    }

}