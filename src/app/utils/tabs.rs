use crate::app::{error::DotViewerError, error::Res};

pub struct StatefulTabs<T> {
    pub state: usize,
    pub tabs: Vec<T>,
}

impl<T> StatefulTabs<T> {
    pub fn with_tabs(tabs: Vec<T>) -> Result<StatefulTabs<T>, DotViewerError> {
        if tabs.is_empty() {
            return Err(DotViewerError::TabError(
                "no tab given to tabs constructor".to_string(),
            ));
        }

        Ok(StatefulTabs { state: 0, tabs })
    }

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

    pub fn close(&mut self) -> Res {
        if self.state == 0 {
            return Err(DotViewerError::TabError(
                "cannot close the first tab".to_string(),
            ));
        }

        self.tabs.remove(self.state);
        if self.state == self.tabs.len() {
            self.state -= 1;
        }

        Ok(None)
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
