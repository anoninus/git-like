use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::{error::Error, io::BufWriter, path::PathBuf, time::UNIX_EPOCH};

pub struct FileMeta {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64, // use K at modified
}

pub fn par_indexer(walker_output: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut entries: Vec<FileMeta> = walker_output
        .into_par_iter()
        .filter_map(|file| metadata_to_stuct(&file).ok())
        .collect();

    entries.sort_by(|a, b| a.path.cmp(&b.path));

    write_better(entries)?;
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

fn write_better(entries: Vec<FileMeta>) -> Result<(), Box<dyn Error>> {
    // fs::metadata(path) returns error if path do not exists.
    // .is_ok() transform's success into true and faliure into false
    if fs::metadata("Index.dat").is_ok() {
        crate::index_loader::load_index()?;
    } else {
        let file = std::fs::File::create("Index.dat")?;
        let mut writer = BufWriter::new(file);
        for e in entries {
            writeln!(writer, "{}, {}, {}", e.path.display(), e.size, e.modified)?;
        }
    }
    Ok(())
}
