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
            Mode::Navigate => self.char_nav(c),
            Mode::Input => self.char_input(c),
        } 
    }

    fn char_nav(&mut self, c: char) {
        match c {
            'q' => self.quit = true,
            '/' => self.to_input_mode(Focus::Search),
            'f' => self.to_input_mode(Focus::Filter),
            'c' => self.tabs.close(),
            _ => {},
        }
    }

    fn char_input(&mut self, c: char) {
        self.input.push(c);

        let viewer = self.tabs.selected();
        viewer.char(self.input.clone());
    }

    fn enter(&mut self) {
        let viewer = self.tabs.selected();

        match viewer.focus {
            Focus::Filter => {
                let key: String = self.input.drain(..).collect();
                self.history.push(key.clone());

                match viewer.filter(key) {
                    Ok(viewer) => {
                        self.tabs.open(viewer);
                    },
                    Err(msg) => {
                        self.errormsg = Some(msg);
                    },
                }

                self.to_nav_mode();
            },
            Focus::Search => {
                viewer.enter();
                self.to_nav_mode();
            },
            _ => {
                viewer.enter();
            },
        }
    }

    fn backspace(&mut self) {
        match self.mode {
            Mode::Input => {
                self.input.pop();

                let viewer = self.tabs.selected();
                viewer.backspace(self.input.clone());
            },
            _ => {},
        } 
    }

    fn esc(&mut self) {
        match self.mode {
            Mode::Input => {
                self.input = String::from("");
                self.to_nav_mode();
            },
            _ => {},
        } 
    }

    fn tab(&mut self) {
        match &self.mode {
            Mode::Navigate => self.tabs.next(),
            _ => {},
        }
    }

    fn backtab(&mut self) {
        match &self.mode {
            Mode::Navigate => self.tabs.previous(),
            _ => {},
        }
    }

    fn up(&mut self) {
        let viewer = self.tabs.selected();
        viewer.up()
    }

    fn down(&mut self) {
        let viewer = self.tabs.selected();
        viewer.down()
    } 

    fn right(&mut self) {
        let viewer = self.tabs.selected();
        viewer.right()
    }

    fn left(&mut self) {
        let viewer = self.tabs.selected();
        viewer.left()
    }
}

impl Viewer {
    pub fn char(&mut self, input: String) {
        match self.focus {
            Focus::Search => self.update_search_fwd(input),
            Focus::Filter => self.update_filter(input),
            _ => {},
        }
    }

    pub fn backspace(&mut self, input: String) {
        match self.focus {
            Focus::Search => self.update_search_bwd(input),
            Focus::Filter => self.update_filter(input),
            _ => {},
        }
    }

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
            Focus::Current => {
                self.current.previous();
                self.update_adjacent();
            },
            Focus::Prevs => self.prevs.previous(),
            Focus::Nexts => self.nexts.previous(),
            Focus::Search => self.search.previous(),
            Focus::Filter => self.filter.previous(),
        }
    }

    pub fn down(&mut self) {
        match self.focus {
            Focus::Current => {
                self.current.next();
                self.update_adjacent();
            },
            Focus::Prevs => self.prevs.next(),
            Focus::Nexts => self.nexts.next(),
            Focus::Search => self.search.next(),
            Focus::Filter => self.filter.next(),
        }
    }

    pub fn right(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Prevs,
            Focus::Prevs => Focus::Nexts,
            Focus::Nexts => Focus::Current,
            Focus::Search => Focus::Search,
            Focus::Filter => Focus::Filter,
        }
    }

    pub fn left(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Nexts,
            Focus::Prevs => Focus::Current,
            Focus::Nexts => Focus::Prevs,
            Focus::Search => Focus::Search,
            Focus::Filter => Focus::Filter,
        }
    }
}
