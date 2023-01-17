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

    fn char(&mut self, c: char) {
        match self.mode {
            Mode::Normal => self.normal_char(c),
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
        match self.mode {
            Mode::Command => {
                let command: String = self.input.drain(..).collect();
                self.history.push(command.clone());
                self.exec(command); 
                
                self.input = String::from("");
                self.mode = Mode::Normal;
            },
            _ => {},
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
                self.mode = Mode::Normal;
            },
            _ => {},
        } 
    }

    fn up(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.previous();
            },
            _ => {},
        }
    }

    fn down(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.next();
            },
            _ => {},
        }
    } 
}
