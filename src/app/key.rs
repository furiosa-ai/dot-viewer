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
                Focus::Prevs => {
                    let id = self.prevs.selected().unwrap();
                    self.goto(&id); 
                },
                Focus::Nexts => {
                    let id = self.nexts.selected().unwrap();
                    self.goto(&id);    
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

    fn up(&mut self) {
        match &self.mode {
            Mode::Traverse(focus) => match focus {
                Focus::All => {
                    self.all.previous();
                    self.prevs();
                    self.nexts();
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
                    self.prevs();
                    self.nexts();
                },
                Focus::Prevs => self.prevs.next(),
                Focus::Nexts => self.nexts.next(),
            },
            _ => {},
        }
    } 
}
