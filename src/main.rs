use std::fs;
use std::path::{Path,Component};
use std::env;

use toml::Value;
use walkdir::{DirEntry, WalkDir};

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

#[macro_use] extern crate serde_derive;

mod page_index;
mod config;
mod constants;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args);
    
    println!("Scanning ${0}", &config.scan_path);
    let index = index_pages(&Path::new(&config.scan_path));
    let index = serde_json::to_string(&index).expect("Unable to serialize page index");
    
    println!("Writing index to {0}", &config.index_path);
    fs::create_dir_all(Path::new(&config.index_path).with_file_name(constants::EMPTY_STRING)).expect("Error creating directories to write index");
    
    fs::write(config.index_path, index)
}


fn index_pages(content_dir_path: &Path) -> Vec<page_index::PageIndex> {
    let mut index = Vec::new();
    for entry in WalkDir::new(content_dir_path)
                        .into_iter()
                        .filter_entry(|e| !is_hidden(e)) {
        let file = entry.expect("Error accessing file/directory during traversal.");
        let page_index = process_file(content_dir_path, file);
        if page_index.is_some() {
            index.push(page_index.unwrap());
        }
    }
    index
}

fn process_file(root_dir: &Path, file: walkdir::DirEntry) -> Option<page_index::PageIndex>{
    if !file.file_type().is_file() {
        return None;
    }

    let path = file.path();
    let extension: Option<_> = path.extension().and_then(|e| e.to_str());
    if extension.is_none() {
        return None;
    }
    match extension.unwrap() {
        "md" => process_md_file(root_dir, path),
        // TODO: HTML
        _ => None
    }
}

fn process_md_file(root_dir: &Path, abs_path: &Path) -> Option<page_index::PageIndex> {
    let contents = fs::read_to_string(abs_path)
        .expect("Something went wrong reading the file");

    // Get the subdirectory path. Given ./blog/content/sub/post/example.md and a root_dir of ./blog/content produce sub/post 
    
    let directory: String = abs_path.strip_prefix(root_dir).expect("Error fetching subdir")
        .components().take_while(|comp: &Component| comp.as_os_str() != abs_path.file_name().unwrap())
        .map(|comp: Component| comp.as_os_str().to_str().unwrap())
        .collect::<Vec<&str>>()
        .join(constants::FORWARD_SLASH);
    
    let first_line = contents.lines().next();

    if first_line.is_none() || first_line.unwrap().trim().is_empty() {
        return None;
    }

    match first_line.unwrap().chars().next().unwrap() {
        '+' => process_toml_front_matter(contents, directory),
        '-' => process_yaml_front_matter(contents, directory),
        // '{' => process_json_frontmatter()
        _ => None
    }
}

fn process_toml_front_matter(contents: String, directory: String) -> Option<page_index::PageIndex> {
    let split_content: Vec<&str> = contents.trim().split(constants::TOML_FENCE).collect();

    let length = split_content.len();
    if  length <= 1 {
        println!("Error reading {0}. Could not split on TOML fence. Skipping.", directory);
        return None;
    }

    let front_matter = split_content[length - 2].trim().parse::<Value>().expect("Unable to parse TOML");
    let is_draft =  front_matter.get("draft").and_then(|v| v.as_bool()).unwrap_or(false);

    if is_draft {
        return None;
    }
    
    let title = front_matter.get("title").and_then(|v| v.as_str());
    let slug = front_matter.get("slug").and_then(|v| v.as_str());
    let tags: Vec<String> = front_matter.get("tags").and_then(|v| v.as_array()).expect("Unable to get tags array")
        .iter()
        .map(|v| v.as_str().expect("A tag in front matter is not a string").trim().to_owned())
        .collect();
    
    let content = split_content[length - 1].trim().to_owned();

    page_index::PageIndex::new(title, slug, tags, content, directory)
}

fn process_yaml_front_matter(contents: String, directory: String) -> Option<page_index::PageIndex> {
    // TODO: DRY File Spliting
    let split_content: Vec<&str> = contents.trim().split(constants::YAML_FENCE).collect();
    let length = split_content.len();
    if length <= 1 {
        println!("Error reading {0}. Could not split on YAML fence. Skipping.", directory);
        return None;
    }

    let front_matter = split_content[length - 2].trim();
    let front_matter = &YamlLoader::load_from_str(front_matter).expect("Unable to parse YAML")[0];
    let is_draft =  front_matter["draft"].as_bool().unwrap_or(false);

    if is_draft {
        return None;
    }
    
    let title = front_matter["title"].as_str();
    let slug = front_matter["slug"].as_str();
    let tags: Vec<String> = front_matter["tags"].as_vec().expect("Unable to get tags array")
        .iter()
        .map(|v| v.as_str().expect("A tag in front matter is not a string").trim().to_owned())
        .collect();
    
    let content = split_content[length - 1].trim().to_owned();

    page_index::PageIndex::new(title, slug, tags, content, directory)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}