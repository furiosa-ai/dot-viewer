use crate::viewer::{
    app::App,
    error::{DotViewerError, DotViewerResult},
    modes::{InputMode, Mode, PopupMode, SearchMode},
    success::SuccessState,
    view::{Focus, View},
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
            Mode::Normal => self.char_normal(c),
            Mode::Input(_) => self.char_input(c).map(|_| SuccessState::default()),
            Mode::Popup(_) => self.char_popup(c).map(|_| SuccessState::default()),
        }
    }

    fn char_normal(&mut self, c: char) -> DotViewerResult<SuccessState> {
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
            'n' => {
                let view = self.tabs.selected();
                view.matches.next();
                view.goto_match().map(|_| SuccessState::default())
            }
            'N' => {
                let view = self.tabs.selected();
                view.matches.previous();
                view.goto_match().map(|_| SuccessState::default())
            }
            'h' => self.left().map(|_| SuccessState::default()),
            'j' => self.down().map(|_| SuccessState::default()),
            'k' => self.up().map(|_| SuccessState::default()),
            'l' => self.right().map(|_| SuccessState::default()),
            d if d.is_ascii_digit() => self.neighbors(d.to_digit(10).unwrap() as usize),
            _ => Err(DotViewerError::KeyError(KeyCode::Char(c))),
        }
    }

    fn char_input(&mut self, c: char) -> DotViewerResult<()> {
        self.input.insert(c);

        match &self.mode {
            Mode::Input(imode) => match imode {
                InputMode::Search(smode) => {
                    let view = self.tabs.selected();
                    let key = &self.input.key;

                    match smode {
                        SearchMode::Fuzzy => view.update_fuzzy(key),
                        SearchMode::Regex => view.update_regex(key),
                    }

                    view.update_trie();

                    view.goto_match()
                }
                InputMode::Command => Ok(()),
            }
            _ => unreachable!(),
        }
    }

    fn char_popup(&mut self, c: char) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Popup(pmode) => match pmode {
                PopupMode::Tree => self.char_tree(c),
                PopupMode::Help => self.char_help(c),
            },
            _ => unreachable!()
        }
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
            Mode::Normal => {
                let view = self.tabs.selected();
                view.enter()
            }
            Mode::Input(imode) => match imode {
                InputMode::Search(_) => {
                    self.set_normal_mode();
                    Ok(())
                }
                InputMode::Command => self.exec(),
            }
            Mode::Popup(pmode) => match pmode {
                PopupMode::Tree => self.subgraph(), 
                _ => Ok(()),
            },
        }
    }

    fn backspace(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Input(imode) => {
                self.input.delete();

                let key = &self.input.key;
                match imode {
                    InputMode::Search(smode) => match smode {
                        SearchMode::Fuzzy => view.update_fuzzy(&key),
                        SearchMode::Regex => view.update_regex(&key),
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
            Mode::Normal => Err(DotViewerError::KeyError(KeyCode::Esc)),
            _ => {
                self.set_normal_mode();
                Ok(())
            }
        }
    }

    fn tab(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Normal => self.tabs.next(),
            Mode::Input(imode) => match imode {
                InputMode::Search(smode) => match smode {
                    SearchMode::Fuzzy => self.autocomplete_fuzzy(),
                    SearchMode::Regex => self.autocomplete_regex(),
                }
                InputMode::Command => {},
            }
            _ => {},
        }

        Ok(())
    }

    fn backtab(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Normal => {
                self.tabs.previous();
                Ok(())
            }
            _ => Err(DotViewerError::KeyError(KeyCode::BackTab)),
        }
    }

    fn up(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Normal => view.up()?,
            Mode::Input(_) => view.matches.previous(),
            Mode::Popup(PopupMode::Tree) => view.subtree.up(),
            _ => {},
        };

        Ok(())
    }

    fn down(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();

        match &self.mode {
            Mode::Normal => view.down()?,
            Mode::Input(_) => view.matches.next(),
            Mode::Popup(PopupMode::Tree) => view.subtree.down(),
            _ => {},
        };

        Ok(())
    }

    fn right(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Normal => {
                let view = self.tabs.selected();
                view.right();
            }
            Mode::Input(_) => self.input.front(),
            Mode::Popup(PopupMode::Tree) => {
                let view = self.tabs.selected();
                view.subtree.right();
            }
            _ => {},
        }

        Ok(())
    }

    fn left(&mut self) -> DotViewerResult<()> {
        match &self.mode {
            Mode::Normal => {
                let view = self.tabs.selected();
                view.left();
            }
            Mode::Input(_) => self.input.back(),
            Mode::Popup(PopupMode::Tree) => {
                let view = self.tabs.selected();
                view.subtree.left();
            }
            _ => {},
        }

        Ok(())
    }
}

impl View {
    pub(super) fn enter(&mut self) -> DotViewerResult<()> {
        match &self.focus {
            Focus::Prev | Focus::Next => self.goto_adjacent(),
            Focus::Current => Ok(())
        }
    }

    pub(super) fn up(&mut self) -> DotViewerResult<()> {
        match &self.focus {
            Focus::Current => {
                self.current.previous();
                self.update_adjacent()?
            }
            Focus::Prev => self.prevs.previous(),
            Focus::Next => self.nexts.previous(),
        }

        Ok(())
    }

    pub(super) fn down(&mut self) -> DotViewerResult<()> {
        match &self.focus {
            Focus::Current => {
                self.current.next();
                self.update_adjacent()?
            }
            Focus::Prev => self.prevs.next(),
            Focus::Next => self.nexts.next(),
        }

        Ok(())
    }

    pub(super) fn right(&mut self) {
        self.focus = match &self.focus {
            Focus::Current => Focus::Prev,
            Focus::Prev => Focus::Next,
            Focus::Next => Focus::Current,
        };
    }

    pub(super) fn left(&mut self) {
        self.focus = match &self.focus {
            Focus::Current => Focus::Next,
            Focus::Prev => Focus::Current,
            Focus::Next => Focus::Prev,
        };
    }
}
