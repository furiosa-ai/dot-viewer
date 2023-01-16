use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Mode };

impl App {    
    pub fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.on_char(c),
            KeyCode::Enter => self.on_enter(),
            KeyCode::Backspace => self.on_backspace(),
            KeyCode::Esc => self.on_esc(),
            KeyCode::Up => self.on_up(),
            KeyCode::Down => self.on_down(),
            _ => {},
        }
    }

    pub fn on_char(&mut self, c: char) {
        match self.mode {
            Mode::Normal => self.on_normal_char(c),
            Mode::Command => self.on_command_char(c),
        } 
    }

    pub fn on_normal_char(&mut self, c: char) {
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

    pub fn on_command_char(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn on_enter(&mut self) {
        match self.mode {
            Mode::Command => {
                let command = self.command.drain(..).collect();
                self.history.push(command);
                self.command = String::from("");
                self.mode = Mode::Normal;
            },
            _ => {},
        } 
    }

    pub fn on_backspace(&mut self) {
        match self.mode {
            Mode::Command => {
                self.command.pop();
            },
            _ => {},
        } 
    }

    pub fn on_esc(&mut self) {
        match self.mode {
            Mode::Command => {
                self.command = String::from("");
                self.mode = Mode::Normal;
            },
            _ => {},
        } 
    }

    pub fn on_up(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.previous();
            },
            _ => {},
        }
    }

    pub fn on_down(&mut self) {
        match self.mode {
            Mode::Normal => {
                self.nodes.next();
            },
            _ => {},
        }
    } 
}
