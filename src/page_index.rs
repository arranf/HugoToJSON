use crate::constants::*;

#[derive(Serialize)]
pub struct PageIndex {
    title: String,
    tags: Vec<String>,
    href: String,
    content: String
}

impl PageIndex {
    pub fn new (title: Option<&str>, slug: Option<&str>, tags: Vec<String>, content: String, directory: String) -> Option<PageIndex> {
        if title.is_none() || slug.is_none() {
            println!("Error reading {0}. Could not read expected fields from front matter. Skipping.", directory);
            return None;
        }
        
        let title = title.unwrap().trim().to_owned();
        let href = [FORWARD_SLASH, &directory, FORWARD_SLASH, slug.unwrap().trim()].join(EMPTY_STRING);

        Some(PageIndex{title, tags, href, content })
    }
}
