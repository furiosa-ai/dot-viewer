use crate::{viewer::App, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use std::io::Stdout;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn launch(path: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut terminal = setup()?;

    // create and run app
    let app = App::new(&path).expect("user should provide path to a valid dot file");
    let _ = run(&mut terminal, app);

    cleanup()?;

    Ok(())
}

fn setup() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    setup_panic_hook();

    Ok(terminal)
}

#[allow(unused_variables)]
fn setup_panic_hook() { 
    std::panic::set_hook(Box::new(|info| {
        let _ = cleanup();

        #[cfg(debug_assertions)]
        better_panic::Settings::auto().create_panic_handler()(info);
    }));
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw_app(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            app.key(key);
        }

        if app.quit {
            break;
        }
    } 

    Ok(())
}

fn cleanup() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;

    Ok(())
}
