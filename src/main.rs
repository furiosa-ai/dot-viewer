extern crate dot_viewer;

use clap::{arg, Parser};
use dot_viewer::terminal::launch;
use std::error::Error;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    launch(args.path)?;
    Ok(())
}
