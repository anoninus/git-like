mod clap;
mod index_loader;
mod par_indexer;
mod walker;
mod refiner;
mod comparison_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    crate::clap::parser()?;

    Ok(())
}
