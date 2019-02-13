use crate::constants::*;
use crate::operation_result::*;

#[derive(Serialize, Debug)]
pub struct PageIndex {
    pub title: String,
    pub href: String,
    pub date: String,
    pub content: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}

impl PageIndex {
    pub fn new (title: Option<&str>, slug: Option<&str>, date: Option<&str>, description: Option<&str>, categories: Vec<String>, series: Vec<String>, tags: Vec<String>, keywords: Vec<String>, content: String, directory: String) -> Result<PageIndex, OperationResult> {
        if title.is_none() || slug.is_none() || date.is_none() {
            return Err(OperationResult::Parse(ParseError::new(directory, "Could not read expected fields from front matter")))
        }
        
        let title = title.unwrap().trim().to_owned();
        let date = date.unwrap().trim().to_owned();
        let description = description.unwrap_or("").to_owned();
        let href = [FORWARD_SLASH, &directory, FORWARD_SLASH, slug.unwrap().trim()].join(EMPTY_STRING);

        Ok(PageIndex{title, date, description, categories, tags, series, keywords, href, content})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_valid_href() {
        let title = Some("Title");
        let slug = Some("my-example-post");
        let date = Some("2018-01-01");
        let description = Some("An example description");
        let categories = Vec::new();
        let tags = Vec::new();
        let keywords = Vec::new();
        let series = Vec::new();
        let content = "A lot of content".to_owned();
        let directory = "post".to_owned();

        let page_index = PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory);
        assert!(page_index.is_ok());
        assert_eq!(page_index.unwrap().href, "/post/my-example-post")
    }

    #[test]
    fn no_title_returns_err() {
        let title = None;
        let slug = Some("my-example-post");
        let date = Some("2018-01-01");
        let description = Some("An example description");
        let categories = Vec::new();
        let tags = Vec::new();
        let keywords = Vec::new();
        let series = Vec::new();
        let content = "A lot of content".to_owned();
        let directory = "post".to_owned();

        assert!(PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory).is_err());
    }
    #[test]
    fn no_slug_returns_err() {
        let title = Some("Title");
        let slug = None;
        let date = Some("2018-01-01");
        let description = Some("An example description");
        let categories = Vec::new();
        let tags = Vec::new();
        let keywords = Vec::new();
        let series = Vec::new();
        let content = "A lot of content".to_owned();
        let directory = "post".to_owned();

        assert!(PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory).is_err());
    }

    #[test]
    fn no_date_returns_err() {
        let title = Some("Title");
        let slug = Some("my-example-post");
        let date = None;
        let description = Some("An example description");
        let categories = Vec::new();
        let tags = Vec::new();
        let keywords = Vec::new();
        let series = Vec::new();
        let content = "A lot of content".to_owned();
        let directory = "post".to_owned();

        assert!(PageIndex::new(title, slug, date, description, categories, series, tags, keywords, content, directory).is_err());
    }
}