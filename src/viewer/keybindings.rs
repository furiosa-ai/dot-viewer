use crate::viewer::{
    error::{DotViewerError, DotViewerResult as Result},
    modes::{InputMode, MainMode, Mode, NavMode, SearchMode},
    App,
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

    fn char(&mut self, c: char) -> Result<Option<String>> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(_) => self.char_nav(c),
                MainMode::Input(imode) => self.char_input(c, &imode.clone()),
            },
            Mode::Popup => self.char_popup(c),
        }
    }

    fn char_nav(&mut self, c: char) -> Result<Option<String>> {
        match c {
            'q' => {
                self.quit = true;
                Ok(None)
            }
            '/' => {
                self.set_input_mode(InputMode::Search(SearchMode::Fuzzy));
                Ok(None)
            }
            'r' => {
                self.set_input_mode(InputMode::Search(SearchMode::Regex));
                Ok(None)
            }
            'f' => {
                self.set_input_mode(InputMode::Filter);
                Ok(None)
            }
            's' => {
                self.set_popup_mode();
                Ok(None)
            }
            'c' => self.tabs.close(),
            'e' => self.export(),
            'x' => self.xdot(),
            'h' => self.left(),
            'j' => self.down(),
            'k' => self.up(),
            'l' => self.right(),
            d if d.is_ascii_digit() => self.neighbors(d.to_digit(10).unwrap() as usize),
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn char_input(&mut self, c: char, imode: &InputMode) -> Result<Option<String>> {
        self.input.insert(c);

        let view = self.tabs.selected();
        let key = self.input.key();
        match imode {
            InputMode::Search(smode) => match smode {
                SearchMode::Fuzzy => view.update_fuzzy(key),
                SearchMode::Regex => view.update_regex(key),
            },
            InputMode::Filter => view.update_filter(key),
        };
        view.update_trie();

        Ok(None)
    }

    fn char_popup(&mut self, c: char) -> Result<Option<String>> {
        match c {
            'q' => {
                self.quit = true;
                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn enter(&mut self) -> Result<Option<String>> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Prevs | NavMode::Nexts => self.goto(),
                    NavMode::Current => Ok(None),
                },
                MainMode::Input(imode) => {
                    let res = match imode {
                        InputMode::Search(_) => self.goto(),
                        InputMode::Filter => self.filter(),
                    };
                    self.set_nav_mode();

                    res
                }
            },
            Mode::Popup => {
                let res = self.subgraph();
                self.set_nav_mode();

                res
            }
        }
    }

    fn backspace(&mut self) -> Result<Option<String>> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Main(MainMode::Input(imode)) => {
                self.input.delete();

                let key = self.input.key();
                match imode {
                    InputMode::Search(smode) => match smode {
                        SearchMode::Fuzzy => view.update_fuzzy(key),
                        SearchMode::Regex => view.update_regex(key),
                    },
                    InputMode::Filter => view.update_filter(key),
                };
                view.update_trie();

                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Backspace)),
        }
    }

    fn esc(&mut self) -> Result<Option<String>> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Input(_) => {
                    self.set_nav_mode();
                    Ok(None)
                }
                _ => Err(DotViewerError::KeyError(KeyCode::Esc)),
            },
            _ => {
                self.set_nav_mode();
                Ok(None)
            }
        }
    }

    fn tab(&mut self) -> Result<Option<String>> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(_) => {
                    self.tabs.next();
                    Ok(None)
                }
                MainMode::Input(imode) => {
                    let view = self.tabs.selected();

                    if let Some(key) = view.autocomplete(&self.input.key()) {
                        self.input.set(key);

                        let key = self.input.key();
                        match imode {
                            InputMode::Search(smode) => match smode {
                                SearchMode::Fuzzy => view.update_fuzzy(key),
                                SearchMode::Regex => view.update_regex(key),
                            },
                            InputMode::Filter => view.update_filter(key),
                        };
                    }

                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }

    fn backtab(&mut self) -> Result<Option<String>> {
        match &self.mode {
            Mode::Main(MainMode::Navigate(_)) => {
                self.tabs.previous();
                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
        }
    }

    fn up(&mut self) -> Result<Option<String>> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Current => {
                        view.current.previous();
                        view.update_adjacent()?;
                    }
                    NavMode::Prevs => view.prevs.previous(),
                    NavMode::Nexts => view.nexts.previous(),
                },
                MainMode::Input(_) => view.matches.previous(),
            },
            Mode::Popup => view.subtree.up(),
        };

        Ok(None)
    }

    fn down(&mut self) -> Result<Option<String>> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Current => {
                        view.current.next();
                        view.update_adjacent()?;
                    }
                    NavMode::Prevs => view.prevs.next(),
                    NavMode::Nexts => view.nexts.next(),
                },
                MainMode::Input(_) => view.matches.next(),
            },
            Mode::Popup => view.subtree.down(),
        };

        Ok(None)
    }

    fn right(&mut self) -> Result<Option<String>> {
        let mode = self.mode.clone();

        match mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => {
                    self.mode = match nav {
                        NavMode::Current => Mode::Main(MainMode::Navigate(NavMode::Prevs)),
                        NavMode::Prevs => Mode::Main(MainMode::Navigate(NavMode::Nexts)),
                        NavMode::Nexts => Mode::Main(MainMode::Navigate(NavMode::Current)),
                    };
                }
                MainMode::Input(_) => self.input.front(),
            },
            Mode::Popup => {
                let view = self.tabs.selected();
                view.subtree.right();
            }
        }

        Ok(None)
    }

    fn left(&mut self) -> Result<Option<String>> {
        let mode = self.mode.clone();

        match mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => {
                    self.mode = match nav {
                        NavMode::Current => Mode::Main(MainMode::Navigate(NavMode::Nexts)),
                        NavMode::Prevs => Mode::Main(MainMode::Navigate(NavMode::Current)),
                        NavMode::Nexts => Mode::Main(MainMode::Navigate(NavMode::Prevs)),
                    };
                }
                MainMode::Input(_) => self.input.back(),
            },
            Mode::Popup => {
                let view = self.tabs.selected();
                view.subtree.left();
            }
        }

        Ok(None)
    }
}
