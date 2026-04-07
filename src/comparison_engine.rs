use crate::file_meta::FileMeta;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn compare(old: Vec<FileMeta>, new: Vec<FileMeta>) {
    let old_map: HashMap<PathBuf, FileMeta> =
        old.iter().map(|f| (f.path.clone(), f.clone())).collect();

    let old_hash_map: HashMap<[u8; 32], PathBuf> =
        old.iter().map(|f| (f.hash, f.path.clone())).collect();

    let mut changed = Vec::new();
    let mut unchanged = Vec::new();
    let mut renamed = Vec::new();
    let mut new_files = Vec::new();
    let mut deleted = Vec::new();

    let mut seen_old_paths: HashSet<PathBuf> = HashSet::new();

    for new_file in &new {
        match old_map.get(&new_file.path) {
            Some(old_file) => {
                seen_old_paths.insert(old_file.path.clone());

                if old_file.hash == new_file.hash {
                    unchanged.push(new_file.path.clone());
                } else {
                    changed.push(new_file.path.clone());
                }
            }
            None => {
                if let Some(old_path) = old_hash_map.get(&new_file.hash) {
                    seen_old_paths.insert(old_path.clone());

                    renamed.push((old_path.clone(), new_file.path.clone()));
                } else {
                    new_files.push(new_file.path.clone());
                }
            }
        }
    }

    for old_file in &old {
        if !seen_old_paths.contains(&old_file.path) {
            deleted.push(old_file.path.clone());
        }
    }
}
