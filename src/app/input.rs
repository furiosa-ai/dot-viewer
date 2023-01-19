use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };
use crate::app::{
    app::Viewer,
    utils::list::StatefulList,
};

impl Viewer {
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

    pub fn filter(&mut self, key: String) -> Result<Viewer, String> {
        if self.filter.items.is_empty() {
            return Err(format!("Err: no match for key {:?}", key));
        }

        // TODO instead of cloning the graph, make a subgraph
        let mut viewer = Viewer::new(format!("{} > {}", self.title, key), self.graph.clone());
        viewer.current = StatefulList::with_items(self.filter.items.iter().map(|item| item.0.clone()).collect());
        viewer.update_adjacent();

        self.filter = StatefulList::with_items(Vec::new());

        Ok(viewer)
    }

    pub fn update_adjacent(&mut self) {
        let id = self.current().unwrap();

        let prevs = self.graph.froms(&id).iter().map(|n| n.to_string()).collect();
        self.prevs = StatefulList::with_items(prevs);

        let nexts = self.graph.tos(&id).iter().map(|n| n.to_string()).collect();
        self.nexts = StatefulList::with_items(nexts);
    }

    pub fn update_search_fwd(&mut self, key: String) {
        let matcher = SkimMatcherV2::default();

        self.cache = StatefulList::with_items(self.search.items.clone());

        let mut search: Vec<(String, Vec<usize>)> = Vec::new();
        for id in &self.search.items {
            let id = &id.0;
            let res = matcher.fuzzy_indices(&id, &key);
            if let Some((_, idxs)) = res {
                search.push((id.clone(), idxs));
            }
        }
        self.search = StatefulList::with_items(search);
    }

    pub fn update_search_bwd(&mut self, mut key: String) {
        let matcher = SkimMatcherV2::default();

        self.search = StatefulList::with_items(self.cache.items.clone());

        key.pop();

        let mut cache: Vec<(String, Vec<usize>)> = Vec::new();
        for id in &self.current.items {
            let res = matcher.fuzzy_indices(&id, &key);
            if let Some((_, idxs)) = res {
                cache.push((id.clone(), idxs));
            }
        }
        self.cache = StatefulList::with_items(cache);
    }

    pub fn update_filter(&mut self, key: String) {
        let mut filter: Vec<(String, Vec<usize>)> = Vec::new();
        let nodes = self.current.items.clone();
        let highlight: Vec<usize> = (0..key.len()).collect();
        for id in nodes {
            if id.starts_with(&key) {
                filter.push((id.clone(), highlight.clone()));
            }
        }

        self.filter = StatefulList::with_items(filter);
    }
}
