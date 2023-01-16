extern crate dot_viewer;

use std::{ error::Error, time::Duration };
use dot_viewer::terminal::launch;

fn main() -> Result<(), Box<dyn Error>> {
    launch(Duration::from_millis(250))?;
    Ok(())
}
