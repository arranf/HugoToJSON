use std::fs;
use std::path::{Path,Component};
use std::env;

use toml::Value;
use walkdir::{DirEntry, WalkDir};

#[macro_use] extern crate serde_derive;

const FORWARD_SLASH: &str = "/";
const EMPTY_STRING: &str = "";
const TOML_FENCE: &str = "+++";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Scanning ${0}", &config.scan_path);
    let index = index_pages(&Path::new(&config.scan_path));
    let index = serde_json::to_string(&index).expect("Unable to serialize page index");
    println!("Writing index to {0}", &config.index_path);
    fs::create_dir_all(Path::new(&config.index_path).with_file_name(EMPTY_STRING)).expect("Error creating directories to write index");
    fs::write(config.index_path, index).expect("Error writing index");
}


fn index_pages(content_dir_path: &Path) -> Vec<PageIndex> {
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

fn process_file(root_dir: &Path, file: walkdir::DirEntry) -> Option<PageIndex>{
    if !file.file_type().is_file() {
        return None;
    }

    let path = file.path();
    let extension: Option<_> = path.extension().and_then(|e| e.to_str());
    if extension.is_none() {
        return None;
    }
    match extension.unwrap() {
        "md" => process_md(root_dir, path),
        // TODO: HTML
        _ => None
    }
}

fn process_md(root_dir: &Path, abs_path: &Path) -> Option<PageIndex> {
    let contents = fs::read_to_string(abs_path)
        .expect("Something went wrong reading the file");

    // Get the subdirectory path. Given ./blog/content/sub/post/example.md and a root_dir of ./blog/content produce sub/post 
    let directory: String = abs_path.strip_prefix(root_dir).expect("Error fetching subdir")
        .components().take_while(|comp: &Component| comp.as_os_str() != abs_path.file_name().unwrap())
        .map(|comp: Component| comp.as_os_str().to_str().unwrap())
        .collect::<Vec<&str>>()
        .join(FORWARD_SLASH);
    
    // Split out TOML
    // TODO: Support JSON and YAML by looking at the first line, using a match, and creating separate front matter structs
    let split_content: Vec<&str> = contents.trim().split(TOML_FENCE).collect();

    let length = split_content.len();
    if  length <= 1 {
        println!("Error reading {0}. No front matter detected. Skipping.", abs_path.to_str().unwrap());
        return None;
    }

    let front_matter = split_content[length - 2].trim().parse::<Value>().expect("Unable to parse TOML");
    let is_draft =  front_matter.get("draft").and_then(|v| v.as_bool()).unwrap_or(false);

    if is_draft {
        return None;
    }
    
    let title = front_matter.get("title").and_then(|v| v.as_str()).expect("Unable to get title from front matter").trim().to_owned();
    let tags: Vec<String> = front_matter.get("tags").and_then(|v| v.as_array()).expect("Unable to get tags array").iter().map(|v| v.as_str().expect("A tag in front matter is not a string").trim().to_owned()).collect();
    let slug = front_matter.get("slug").and_then(|v| v.as_str()).expect("No slug found").trim();
    let href = [FORWARD_SLASH, &directory, FORWARD_SLASH, slug].join(EMPTY_STRING);
    let content = split_content[length - 1].trim().to_owned();

    Some(PageIndex{title, tags, href, content })
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

#[derive(Serialize)]
struct PageIndex {
    title: String,
    tags: Vec<String>,
    href: String,
    content: String
}

struct Config {
    scan_path: String,
    index_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let scan_path = args.get(1).and_then(|v| Some(v.clone())).unwrap_or(String::from("./content/"));
        let index_path = args.get(2).and_then(|v| Some(v.clone())).unwrap_or(String::from("./static/index.json"));
        Config { scan_path, index_path }
    }    
}