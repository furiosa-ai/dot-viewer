use std::{
    io::Stdout,
    thread,
    sync::{ Arc, Mutex },
};
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

pub fn setup() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn cleanup<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
    terminal.clear()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    
    Ok(())
}

pub fn launch(path: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let terminal = setup()?;
    let terminal = Arc::new(Mutex::new(terminal));
    let recovery = terminal.clone();

    // create and run app in a child thread
    // https://stackoverflow.com/questions/43441047/whats-the-best-way-to-register-a-function-to-run-during-an-unexpected-exit-of-a
    let child = thread::spawn(move || {
        let mut terminal = terminal.lock().unwrap();
        let app = App::new(&path);
        let _ = run(&mut terminal, app);
    });

    match child.join() {
        Ok(_) => {},
        Err(_) => {
            println!("Err: dot-viewer paniced");

            let mut terminal = match recovery.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            cleanup(&mut terminal)?;
        }
    };

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
            break;
        }
    }

    let _ = cleanup(terminal);

    Ok(())
}
