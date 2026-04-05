use std::{
    error::Error,
    path::{Path, PathBuf},
};

// Called the parser with intial data;
use clap::Parser;
#[derive(clap::Parser)]
#[command(name = "gl", about = "The git like program")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

// The subcommands for the gl program
#[derive(clap::Subcommand)]
enum Commands {
    Add(ArgsForAdd),
}

#[derive(clap::Args)]
pub struct ArgsForAdd {
    // purpose of this is to force the user
    // to type the path after gl add
    #[arg(required = true)]
    path: Vec<PathBuf>,
}

pub fn parser() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Add(args) => {
            add_operation(args)?;
        }
    }
    Ok(())
}

fn add_operation(args: ArgsForAdd) -> Result<(), Box<dyn Error>> {
    let refined_paths = crate::refiner::refiner(&args.path);

    for path in refined_paths {
        if path == Path::new(".") {
            let walker_output = crate::walker::repo_walker(path)?;
            crate::parallel::par_indexer(walker_output)?;
        } else if path.is_dir() {
            // walkdir logic
        } else if path.is_file() {
            // for loop to catch all files and produce the vector of paths
        } else {
            println!("Wrong path!");
            std::process::exit(1);
        }
    }
    Ok(())
}
