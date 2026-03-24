use std::{error::Error, path::PathBuf};

use crate::par_indexer::FileMeta;
fn load_index_vec() -> Result<Vec<FileMeta>, Box<dyn Error>> {
    let content = std::fs::read_to_string("Index.dat")?;
    let mut vec = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() != 3 {
            continue;
        }

        vec.push(FileMeta {
            path: PathBuf::from(parts[0]),
            size: parts[1].parse::<u64>()?,
            modified: parts[2].parse::<u64>()?,
        });
    }

    vec.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(vec)
}
