#![warn(clippy::all, clippy::pedantic)]

#![macro_use]
extern crate structopt;
extern crate env_logger;
extern crate hugo_to_json;

mod settings;

use structopt::StructOpt;
use env_logger::Env;
use hugo_to_json::{settings::*, hugo_to_json_error::*};

fn main() -> Result<(), HugotoJsonError> {
    env_logger::Builder::from_env(Env::new().filter_or("HUGO_TO_JSON_LOG", "info")).init();
    let settings = Settings::from_args();
    hugo_to_json::convert_to_json_and_write(settings.scan_path, settings.output)
}