#![allow(dead_code)]

use crate::viewer::error::{DotViewerError, DotViewerResult};

// https://github.com/fdehau/tui-rs/blob/master/examples/tabs.rs
pub(crate) struct Tabs<T> {
    pub state: usize,
    pub tabs: Vec<T>,
}

impl<T> std::iter::FromIterator<T> for Tabs<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let state = 0;
        let tabs = Vec::from_iter(iter);

        Self { state, tabs }
    }
}

impl<T> Tabs<T> {
    pub fn next(&mut self) {
        let state = self.state;
        let len = self.tabs.len();

        self.state = if state < len - 1 { state + 1 } else { 0 };
    }

    pub fn previous(&mut self) {
        let state = self.state;
        let len = self.tabs.len();

        self.state = if state == 0 { len - 1 } else { state - 1 };
    }

    pub fn open(&mut self, tab: T) {
        self.tabs.push(tab);
        self.state = self.tabs.len() - 1;
    }

    pub fn close(&mut self) -> DotViewerResult<()> {
        if self.state == 0 {
            return Err(DotViewerError::ViewerError("cannot close the first tab".to_string()));
        }

        self.tabs.remove(self.state);
        if self.state == self.tabs.len() {
            self.state -= 1;
        }

        Ok(())
    }

    pub fn select(&mut self, state: usize) {
        if state < self.tabs.len() {
            self.state = state;
        }
    }

    pub fn selected(&mut self) -> &mut T {
        &mut self.tabs[self.state]
    }
}
