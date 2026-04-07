use crate::file_meta::FileMeta;
use std::io::BufWriter;
use std::{
    error::Error,
    fs::{self, File},
    io::BufReader,
};

pub fn write_better(entries: Vec<FileMeta>) -> Result<(), Box<dyn Error>> {
    if fs::metadata("index.json").is_ok() {
        let old_data = load_index()?;
        let compared_output = crate::comparison_engine::compare(old_data, entries);
    } else {
        save_index(&entries)?;
    }
    Ok(())
}

pub fn save_index(entries: &Vec<FileMeta>) -> Result<(), Box<dyn Error>> {
    let file = File::create("index.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, entries)?;
    Ok(())
}

pub fn load_index() -> Result<Vec<FileMeta>, Box<dyn Error>> {
    let file = File::open("index.json")?;
    let reader = BufReader::new(file);
    let entries = serde_json::from_reader(reader)?;
    Ok(entries)
}
