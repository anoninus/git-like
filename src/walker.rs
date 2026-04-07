use ignore::WalkBuilder;
use std::{error::Error, path::PathBuf};

pub fn repo_walker(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();

    for result in WalkBuilder::new(dir).build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if let Some(ft) = entry.file_type()
            && ft.is_file()
        {
            files.push(entry.into_path());
        }
    }

    Ok(files)
}
