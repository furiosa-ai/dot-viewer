use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Viewer, Mode, Focus };

impl App {    
    pub fn key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.char(c),
            KeyCode::Enter => self.enter(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Esc => self.esc(),
            KeyCode::Tab => self.tab(),
            KeyCode::BackTab => self.backtab(),
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            KeyCode::Right => self.right(),
            KeyCode::Left => self.left(),

            _ => {},
        }
    }

    fn char(&mut self, c: char) {
        match self.mode {
            Mode::Navigate => self.nav_char(c),
            Mode::Search => self.search_char(c),
        } 
    }

    fn nav_char(&mut self, c: char) {
        match c {
            'q' => self.quit = true,
            '/' => self.to_search_mode(),
            _ => {},
        }
    }

    fn search_char(&mut self, c: char) {
        self.input.push(c);

        self.viewer.update_search_fwd(self.input.clone());
    }

    fn enter(&mut self) {
        self.viewer.enter();

        match self.mode {
            Mode::Search => self.to_nav_mode(),
            _ => {},
        }
    }

    fn backspace(&mut self) {
        match self.mode {
            Mode::Search => {
                self.input.pop();

                self.viewer.update_search_bwd(self.input.clone());
            },
            _ => {},
        } 
    }

    fn esc(&mut self) {
        match self.mode {
            Mode::Search => {
                self.input = String::from("");
                self.to_nav_mode();
            },
            _ => {},
        } 
    }

    fn tab(&mut self) {
        match &self.mode {
            Mode::Search => {
                let keyword: String = self.input.clone();
                self.autocomplete(keyword);
            },
            _ => {},
        }
    }

    fn backtab(&mut self) {
        match &self.mode {
            _ => {},
        }
    }

    fn up(&mut self) {
        self.viewer.up()
    }

    fn down(&mut self) {
        self.viewer.down()
    } 

    fn right(&mut self) {
        self.viewer.right()
    }

    fn left(&mut self) {
        self.viewer.left()
    }
}

impl Viewer {
    pub fn enter(&mut self) {
        let id = match self.focus {
            Focus::Prevs => self.prevs.selected(),
            Focus::Nexts => self.nexts.selected(),
            Focus::Search => if let Some((id, _)) = self.search.selected() {
                Some(id)
            } else {
                None
            },
            _ => None,
        };

        if let Some(id) = id {
            self.goto(&id);
        }
    }

    pub fn up(&mut self) {
        match self.focus {
            Focus::Current => self.current.previous(),
            Focus::Prevs => self.prevs.previous(),
            Focus::Nexts => self.nexts.previous(),
            Focus::Search => self.search.previous(),
        }
    }

    pub fn down(&mut self) {
        match self.focus {
            Focus::Current => self.current.next(),
            Focus::Prevs => self.prevs.next(),
            Focus::Nexts => self.nexts.next(),
            Focus::Search => self.search.next(),
        }
    }

    pub fn right(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Prevs,
            Focus::Prevs => Focus::Nexts,
            Focus::Nexts => Focus::Current,
            Focus::Search => Focus::Search,
        }
    }

    pub fn left(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Nexts,
            Focus::Prevs => Focus::Current,
            Focus::Nexts => Focus::Prevs,
            Focus::Search => Focus::Search,
        }
    }
}
