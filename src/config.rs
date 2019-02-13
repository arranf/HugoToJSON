
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_produces_defaults() {
        let args = [];
        let config = Config::new(&args);
        assert_eq!(config.scan_path, "./content/");
        assert_eq!(config.index_path, "./static/index.json");
    }

    #[test]
    fn first_arg_sets_scan_path() {
        let args = [String::from("This is the path of the executable"), String::from("hello")];
        let config = Config::new(&args);
        assert_eq!(config.scan_path, "hello");
    }

    #[test]
    fn second_arg_sets_index_path() {
        let args = [String::from("This is the path of the executable"), String::from("hello"), String::from("world")];
        let config = Config::new(&args);
        assert_eq!(config.index_path, "world");
    }
}