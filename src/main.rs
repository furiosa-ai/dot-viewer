extern crate dot_viewer;

use std::error::Error;
use dot_viewer::terminal::launch;

fn main() -> Result<(), Box<dyn Error>> {
    launch()?;
    Ok(())
}
