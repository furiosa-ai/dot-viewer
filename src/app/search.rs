use crate::app::{
    app::{ App, Viewer },
    utils::list::StatefulList,
};

impl App {
    pub fn autocomplete(&mut self, keyword: String) {
        if let Some(id) = self.viewer.autocomplete(keyword) {
            self.input = id;
        }
    }
}

impl Viewer {
    pub fn autocomplete(&mut self, keyword: String) -> Option<String> {
        self.trie.autocomplete(&keyword)
    }

    pub fn goto(&mut self, id: &str) -> Option<String> {
        let idx = self.current.find(id.to_string());
        match idx {
            Some(idx) => {
                self.current.select(idx);
                self.update_adjacent();
                None
            },
            None => Some(format!("Err: no such node {:?}", id))
        }
    }

    pub fn update_adjacent(&mut self) {
        let id = self.current().unwrap();

        let prevs = self.graph.froms(&id).iter().map(|n| n.to_string()).collect();
        self.prevs = StatefulList::with_items(prevs);

        let nexts = self.graph.tos(&id).iter().map(|n| n.to_string()).collect();
        self.nexts = StatefulList::with_items(nexts);
    }

    pub fn update_search(&mut self, key: String) {
        let nodes = self.current.items.clone();
        let search: Vec<String> = nodes.iter().filter(|id| id.starts_with(&key)).cloned().collect();

        self.search = StatefulList::with_items(search);
    }
}
