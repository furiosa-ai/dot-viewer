use crate::app::{
    app::{ App, Ctxt },
    utils::list::StatefulList,
};

impl App {
    pub fn autocomplete(&mut self, keyword: String) {
        if let Some(node) = self.ctxt.autocomplete(keyword) {
            self.input = node;
        }
    }
}

impl Ctxt {
    pub fn autocomplete(&mut self, keyword: String) -> Option<String> {
        self.trie.autocomplete(&keyword)
    }

    pub fn search(&mut self, keyword: String) -> Option<String> {
        if self.search.items.len() > 0 {
            self.current = StatefulList::with_items(self.search.items.clone());
            self.search = StatefulList::with_items(Vec::new());
            self.update_adjacent();

            None
        } else {
            Some(format!("Err: no match for keyword {:?}", keyword))
        }
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

    // TODO only show prev, next nodes contained in current list?
    pub fn update_adjacent(&mut self) {
        let id = self.current().unwrap();

        let prevs = self.graph.froms(&id).iter().map(|n| n.to_string()).collect();
        self.prevs = StatefulList::with_items(prevs);

        let nexts = self.graph.tos(&id).iter().map(|n| n.to_string()).collect();
        self.nexts = StatefulList::with_items(nexts);
    }

    // TODO only show prev, next nodes contained in current list?
    pub fn update_search(&mut self, key: String) {
        let nodes = self.current.items.clone();
        let search: Vec<String> = nodes.iter().filter(|id| id.starts_with(&key)).cloned().collect();

        self.search = StatefulList::with_items(search);
    }
}
