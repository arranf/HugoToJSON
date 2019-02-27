
pub struct Settings {
    pub scan_path: String,
    pub index_path: String,
}

impl Settings {
    pub fn new(args: &[String]) -> Self {
        let scan_path = args.get(1).and_then(|v| Some(v.clone())).unwrap_or_else(|| String::from("./content/"));
        let index_path = args.get(2).and_then(|v| Some(v.clone())).unwrap_or_else(|| String::from("./static/index.json"));
        Self { scan_path, index_path }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_produces_defaults() {
        let args = [];
        let settings = Settings::new(&args);
        assert_eq!(settings.scan_path, "./content/");
        assert_eq!(settings.index_path, "./static/index.json");
    }

    #[test]
    fn first_arg_sets_scan_path() {
        let args = [String::from("This is the path of the executable"), String::from("hello")];
        let settings = Settings::new(&args);
        assert_eq!(settings.scan_path, "hello");
    }

    #[test]
    fn second_arg_sets_index_path() {
        let args = [String::from("This is the path of the executable"), String::from("hello"), String::from("world")];
        let settings = Settings::new(&args);
        assert_eq!(settings.index_path, "world");
    }
}