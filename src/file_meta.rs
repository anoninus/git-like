use std::path::PathBuf;

pub struct FileMeta {
  pub path: PathBuf,
  pub size: u64,
  pub modified: u64, // use K at modified
}
