use std::{collections::HashMap, path::PathBuf};

use crate::par_indexer::FileMeta;

pub fn compare(old: Vec<FileMeta>, new: Vec<FileMeta>) {
    let mut old_map: HashMap<PathBuf, FileMeta> =
        old.into_iter().map(|f| (f.path.clone(), f)).collect();

    let mut changed = Vec::new();
    let mut unchanged = Vec::new();
    let mut new_files = Vec::new();

    for new_file in new {
        match old_map.remove(&new_file.path) {
            Some(old_file) => {
                // File exists in old → compare metadata
                if old_file.size == new_file.size &&
                   old_file.modified == new_file.modified {
                    
                    unchanged.push(old_file);
                } else {
                    changed.push(old_file);
                }
            }

            None => {
                // File not found in old → new file
                new_files.push(new_file);
            }
        }
    }

    // Remaining files in old_map = deleted
    let deleted: Vec<_> = old_map.into_values().collect();

    // (optional) debug
    println!("changed: {}", changed.len());
    println!("unchanged: {}", unchanged.len());
    println!("new: {}", new_files.len());
    println!("deleted: {}", deleted.len());
}
