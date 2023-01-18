use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Mode, Focus };

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
            _ => {},
        }
    }

    fn char(&mut self, c: char) {
        match self.mode {
            Mode::Traverse(_) => self.normal_char(c),
            Mode::Command => self.command_char(c),
        } 
    }

    fn normal_char(&mut self, c: char) {
        match c {
            'q' => {
                self.quit = true;
            },
            '!' => {
                self.mode = Mode::Command;
            },
            _ => {},
        }
    }

    fn command_char(&mut self, c: char) {
        self.input.push(c);
    }

    fn enter(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => match focus {
                Focus::Prevs => match self.prevs.selected() {
                    Some(id) => {
                        self.goto(&id);
                        ()
                    },
                    None => {},
                },
                Focus::Nexts => match self.nexts.selected() {
                    Some(id) => {
                        self.goto(&id);
                        ()
                    },
                    None => {},
                },
                _ => {},
            },
            Mode::Command => {
                let command: String = self.input.drain(..).collect();
                self.history.push(command.clone());
                self.exec(command); 
                
                self.input = String::from("");
                self.mode = Mode::Traverse(Focus::All);
            },
        } 
    }

    fn backspace(&mut self) {
        match self.mode {
            Mode::Command => {
                self.input.pop();
            },
            _ => {},
        } 
    }

    fn esc(&mut self) {
        match self.mode {
            Mode::Command => {
                self.input = String::from("");
                self.mode = Mode::Traverse(Focus::All);
            },
            _ => {},
        } 
    }

    fn tab(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => {
                // all -> prevs -> nexts
                self.mode = Mode::Traverse(match focus {
                    Focus::All => Focus::Prevs,
                    Focus::Prevs => Focus::Nexts,
                    Focus::Nexts => Focus::All,
                })
            },
            _ => {},
        }
    }

    fn backtab(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => {
                // all <- prevs <- nexts
                self.mode = Mode::Traverse(match focus {
                    Focus::All => Focus::Nexts,
                    Focus::Prevs => Focus::All,
                    Focus::Nexts => Focus::Prevs,
                })
            },
            _ => {},
        }
    }

    fn up(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => match focus {
                Focus::All => {
                    self.all.previous();
                    self.update_list();
                },
                Focus::Prevs => self.prevs.previous(),
                Focus::Nexts => self.nexts.previous(),
            },
            _ => {},
        }
    }

    fn down(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => match focus {
                Focus::All => {
                    self.all.next();
                    self.update_list();
                },
                Focus::Prevs => self.prevs.next(),
                Focus::Nexts => self.nexts.next(),
            },
            _ => {},
        }
    } 
}
