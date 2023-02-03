use crate::app::{
    app::App,
    error::{DotViewerError, Res},
    modes::{InputMode, MainMode, Mode, NavMode, SearchMode},
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
            Mode::Main(main) => match main {
                MainMode::Navigate(_) => self.char_nav(c),
                MainMode::Input(input) => self.char_input(c, &input.clone()),
            }
            Mode::Popup => self.char_popup(c),
        }
    }

    fn char_nav(&mut self, c: char) -> Res {
        match c {
            'q' => {
                self.quit = true;
                Ok(None)
            }
            '/' => {
                self.to_input_mode(InputMode::Search(SearchMode::Fuzzy));
                Ok(None)
            }
            'r' => {
                self.to_input_mode(InputMode::Search(SearchMode::Regex));
                Ok(None)
            }
            'f' => {
                self.to_input_mode(InputMode::Filter);
                Ok(None)
            }
            's' => {
                self.to_popup_mode();
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

    fn char_input(&mut self, c: char, mode: &InputMode) -> Res {
        self.input.insert(c);

        let viewer = self.tabs.selected();
        let key = self.input.key();
        match mode {
            InputMode::Search(search) => match search {
                SearchMode::Fuzzy => viewer.update_fuzzy(key),
                SearchMode::Regex => viewer.update_regex(key),
            },
            InputMode::Filter => viewer.update_filter(key),
        };
        viewer.update_trie();

        Ok(None)
    }

    fn char_popup(&mut self, c: char) -> Res {
        match c {
            'q' => {
                self.quit = true;
                Ok(None)
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn enter(&mut self) -> Res {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Prevs | NavMode::Nexts => self.goto(),
                    NavMode::Current => Ok(None),
                },
                MainMode::Input(input) => {
                    let res = match input {
                        InputMode::Search(_) => self.goto(),
                        InputMode::Filter => self.filter(),
                    };
                    self.to_nav_mode();

                    res
                }
            }
            Mode::Popup => {
                let res = self.subgraph();
                self.to_nav_mode();

                res
            }
        }
    }

    fn backspace(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Input(input) => {
                    self.input.delete();

                    let key = self.input.key();
                    match input {
                        InputMode::Search(search) => match search {
                            SearchMode::Fuzzy => viewer.update_fuzzy(key),
                            SearchMode::Regex => viewer.update_regex(key),
                        },
                        InputMode::Filter => viewer.update_filter(key),
                    };
                    viewer.update_trie();

                    Ok(None)
                }
                _ => Err(DotViewerError::KeyError(KeyCode::Backspace)),
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Backspace)),
        }
    }

    fn esc(&mut self) -> Res {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Input(_) => {
                    self.to_nav_mode();
                    Ok(None)
                }
                _ => Err(DotViewerError::KeyError(KeyCode::Esc)),
            }
            _ => {
                self.to_nav_mode();
                Ok(None)
            },
        }
    }

    fn tab(&mut self) -> Res {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(_) => {
                    self.tabs.next();
                    Ok(None)
                }
                MainMode::Input(input) => {
                    let viewer = self.tabs.selected();

                    if let Some(key) = viewer.autocomplete(&self.input.key()) {
                        self.input.set(key);

                        let key = self.input.key();
                        match input {
                            InputMode::Search(search) => match search {
                                SearchMode::Fuzzy => viewer.update_fuzzy(key),
                                SearchMode::Regex => viewer.update_regex(key),
                            },
                            InputMode::Filter => viewer.update_filter(key),
                        };
                    }

                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn backtab(&mut self) -> Res {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(_) => {
                    self.tabs.previous();
                    Ok(None)
                }
                _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
            }
            _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
        }
    }

    fn up(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Current => {
                        viewer.current.previous();
                        viewer.update_adjacent();
                    }
                    NavMode::Prevs => viewer.prevs.previous(),
                    NavMode::Nexts => viewer.nexts.previous(),
                },
                MainMode::Input(_) => viewer.matches.previous(),
            }
            Mode::Popup => viewer.tree.up(),
        };

        Ok(None)
    }

    fn down(&mut self) -> Res {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Current => {
                        viewer.current.next();
                        viewer.update_adjacent();
                    }
                    NavMode::Prevs => viewer.prevs.next(),
                    NavMode::Nexts => viewer.nexts.next(),
                },
                MainMode::Input(_) => viewer.matches.next(),
            }
            Mode::Popup => viewer.tree.down(),
        };

        Ok(None)
    }

    fn right(&mut self) -> Res {
        let mode = self.mode.clone();

        match mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => {
                    self.mode = match nav {
                        NavMode::Current => Mode::Main(MainMode::Navigate(NavMode::Prevs)),
                        NavMode::Prevs => Mode::Main(MainMode::Navigate(NavMode::Nexts)),
                        NavMode::Nexts => Mode::Main(MainMode::Navigate(NavMode::Current)),
                    };
                }
                MainMode::Input(_) => self.input.front(),
            }
            Mode::Popup => {
                let viewer = self.tabs.selected();
                viewer.tree.right();
            }
        }

        Ok(None)
    }

    fn left(&mut self) -> Res {
        let mode = self.mode.clone();

        match mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => {
                    self.mode = match nav {
                        NavMode::Current => Mode::Main(MainMode::Navigate(NavMode::Nexts)),
                        NavMode::Prevs => Mode::Main(MainMode::Navigate(NavMode::Current)),
                        NavMode::Nexts => Mode::Main(MainMode::Navigate(NavMode::Prevs)),
                    };
                }
                MainMode::Input(_) => self.input.back(),
            }
            Mode::Popup => {
                let viewer = self.tabs.selected();
                viewer.tree.left();
            }
        }

        Ok(None)
    }
}
