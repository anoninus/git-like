mod clap;
mod index_loader;
mod parallel;
mod walker;
mod refiner;
mod comparison_engine;
mod writer;
mod file_meta;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate::clap::parser()?;

    Ok(())
}
