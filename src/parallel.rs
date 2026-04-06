use crate::file_meta::FileMeta;
use rayon::prelude::*;
use std::{error::Error, path::PathBuf};

pub fn par_indexer(walker_output: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
  let mut entries: Vec<FileMeta> = walker_output
    .into_par_iter()
    .filter_map(|file| crate::meta_to_struct::metadata_to_stuct(&file).ok())
    .collect();

  entries.sort_by(|a, b| natord::compare(&a.path.to_string_lossy(), &b.path.to_string_lossy()));

  crate::writer::write_better(entries)?;
  Ok(())
}
