use crate::file_meta::FileMeta;
use rayon::prelude::*;
use std::{error::Error, path::PathBuf, time::UNIX_EPOCH};


pub fn par_indexer(walker_output: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<FileMeta> = walker_output
        .into_par_iter()
        .filter_map(|file| metadata_to_stuct(&file).ok())
        .collect();

    entries.sort_by(|a, b| natord::compare(&a.path.to_string_lossy(), &b.path.to_string_lossy()));

    crate::writer::write_better(entries)?;
    Ok(())
}

fn metadata_to_stuct(file: &PathBuf) -> Result<FileMeta, Box<dyn Error>> {
    let metadata = std::fs::metadata(file)?;
    let modified_info = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let size_info = metadata.len();
    Ok(FileMeta {
        modified: modified_info,
        size: size_info,
        path: file.to_path_buf(),
    })
}

