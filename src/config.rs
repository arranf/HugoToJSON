
pub struct Config {
    pub scan_path: String,
    pub index_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let scan_path = args.get(1).and_then(|v| Some(v.clone())).unwrap_or(String::from("./content/"));
        let index_path = args.get(2).and_then(|v| Some(v.clone())).unwrap_or(String::from("./static/index.json"));
        Config { scan_path, index_path }
    }    
}