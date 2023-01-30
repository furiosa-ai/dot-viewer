use crate::app::{
    app::App,
    error::{Res, DotViewerError},
    modes::{Input, Navigate, Mode, Search}, 
};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub fn key(&mut self, key: KeyEvent) {
        self.result = match key.code {
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
            _ => Ok(None),
        };
    }

    fn char(&mut self, c: char) -> Res {
        match &self.mode {
            Mode::Navigate(_) => self.char_nav(c),
            Mode::Input(input) => self.char_input(c, &input.clone()),
        }
    }

    fn char_nav(&mut self, c: char) -> Res {
        match c {
            'q' => {
                self.quit = true;
                Ok(None)
            }
            '/' => {
                self.to_input_mode(Input::Search(Search::Fuzzy));
                Ok(None)
            }
            'r' => {
                self.to_input_mode(Input::Search(Search::Regex));
                Ok(None)
            }
            'f' => {
                self.to_input_mode(Input::Filter);
                Ok(None)
            }
            'c' => self.tabs.close(),
            'e' => self.export(),
            'x' => self.xdot(),
            d if d.is_ascii_digit() => self.neighbors(d.to_digit(10).unwrap() as usize),
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn char_input(&mut self, c: char, input: &Input) -> Res {
        self.input.push(c);

        let viewer = self.tabs.selected();
        let key = self.input.clone();
        match input {
            Input::Search(search) => match search {
                Search::Fuzzy => viewer.update_prefix_fwd(key),
                Search::Regex => viewer.update_regex(key),
            }
            Input::Filter => viewer.update_filter(key),
        };

        Ok(None)
    }

    fn enter(&mut self) -> Res {
        match &self.mode {
            Mode::Navigate(nav) => match nav {
                Navigate::Prevs | Navigate::Nexts => self.goto(),
                Navigate::Current => Ok(None),
            },
            Mode::Input(input) => {
                let res = match input {
                    Input::Search(_) => self.goto(),
                    Input::Filter => self.filter(),
                };
                self.to_nav_mode();

                res
            }
        }
    }

    fn backspace(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Input(input) => {
                self.input.pop();
                let key = self.input.clone();

                match input {
                    Input::Search(search) => match search {
                        Search::Fuzzy => viewer.update_prefix_bwd(key),
                        Search::Regex => viewer.update_regex(key),
                    },
                    Input::Filter => viewer.update_filter(key),
                };

                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Backspace)),
        }
    }

    fn esc(&mut self) -> Res {
        match self.mode {
            Mode::Input(_) => {
                self.input = String::from("");
                self.to_nav_mode();

                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Esc)),
        }
    }

    fn tab(&mut self) -> Res {
        match &self.mode {
            Mode::Navigate(_) => {
                self.tabs.next();
                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Tab)),
        }
    }

    fn backtab(&mut self) -> Res {
        match &self.mode {
            Mode::Navigate(_) => {
                self.tabs.previous();
                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
        }
    }

    fn up(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Navigate(nav) => match nav {
                Navigate::Current => {
                    viewer.current.previous();
                    viewer.update_adjacent();
                }
                Navigate::Prevs => viewer.prevs.previous(),
                Navigate::Nexts => viewer.nexts.previous(),
            },
            Mode::Input(input) => match input {
                Input::Search(_) => viewer.search.previous(),
                Input::Filter => viewer.filter.previous(),
            },
        };

        Ok(None)
    }

    fn down(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Navigate(nav) => match nav {
                Navigate::Current => {
                    viewer.current.next();
                    viewer.update_adjacent();
                }
                Navigate::Prevs => viewer.prevs.next(),
                Navigate::Nexts => viewer.nexts.next(),
            },
            Mode::Input(input) => match input {
                Input::Search(_) => viewer.search.next(),
                Input::Filter => viewer.filter.next(),
            },
        };

        Ok(None)
    }

    fn right(&mut self) -> Res {
        self.mode = match &self.mode {
            Mode::Navigate(nav) => match nav {
                Navigate::Current => Mode::Navigate(Navigate::Prevs),
                Navigate::Prevs => Mode::Navigate(Navigate::Nexts),
                Navigate::Nexts => Mode::Navigate(Navigate::Current),
            },
            Mode::Input(input) => Mode::Input(input.clone()),
        };

        Ok(None)
    }

    fn left(&mut self) -> Res {
        self.mode = match &self.mode {
            Mode::Navigate(nav) => match nav {
                Navigate::Current => Mode::Navigate(Navigate::Nexts),
                Navigate::Prevs => Mode::Navigate(Navigate::Current),
                Navigate::Nexts => Mode::Navigate(Navigate::Prevs),
            },
            Mode::Input(input) => Mode::Input(input.clone()),
        };

        Ok(None)
    }
}
