use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use crate::file_meta::FileMeta;

pub fn load_index() -> Result<Vec<FileMeta>, Box<dyn std::error::Error>>{
    let file = File::open("./Index.dat")?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() != 3 {
            continue;
        }
        let path = PathBuf::from(parts[0]);
        let size = parts[1].parse::<u64>().unwrap_or(0);
        let modified = parts[2].parse::<u64>().unwrap_or(0);

        entries.push(FileMeta {
            path,
            size,
            modified,
        });
        
    }
    Ok(entries)
}
