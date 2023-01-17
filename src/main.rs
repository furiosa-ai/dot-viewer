extern crate dot_viewer;

use std::error::Error;
use clap::{ arg, Parser };
use dot_viewer::terminal::launch;

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
