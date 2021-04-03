#![warn(clippy::all, clippy::pedantic)]
#![macro_use]
extern crate env_logger;

use env_logger::Env;
use hugo_to_json::{
    convert_to_json_and_write, hugo_to_json_error::HugotoJsonError, settings::Settings,
};
use structopt::StructOpt;

fn main() -> Result<(), HugotoJsonError> {
    env_logger::Builder::from_env(Env::new().filter_or("HUGO_TO_JSON_LOG", "info")).init();
    let settings = Settings::from_args();
    convert_to_json_and_write(settings.scan_path, settings.output, settings.drafts)
}
