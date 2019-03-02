use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "Hugo to JSON", about = "A tool to turn Hugo sites into a JSON representation.")]
pub struct Settings {
    /// The path to [Hugo](https://gohugo.io/)'s [contents](https://gohugo.io/content-management/organization/) directory. Defaults to `./content/`
    pub scan_path: String,
    /// The path that index will be output to. If not provided, the library writes to stdout
    #[structopt(short = "o", parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl Settings {
    /// Creates a 
    pub fn new(contents_directory: Option<String>, output: Option<PathBuf>) -> Self {
        let scan_path = contents_directory.unwrap_or_else(|| String::from("./content/"));
        let output = output;
        Self { scan_path, output }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_produces_defaults() {
        let settings = Settings::new(None, None);
        assert_eq!(settings.scan_path, "./content/");
        assert_eq!(settings.output, None);
    }

    #[test]
    fn first_arg_sets_scan_path() {
        let settings = Settings::new(Some(String::from("hello")), None);
        assert_eq!(settings.scan_path, "hello");
    }

    #[test]
    fn second_arg_sets_index_path() {
         let settings = Settings::new(Some(String::from("hello")), Some(PathBuf::from("world")));
        assert_eq!(settings.scan_path, "world");
    }
}