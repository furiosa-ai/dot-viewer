mod terminal;
mod ui;
mod viewer;

use std::error::Error;
use std::fs;

use chrono::prelude::*;
use clap::Parser;
use simplelog::{Config, LevelFilter, WriteLogger};

use terminal::launch;

#[derive(Parser, Default, Debug)]
struct Cli {
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    fs::create_dir_all("./logs")?;
    let file = fs::File::create(format!("logs/log_{}.log", Local::now()))?;
    WriteLogger::init(LevelFilter::Info, Config::default(), file)?;

    launch(args.path)?;

    Ok(())
}
