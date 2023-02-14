mod terminal;
mod ui;
mod viewer;

use chrono::prelude::*;
use clap::Parser;
use simplelog::*;
use std::error::Error;
use terminal::launch;

#[derive(Parser, Default, Debug)]
struct Cli {
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    std::fs::create_dir_all("./logs")?;
    let file = std::fs::File::create(format!("logs/log_{}.log", Local::now()))?;
    WriteLogger::init(LevelFilter::Info, Config::default(), file).unwrap();

    launch(args.path)?;

    Ok(())
}
