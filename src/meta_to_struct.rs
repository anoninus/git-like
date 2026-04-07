use crate::file_meta::FileMeta;
use std::{error::Error, fs::File, path::PathBuf, time::UNIX_EPOCH};
pub fn metadata_to_stuct(path: &PathBuf) -> Result<FileMeta, Box<dyn Error>> {
    let metadata = std::fs::metadata(path)?;
    let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();
    let size = metadata.len();
    let file = File::open(path)?;
    let hash = crate::hasher::hasher(file, size)?;
    Ok(FileMeta {
        modified: modified_time,
        size,
        path: path.to_path_buf(),
        hash,
    })
}
