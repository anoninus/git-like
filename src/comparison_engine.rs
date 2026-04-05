use crate::file_meta::FileMeta;
use std::{collections::HashMap, path::PathBuf};


pub fn compare(old: Vec<FileMeta>, new: Vec<FileMeta>) {
  let mut old_map: HashMap<PathBuf, FileMeta> =
    old.into_iter().map(|f| (f.path.clone(), f)).collect();

  let mut changed = Vec::new();
  let mut unchanged = Vec::new();
  let mut new_files = Vec::new();

  for new_file in new {
    match old_map.remove(&new_file.path) {
      Some(old_files) => {}

      None => {}
    }
    if let Some(old_file) = old_map.remove(&new_file.path) {
      if old_file.size == new_file.size && old_file.modified == new_file.modified {
        unchanged.push(old_file);
      } else {
        changed.push(old_file);
      }
    } else {
      new_files.push(new_file);
    }
  }

  // collect the unmatched leftover
  let deleted: Vec<_> = old_map.into_values().collect();
}
