#![warn(clippy::all, clippy::pedantic)]
extern crate hugo_to_json;

use std::env;
use hugo_to_json::{config::*, program_error::*};

fn main() -> Result<(), ProgramError> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    hugo_to_json::convert_to_json(config)
}