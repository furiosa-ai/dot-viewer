pub struct StatefulTabs<T> {
    pub state: usize,
    pub tabs: Vec<T>,
}

impl<T> StatefulTabs<T> {
    pub fn with_tabs(tabs: Vec<T>) -> StatefulTabs<T> {
        // TODO there should be at least one tab in tabs now
        if tabs.is_empty() {
            panic!("Err: empty tabs");
        }

        StatefulTabs {
            state: 0,
            tabs,
        }
    }

    pub fn next(&mut self) {
        let state = self.state;
        let len = self.tabs.len();
        
        self.state = if state < len - 1 {
            state + 1
        } else {
            0
        };
    }

    pub fn previous(&mut self) {
        let state = self.state;
        let len = self.tabs.len();
        
        self.state = if state == 0 {
            len - 1
        } else {
            state - 1
        };
    }

    pub fn insert(&mut self, tab: T) {
        self.tabs.push(tab);
        self.state = self.tabs.len() - 1;
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
