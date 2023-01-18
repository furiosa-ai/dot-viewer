use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Lists, Mode, Focus };

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
            KeyCode::Right => self.right(),
            KeyCode::Left => self.left(),

            _ => {},
        }
    }

    fn char(&mut self, c: char) {
        match self.mode {
            Mode::Traverse => self.normal_char(c),
            Mode::Search => self.search_char(c),
        } 
    }

    fn normal_char(&mut self, c: char) {
        match c {
            'q' => {
                self.quit = true;
            },
            '/' => {
                self.mode = Mode::Search;
            },
            _ => {},
        }
    }

    fn search_char(&mut self, c: char) {
        self.input.push(c);
    }

    fn enter(&mut self) {
        match &self.mode {
            Mode::Traverse => self.lists.enter(),
            Mode::Search => {
                let keyword: String = self.input.drain(..).collect();
                self.history.push(keyword.clone());
                self.lists.search(keyword); 
                
                self.mode = Mode::Traverse;
            },
        } 
    }

    fn backspace(&mut self) {
        match self.mode {
            Mode::Search => {
                self.input.pop();
            },
            _ => {},
        } 
    }

    fn esc(&mut self) {
        match self.mode {
            Mode::Search => {
                self.input = String::from("");
                self.mode = Mode::Traverse;
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

    fn up(&mut self) {
        match &self.mode {
            Mode::Traverse => self.lists.up(),
            _ => {},
        }
    }

    fn down(&mut self) {
        match &self.mode {
            Mode::Traverse => self.lists.down(),
            _ => {},
        }
    } 

    fn right(&mut self) {
        match &self.mode {
            Mode::Traverse => self.lists.right(),
            _ => {},
        }
    }

    fn left(&mut self) {
        match &self.mode {
            Mode::Traverse => self.lists.left(),
            _ => {},
        }
    }
}

impl Lists {
    pub fn enter(&mut self) {
        let id = match self.focus {
            Focus::Prevs => self.prevs.selected(),
            Focus::Nexts => self.nexts.selected(),
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
        }
    }

    pub fn down(&mut self) {
        match self.focus {
            Focus::Current => self.current.next(),
            Focus::Prevs => self.prevs.next(),
            Focus::Nexts => self.nexts.next(),
        }
    }

    pub fn right(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Prevs,
            Focus::Prevs => Focus::Nexts,
            Focus::Nexts => Focus::Current,
        }
    }

    pub fn left(&mut self) {
        self.focus = match self.focus {
            Focus::Current => Focus::Nexts,
            Focus::Prevs => Focus::Current,
            Focus::Nexts => Focus::Prevs,
        }
    }
}
