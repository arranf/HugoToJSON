#![warn(clippy::all, clippy::pedantic)]
extern crate hugo_to_json;
extern crate env_logger;

use std::env;

use env_logger::{Builder, Env};
use hugo_to_json::{settings::*, hugo_to_json_error::*};


fn main() -> Result<(), HugotoJsonError> {
    Builder::from_env(Env::new().filter_or("HUGO_TO_JSON_LOG", "info")).init();

    let args: Vec<String> = env::args().collect();
    let settings = Settings::new(&args);
    hugo_to_json::convert_to_json_and_write(&settings)
}