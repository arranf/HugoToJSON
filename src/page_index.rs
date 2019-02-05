use crate::constants::*;

#[derive(Serialize)]
pub struct PageIndex {
    title: String,
    href: String,
    date: String,
    content: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    series: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    keywords: Vec<String>,
}

impl PageIndex {
    pub fn new (title: Option<&str>, slug: Option<&str>, date: Option<&str>, description: Option<&str>, categories: Vec<String>, series: Vec<String>, tags: Vec<String>, keywords: Vec<String>, content: String, directory: String) -> Option<PageIndex> {
        if title.is_none() || slug.is_none() || date.is_none() {
            println!("Error reading {0}. Could not read expected fields from front matter. Skipping.", directory);
            return None;
        }
        
        let title = title.unwrap().trim().to_owned();
        let date = date.unwrap().trim().to_owned();
        let description = description.unwrap_or("").to_owned();
        let href = [FORWARD_SLASH, &directory, FORWARD_SLASH, slug.unwrap().trim()].join(EMPTY_STRING);

        Some(PageIndex{title, date, description, categories, tags, series, keywords, href, content})
    }
}
