use crossterm::event::{ KeyCode, KeyEvent };
use crate::utils::list::StatefulList;
use dot_graph::{
    parser::parse,
    structs::Graph,
};

pub enum Mode {
    Normal,
    Command,
}

pub struct App {
    pub quit: bool,

    pub mode: Mode,
    pub command: String,
    pub history: Vec<String>,

    pub graph: Graph,
    pub nodes: StatefulList<String>,
}

impl App {
    pub fn new(path: &str) -> App{
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();

        App {
            quit: false,
            mode: Mode::Normal,
            command: String::from(""),
            history: Vec::new(),
            graph,
            nodes: StatefulList::with_items(nodes),
        }
    }

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

    pub fn on_tick(&mut self) {
    }
}
