use crate::viewer::{
    error::{DotViewerError, DotViewerResult},
    modes::{InputMode, MainMode, Mode, NavMode, PopupMode, SearchMode},
    success::SuccessState,
    App,
};

use crossterm::event::{KeyCode, KeyEvent};
use log::{info, warn};

impl App {
    pub fn key(&mut self, key: KeyEvent) {
        info!("{:?}", key.code);

        self.result = match key.code {
            KeyCode::Char(c) => self.char(c),
            KeyCode::Enter => self.enter().map(|_| SuccessState::default()),
            KeyCode::Backspace => self.backspace().map(|_| SuccessState::default()),
            KeyCode::Esc => self.esc().map(|_| SuccessState::default()),
            KeyCode::Tab => self.tab().map(|_| SuccessState::default()),
            KeyCode::BackTab => self.backtab().map(|_| SuccessState::default()),
            _ => Ok(SuccessState::default()),
        };

        if let Err(err) = &self.result {
            warn!("{}", err);
        }
    }

    fn char(&mut self, c: char) -> DotViewerResult<SuccessState> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(_) => self.char_nav(c),
                MainMode::Input(imode) => {
                    self.char_input(c, &imode.clone()).map(|_| SuccessState::default())
                }
            },
            Mode::Popup(pmode) => match pmode {
                PopupMode::Tree => self.char_tree(c).map(|_| SuccessState::default()),
                PopupMode::Help => self.char_help(c).map(|_| SuccessState::default()),
            },
        }
    }

    fn char_nav(&mut self, c: char) -> DotViewerResult<SuccessState> {
        match c {
            'q' => {
                self.quit = true;
                Ok(SuccessState::default())
            }
            '/' => {
                self.set_input_mode(InputMode::Search(SearchMode::Fuzzy));
                Ok(SuccessState::default())
            }
            'r' => {
                self.set_input_mode(InputMode::Search(SearchMode::Regex));
                Ok(SuccessState::default())
            }
            ':' => {
                self.set_input_mode(InputMode::Command);
                Ok(SuccessState::default())
            }
            's' => {
                self.set_popup_mode(PopupMode::Tree);
                Ok(SuccessState::default())
            }
            '?' => {
                self.set_popup_mode(PopupMode::Help);
                Ok(SuccessState::default())
            } 
            'c' => self.tabs.close().map(|_| SuccessState::default()),
            'e' => self.export(),
            'x' => self.xdot(),
            'n' => self.goto_match(true).map(|_| SuccessState::default()), 
            'N' => self.goto_match(false).map(|_| SuccessState::default()),
            'h' => self.left().map(|_| SuccessState::default()),
            'j' => self.down().map(|_| SuccessState::default()),
            'k' => self.up().map(|_| SuccessState::default()),
            'l' => self.right().map(|_| SuccessState::default()),
            d if d.is_ascii_digit() => self.neighbors(d.to_digit(10).unwrap() as usize),
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn char_input(&mut self, c: char, imode: &InputMode) -> DotViewerResult<()> {
        self.input.insert(c);

        let view = self.tabs.selected();
        let key = &self.input.key;
        match imode {
            InputMode::Search(smode) => match smode {
                SearchMode::Fuzzy => view.update_fuzzy(key),
                SearchMode::Regex => view.update_regex(key),
            },
            InputMode::Command => {},
        };
        view.update_trie();

        Ok(())
    }

    fn char_tree(&mut self, c: char) -> DotViewerResult<()> {
        match c {
            'q' => {
                self.quit = true;
                Ok(())
            }
            'h' => self.left(),
            'j' => self.down(),
            'k' => self.up(),
            'l' => self.right(),
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn char_help(&mut self, c: char) -> DotViewerResult<()> {
        match c {
            'q' => {
                self.quit = true;
                Ok(())
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn enter(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nav) => match nav {
                    NavMode::Prevs | NavMode::Nexts => self.goto_adjacent(),
                    NavMode::Current => Ok(()),
                },
                MainMode::Input(imode) => {
                    self.set_nav_mode();
                    Ok(())
                }
            },
            Mode::Popup(pmode) => match pmode {
                PopupMode::Tree => self.subgraph(),
                _ => Ok(()),
            },
        }
    }

    fn backspace(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Main(MainMode::Input(imode)) => {
                self.input.delete();

                let key = &self.input.key;
                match imode {
                    InputMode::Search(smode) => match smode {
                        SearchMode::Fuzzy => view.update_fuzzy(key),
                        SearchMode::Regex => view.update_regex(key),
                    },
                    InputMode::Command => {},
                };
                view.update_trie();

                Ok(())
            }
            _ => Err(DotViewerError::KeyError(KeyCode::Backspace)),
        }
    }

    fn esc(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Input(_) => {
                    self.set_nav_mode();
                    Ok(())
                }
                _ => Err(DotViewerError::KeyError(KeyCode::Esc)),
            },
            _ => {
                self.set_nav_mode();
                Ok(())
            }
        }
    }

    fn tab(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(_) => {
                    self.tabs.next();
                    Ok(())
                }
                MainMode::Input(imode) => {
                    let view = self.tabs.selected();

                    if let Some(key) = view.autocomplete(&self.input.key) {
                        self.input.set(key);

                        let key = &self.input.key;
                        match imode {
                            InputMode::Search(smode) => match smode {
                                SearchMode::Fuzzy => view.update_fuzzy(key),
                                SearchMode::Regex => view.update_regex(key),
                            },
                            InputMode::Command => {},
                        };
                    }

                    Ok(())
                }
            },
            _ => Err(DotViewerError::KeyError(KeyCode::Tab)),
        }
    }

    fn backtab(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Main(MainMode::Navigate(_)) => {
                self.tabs.previous();
                Ok(())
            }
            _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
        }
    }

    fn up(&mut self) -> DotViewerResult<()> {
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
            Mode::Popup(PopupMode::Tree) => view.subtree.up(),
            _ => {}
        };

        Ok(())
    }

    fn down(&mut self) -> DotViewerResult<()> {
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
            Mode::Popup(PopupMode::Tree) => view.subtree.down(),
            _ => {}
        };

        Ok(())
    }

    fn right(&mut self) -> DotViewerResult<()> {
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
            Mode::Popup(PopupMode::Tree) => {
                let view = self.tabs.selected();
                view.subtree.right();
            }
            _ => {}
        }

        Ok(())
    }

    fn left(&mut self) -> DotViewerResult<()> {
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
            Mode::Popup(PopupMode::Tree) => {
                let view = self.tabs.selected();
                view.subtree.left();
            }
            _ => {}
        }

        Ok(())
    }
}
