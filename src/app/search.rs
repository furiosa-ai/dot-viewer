use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };
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

    // direction: true if forward, false if backward (backspace)
    pub fn update_search(&mut self, mut key: String, direction: bool) {
        let matcher = SkimMatcherV2::default();
        let search: Vec<String> = if direction {
            self.search.items.iter().filter(|id| matcher.fuzzy_match(id, &key).is_some()).cloned().collect()
        } else {
            self.cache.items.clone()
        };
        self.search = StatefulList::with_items(search);

        if direction {
            self.cache = StatefulList::with_items(self.search.items.clone());
        } else {
            key.pop();
            let cache = self.current.items.iter().filter(|id| matcher.fuzzy_match(id, &key).is_some()).cloned().collect();
            self.cache = StatefulList::with_items(cache);
        }
    }
}
