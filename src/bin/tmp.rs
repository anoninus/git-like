use rayon::prelude::*;
use std::fs;
use std::io::Write;
use std::{error::Error, io::BufWriter, path::PathBuf, time::UNIX_EPOCH};

pub struct FileMeta {
  pub path: PathBuf,
  pub size: u64,
  pub modified: u64, // use K at modified
}

use ignore::WalkBuilder;

pub fn repo_walker(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
  // get the current working directory of the process
  // let current_dir = std::env::current_dir()?;

  // vector that will store the file paths
  let mut files = Vec::new();

  // recursive directory iterator
  for result in WalkBuilder::new(dir).build() {
    // result: Result<DirEntry, ignore::Error>

    // if the walk succeeded we get a DirEntry
    // if the walker encounters an error (permission denied etc.)
    // we skip that entry and continue scanning

    let entry = match result {
      Ok(e) => e,
      Err(_) => continue,
    };

    // the walker yields entries for:
    // files
    // directories
    // symlinks

    // we only want actual files

    // entry.file_type() -> Option<FileType>
    // Option is used because sometimes the OS cannot determine
    // the file type without extra filesystem calls

    // if the value is Some(ft) and ft.is_file() == true
    // then the block executes

    if let Some(ft) = entry.file_type()
      && ft.is_file()
    {
      // entry is a DirEntry struct representing a filesystem item

      // into_path() consumes the DirEntry and extracts its PathBuf
      // "into_" convention in Rust means:
      // convert self into another type while taking ownership

      // DirEntry -> PathBuf
      files.push(entry.into_path());
    }
  }

  // returns the Vec<PathBuf>
  Ok(files)
}

fn main() -> Result<(), Box<dyn Error>> {
  let dir = PathBuf::from(".");
  let walker_output = repo_walker(dir)?;
  par_indexer(walker_output);
  Ok(())
}
pub fn par_indexer(walker_output: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
  let mut entries: Vec<FileMeta> = walker_output
    .into_par_iter()
    .filter_map(|file| metadata_to_stuct(&file).ok())
    .collect();

  entries.sort_by(|a, b| natord::compare(&a.path.to_string_lossy(), &b.path.to_string_lossy()));

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
  if fs::metadata("./Index.dat").is_ok() {
  } else {
    let file = std::fs::File::create("./Index.dat")?;
    let mut writer = BufWriter::new(file);
    for e in entries {
      writeln!(writer, "{}, {}, {}", e.path.display(), e.size, e.modified)?;
    }
  }
  Ok(())
}
