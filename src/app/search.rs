use crate::app::{
    app::{ App, Viewer },
    utils::list::StatefulList,
};

impl App {
    pub fn autocomplete(&mut self, keyword: String) {
        let viewer = &mut self.tabs.selected();
        if let Some(id) = viewer.autocomplete(keyword) {
            self.input = id;
        }
    }
}

impl Viewer {
    pub fn autocomplete(&mut self, keyword: String) -> Option<String> {
        self.trie.autocomplete(&keyword)
    }

    pub fn search(&mut self, keyword: String) -> Result<Viewer, String> {
        if self.search.items.is_empty() {
            return Err(format!("Err: no match for keyword {:?}", keyword));
        }

        // TODO instead of cloning the graph, make a subgraph
        let mut viewer = Viewer::new(format!("{} > {}", self.title, keyword), self.graph.clone());
        viewer.current = StatefulList::with_items(self.search.items.clone());
        viewer.update_adjacent();

        self.search = StatefulList::with_items(Vec::new());

        Ok(viewer)
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

    pub fn update_search(&mut self, key: String) {
        let nodes = self.current.items.clone();
        let search: Vec<String> = nodes.iter().filter(|id| id.starts_with(&key)).cloned().collect();

        self.search = StatefulList::with_items(search);
    }
}
