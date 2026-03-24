use std::{error::Error, path::PathBuf};

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
