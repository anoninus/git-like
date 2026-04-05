use crate::file_meta::FileMeta;
use std::{error::Error, fs, io::Write};
use std::io::BufWriter;
pub fn write_better(entries: Vec<FileMeta>) -> Result<(), Box<dyn Error>> {
    // fs::metadata(path) returns error if path do not exists.
    // .is_ok() transform's success into true and faliure into false
    if fs::metadata("Index.dat").is_ok() {
        let old = crate::index_loader::load_index()?;
        let output = crate::comparison_engine::comparison(&entries, &old);
    } else {
        let file = std::fs::File::create("Index.dat")?;
        let mut writer = BufWriter::new(file);
        for e in entries {
            writeln!(writer, "{}, {}, {}", e.path.display(), e.size, e.modified)?;
        }
    }
    Ok(())
}
