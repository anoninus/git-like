pub struct FileMeta {
  pub path: std::path::PathBuf,
  pub size: u64,
  pub modified: std::time::SystemTime,
}
