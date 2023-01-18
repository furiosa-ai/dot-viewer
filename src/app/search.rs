use crate::app::app::{ App, Lists };

impl App {
    pub fn autocomplete(&mut self, keyword: String) {
        if let Some(node) = self.lists.autocomplete(keyword) {
            self.input = node;
        }
    }
}

impl Lists {
    pub fn autocomplete(&mut self, keyword: String) -> Option<String> {
        self.trie.autocomplete(&keyword)
    }

    pub fn search(&mut self, keyword: String) {
        self.goto(&keyword);
    }

    pub fn goto(&mut self, id: &str) -> Option<String> {
        let idx = self.current.find(id.to_string());
        match idx {
            Some(idx) => {
                self.current.select(idx);
                self.update();
                None
            },
            None => Some(format!("Err: no such node {:?}", id))
        }
    }
}
