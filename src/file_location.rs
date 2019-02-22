use crate::constants;
use crate::operation_result::*;

use std::path::{Path,Component};

use walkdir::{DirEntry};

#[derive(Debug)]
pub struct FileLocation {
    pub extension: String,
    pub absolute_path: String,
    pub file_name: String,
    pub relative_directory_to_content: String
}

impl FileLocation {
    pub fn new (file: DirEntry, content_dir: &Path) -> Result<Self, OperationResult> {
        if file.file_type().is_dir() {
            return Err(OperationResult::Skip(Skip::new(file.path().to_str().unwrap_or_default().to_owned(), "Not a file")));
        }

        let path = file.path();
        let absolute_path = path.to_string_lossy().into_owned();
        let extension = file.path().extension().ok_or_else(| | PathError::new(absolute_path.to_owned(), "Failed to determine extension."))?;
        let file_name = path.file_name().ok_or_else(| | PathError::new(absolute_path.to_owned(), "Failed to retrieve file name."))?;

        // Get the subdirectory path. Given ./blog/content/sub/post/example.md and a root_dir of ./blog/content produce sub/post 
        let relative_directory_to_content: String = path.strip_prefix(content_dir)
            .map_err(|_| PathError::new(absolute_path.to_owned(), "Failed to retrieve sub directory."))?
            .components().take_while(|comp: &Component| comp.as_os_str() != file_name)
            .filter_map(|comp: Component| comp.as_os_str().to_str())
            .collect::<Vec<&str>>()
            .join(constants::FORWARD_SLASH);
        
        let extension = extension.to_string_lossy().into_owned();
        let file_name = file_name.to_string_lossy().into_owned();
        
        Ok(Self { extension, absolute_path, file_name, relative_directory_to_content})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}