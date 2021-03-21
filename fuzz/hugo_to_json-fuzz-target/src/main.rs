#[macro_use]
extern crate afl;

use hugo_to_json::*;
use std::io::prelude::*;
use tempfile::*;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut in_file = Builder::new()
            .prefix("a")
            .tempfile()
            .expect("Error in fuzzer creating tempfile");
        let out_file_path = Builder::new()
            .tempfile()
            .expect("Error creating out tempfile")
            .path()
            .to_path_buf();
        in_file
            .write_all(data)
            .expect("Error in fuzzer writing to file");
        convert_to_json_and_write(
            in_file.path().to_path_buf(),
            Some(out_file_path),
            false, // TODO: Fuzz this?
        )
        .unwrap();
    });
}
