mod terminal;
mod ui;
mod viewer;

use clap::Parser;
use std::error::Error;
use terminal::launch;

#[derive(Parser, Default, Debug)]
struct Cli {
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    launch(args.path)?;

    Ok(())
}
