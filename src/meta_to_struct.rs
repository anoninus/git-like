use crate::file_meta::FileMeta;
use std::{error::Error, path::PathBuf};
pub fn metadata_to_stuct(file: &PathBuf) -> Result<FileMeta, Box<dyn Error>> {
  let metadata = std::fs::metadata(file)?;
  let modified_info = metadata.modified()?;
  let size_info = metadata.len();
  Ok(FileMeta {
    modified: modified_info,
    size: size_info,
    path: file.to_path_buf(),
  })
}
