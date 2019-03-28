#![warn(clippy::all, clippy::pedantic)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate strip_markdown;
extern crate yaml_rust;

pub mod hugo_to_json_error;
pub mod settings;

mod constants;
mod file_location;
mod operation_result;
mod page_index;
mod traverse;

use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

use hugo_to_json_error::*;
use traverse::{TraverseResults, Traverser};

pub fn create_page_index(contents_directory: PathBuf) -> Result<TraverseResults, HugotoJsonError> {
    let traverser = Traverser::new(contents_directory);
    let index = traverser.traverse_files()?;

    let (oks, errors): (Vec<_>, Vec<_>) = index.into_iter().partition(Result::is_ok);
    let index: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    Ok(TraverseResults::new(index, errors))
}

fn write_page_index<W: Write>(
    mut writer: W,
    serialized_page_index: &str,
) -> Result<(), HugotoJsonError> {
    writer.write_all(serialized_page_index.as_bytes())?;
    Ok(())
}

/// Converts a [Hugo](https://gohugo.io/) contents directory to JSON and writes it to a given file location. The file will be created if it doesn't exist
pub fn convert_to_json_and_write(
    contents_directory: PathBuf,
    output_location: Option<PathBuf>,
) -> Result<(), HugotoJsonError> {
    info!("Scanning {:?}", contents_directory);
    let traverse_results = create_page_index(contents_directory)?;
    let index = serde_json::to_string(&traverse_results.page_index)?;

    // Logging
    let writing_to;
    match output_location {
        Some(ref path) => writing_to = path.to_string_lossy().into_owned(),
        None => writing_to = String::from("stdout"),
    }
    info!("Writing index to {}", writing_to);

    match output_location {
        Some(path) => {
            create_dir_all(&path.with_file_name(constants::EMPTY_STRING))?;
            write_page_index(File::create(&path)?, &index)?
        }
        None => write_page_index(io::stdout(), &index)?,
    }

    if traverse_results.error_count > 0 {
        Err(HugotoJsonError::Meta(Meta::new(
            traverse_results.error_count,
            "Failed to process all content files",
        )))
    } else {
        debug!("Succesfully wrote index to {0}", writing_to);
        Ok(())
    }
}
