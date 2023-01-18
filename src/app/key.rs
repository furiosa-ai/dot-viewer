use crossterm::event::{ KeyCode, KeyEvent };
use crate::app::app::{ App, Ctxt, Mode, Focus };

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
                // TODO this is redundant
                self.mode = Mode::Search;
                let tab = &mut self.ctxts[self.tab];
                tab.focus = Focus::Search;
            },
            _ => {},
        }
    }

    fn search_char(&mut self, c: char) {
        self.input.push(c);

        let tab = &mut self.ctxts[self.tab];
        tab.update_search(self.input.clone());
    }

    fn enter(&mut self) {
        let tab = &mut self.ctxts[self.tab];

        let ctxt = match &self.mode {
            Mode::Traverse => {
                tab.enter();
                None
            },
            Mode::Search => {
                let keyword: String = self.input.drain(..).collect();
                self.history.push(keyword.clone());

                // TODO this is redundant
                self.mode = Mode::Traverse;
                tab.focus = Focus::Current;

                match tab.search(keyword) {
                    Ok(ctxt) => Some(ctxt), 
                    Err(msg) => {
                        self.errormsg = Some(msg);
                        None
                    }
                } 
            },
        };

        if let Some(ctxt) = ctxt {
            self.ctxts.push(ctxt);
            self.tab = self.ctxts.len() - 1;
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
        let tab = &mut self.ctxts[self.tab];
        match &self.mode {
            _ => tab.up(),
        }
    }

    fn down(&mut self) {
        let tab = &mut self.ctxts[self.tab];
        match &self.mode {
            _ => tab.down(),
        }
    } 

    fn right(&mut self) {
        let tab = &mut self.ctxts[self.tab];
        match &self.mode {
            Mode::Traverse => tab.right(),
            _ => {},
        }
    }

    fn left(&mut self) {
        let tab = &mut self.ctxts[self.tab];
        match &self.mode {
            Mode::Traverse => tab.left(),
            _ => {},
        }
    }
}

impl Ctxt {
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
