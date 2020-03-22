#![warn(clippy::all, clippy::pedantic)]
#![macro_use]
extern crate env_logger;

mod settings;

use env_logger::Env;
use hugo_to_json::{hugo_to_json_error::*, settings::*};
use structopt::StructOpt;

fn main() -> Result<(), HugotoJsonError> {
    env_logger::Builder::from_env(Env::new().filter_or("HUGO_TO_JSON_LOG", "info")).init();
    let settings = Settings::from_args();
    hugo_to_json::convert_to_json_and_write(settings.scan_path, settings.output)
}
