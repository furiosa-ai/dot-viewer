mod terminal;
mod ui;
mod viewer;

use clap::Parser;
use terminal::launch;
use std::error::Error;

#[derive(Parser, Default, Debug)]
struct Cli {
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    launch(args.path)?;

    Ok(())
}
