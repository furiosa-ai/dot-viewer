use crate::{ 
    app::app::App, 
    ui::ui 
};
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use std::{
    error::Error,
    io,
};
use tui::{
    backend::{ Backend, CrosstermBackend },
    Terminal,
};

pub fn launch(path: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create and run app
    // TODO accept path as command line args
    let app = App::new(&path);
    let res = run(&mut terminal, app);

    // restore terminal 
    terminal.clear()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            app.key(key);
        }

        if app.quit {
            return Ok(());
        }
    }
}
