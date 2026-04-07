mod clap;
mod indexer;
mod walker;
mod refiner;
mod comparison_engine;
mod writer;
mod file_meta;
mod meta_to_struct;
mod hasher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate::clap::parser()?;
    Ok(())
}
