use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Mode };

impl App {    
    pub fn key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.char(c),
            KeyCode::Enter => self.enter(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Esc => self.esc(),
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            _ => {},
        }
    }

    pub fn char(&mut self, c: char) {
        match self.mode {
            Mode::Normal => self.normal_char(c),
            Mode::Command => self.command_char(c),
        } 
    }

    pub fn normal_char(&mut self, c: char) {
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

    pub fn command_char(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn enter(&mut self) {
        match self.mode {
            Mode::Command => {
                let command: String = self.command.drain(..).collect();
                self.history.push(command.clone());
                self.exec(command); 
                
                self.command = String::from("");
                self.mode = Mode::Normal;
            },
            _ => {},
        } 
    }

    pub fn backspace(&mut self) {
        match self.mode {
            Mode::Command => {
                self.command.pop();
            },
            _ => {},
        } 
    }

    pub fn esc(&mut self) {
        match self.mode {
            Mode::Command => {
                self.command = String::from("");
                self.mode = Mode::Normal;
            },
            _ => {},
        } 
    }

    pub fn up(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.previous();
            },
            _ => {},
        }
    }

    pub fn down(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.next();
            },
            _ => {},
        }
    } 
}
