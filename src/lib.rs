//! A crate for parsing YAML or TOML files from a [Hugo](https://gohugo.io/) contents directory to produce a JSON representation of the key front matter and contents of Hugo documents. It's main intent is to produce JSON to be used by [Lunr](https://lunrjs.com/) (and [Lunr-like](http://elasticlunr.com/) packages) to support search on a static Hugo site.

#![warn(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

/// Contains possible errors.
pub mod hugo_to_json_error;
/// Represents the result of trying to parse a file.
pub mod operation_result;
/// Contains the `PageIndex` data structure.
pub mod page_index;
/// Contains configuration options.
pub mod settings;

mod constants;
mod file_location;
mod traverse;

use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;

use hugo_to_json_error::HugotoJsonError;
use traverse::{TraverseResults, Traverser};

/// Given a contents directory it traverses all matching `.md` files with TOML and YAML frontmatter.
///
///  # Examples
/// ```no_run
/// use hugo_to_json::{create_page_index, page_index::PageIndex, operation_result::OperationResult, hugo_to_json_error::HugotoJsonError};
/// use std::path::PathBuf;
///
/// let traverse_result = create_page_index(PathBuf::from("/home/example_user/documents/blog/contents/"), false)?;
/// // We can then see if there were any errors.
/// let indices: Vec<PageIndex> = traverse_result.page_index;
/// let errors: Vec<OperationResult> = traverse_result.errors;
///
/// if traverse_result.error_count > 0 {
///     panic!("Errors found"); // Don't do this for real!
/// }
/// # Ok::<(), HugotoJsonError>(())
/// ```
///
/// # Errors
/// A `HugoToJsonError` should only occur if an IO error occurs trying to access the contents directory.
/// All other errors are stored in the errors property of the `TraverseResults`.
pub fn create_page_index(
    contents_directory: PathBuf,
    drafts: bool,
) -> Result<TraverseResults, HugotoJsonError> {
    let traverser = Traverser::new(contents_directory, drafts);
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

/// Converts a [Hugo](https://gohugo.io/) contents directory to JSON and writes it to a given location.
/// If the output location is provided and it doesn't exist, it will be created. If no output location it will write to stdout.
///
/// # Examples
///
/// A basic example that writes to stdout.
/// ```no_run
/// use hugo_to_json::convert_to_json_and_write;
/// use std::path::PathBuf;
/// # use hugo_to_json::hugo_to_json_error::HugotoJsonError;
/// convert_to_json_and_write(PathBuf::from("/home/example_user/documents/blog/contents/"), None, false)?;
/// # Ok::<(), HugotoJsonError>(())
/// ```
///
/// An example that writes to a file.
/// ```no_run
/// use hugo_to_json::convert_to_json_and_write;
/// use std::path::PathBuf;
/// # use hugo_to_json::hugo_to_json_error::HugotoJsonError;
///
/// let result = convert_to_json_and_write(PathBuf::from("/home/example_user/documents/blog/contents/"), Some(PathBuf::from("/home/example_user/documents/blog/static/index.json")), false)?;
/// # Ok::<(), HugotoJsonError>(())
/// ```
///
/// # Errors
/// Errors can occur if there is an error accessing the contents directory, serializing the page index to JSON, or performing IO writing the result out to either stdout or a file.
pub fn convert_to_json_and_write(
    contents_directory: PathBuf,
    output_location: Option<PathBuf>,
    drafts: bool,
) -> Result<(), HugotoJsonError> {
    info!("Scanning {:?}", contents_directory);
    let traverse_results = create_page_index(contents_directory, drafts)?;
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
        Err(HugotoJsonError::Meta {
            total: traverse_results.error_count,
        })
    } else {
        debug!("Succesfully wrote index to {0}", writing_to);
        Ok(())
    }
}
