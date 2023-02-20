#![allow(dead_code)]

use crate::viewer::error::{DotViewerError, DotViewerResult as Result};

// https://github.com/fdehau/tui-rs/blob/master/examples/tabs.rs
pub(crate) struct Tabs<T> {
    pub state: usize,
    pub tabs: Vec<T>,
}

impl<T> std::iter::FromIterator<T> for Tabs<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let state = 0;
        let tabs = Vec::from_iter(iter);

        Tabs { state, tabs }
    }
}

impl<T> Tabs<T> {
    pub(crate) fn next(&mut self) {
        let state = self.state;
        let len = self.tabs.len();

        self.state = if state < len - 1 { state + 1 } else { 0 };
    }

    pub(crate) fn previous(&mut self) {
        let state = self.state;
        let len = self.tabs.len();

        self.state = if state == 0 { len - 1 } else { state - 1 };
    }

    pub(crate) fn open(&mut self, tab: T) {
        self.tabs.push(tab);
        self.state = self.tabs.len() - 1;
    }

    pub(crate) fn close(&mut self) -> Result<()> {
        if self.state == 0 {
            return Err(DotViewerError::TabError("cannot close the first tab".to_string()));
        }

        self.tabs.remove(self.state);
        if self.state == self.tabs.len() {
            self.state -= 1;
        }

        Ok(())
    }

    pub(crate) fn select(&mut self, state: usize) {
        if state < self.tabs.len() {
            self.state = state;
        }
    }

    pub(crate) fn selected(&mut self) -> &mut T {
        &mut self.tabs[self.state]
    }
}
