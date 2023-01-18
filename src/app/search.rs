use crate::app::app::App;

impl App {
    pub fn autocomplete(&mut self, keyword: String) {
        if let Some(node) = self.trie.autocomplete(&keyword) {
            self.input = node;
        }
    }

    pub fn search(&mut self, keyword: String) {
        self.goto(&keyword);
    }

    pub fn goto(&mut self, id: &str) -> Option<String> {
        let idx = self.graph.lookup.get_by_left(id);
        match idx {
            Some(idx) => {
                self.all.select(*idx);
                self.update_list();
                None
            },
            None => Some(format!("Err: no such node {:?}", id))
        }
    }
}
