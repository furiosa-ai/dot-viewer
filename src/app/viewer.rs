use crate::app::{
    error::{DotViewerError, Res},
    utils::{StatefulList, Trie},
};
use dot_graph::Graph;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use regex::Regex;

pub struct Viewer {
    pub title: String,

    pub graph: Graph,

    pub current: StatefulList<String>,
    pub prevs: StatefulList<String>,
    pub nexts: StatefulList<String>,

    pub matches: StatefulList<(String, Vec<usize>)>,
    pub cache: StatefulList<(String, Vec<usize>)>,
    pub trie: Trie,
}

impl Viewer {
    pub fn new(title: String, graph: Graph) -> Viewer {
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();

        let mut viewer = Viewer {
            title,
            graph,
            current: StatefulList::with_items(nodes),
            prevs: StatefulList::with_items(Vec::new()),
            nexts: StatefulList::with_items(Vec::new()),
            matches: StatefulList::with_items(Vec::new()),
            cache: StatefulList::with_items(Vec::new()),
            trie: Trie::new(&Vec::new()),
        };

        viewer.update_adjacent();

        viewer
    }

    pub fn current(&self) -> Option<String> {
        self.current.selected()
    }

    pub fn matched(&self) -> Option<String> {
        self.matches.selected().map(|(item, _)| item)
    }

    pub fn goto(&mut self, id: &str) -> Res {
        let idx = self.current.find(id.to_string());

        idx.map_or(
            Err(DotViewerError::GraphError(format!("no such node {:?}", id))),
            |idx| {
                self.current.select(idx);
                self.update_adjacent();

                // TODO
                // manually set offset to keep goto-ed node in the middle of the list
                // with modified (forked) tui-rs
                let offset = self.current.state.offset_mut();
                if idx >= 10 {
                    *offset = idx - 10;
                }

                Ok(None)
            },
        )
    }

    pub fn filter(&mut self, key: String) -> Result<Viewer, DotViewerError> {
        let graph = self.graph.filter(&key);

        graph.map_or(
            Err(DotViewerError::GraphError(format!(
                "no match for prefix {}",
                key
            ))),
            |graph| {
                let viewer = Self::new(format!("{} - {}", self.title, key), graph);
                Ok(viewer)
            },
        )
    }

    pub fn autocomplete(&mut self, key: &str) -> Option<String> {
        self.trie.autocomplete(key)
    }

    pub fn update_adjacent(&mut self) {
        let id = self.current().unwrap();

        let prevs = self
            .graph
            .froms(&id)
            .iter()
            .map(|n| n.to_string())
            .collect();
        self.prevs = StatefulList::with_items(prevs);

        let nexts = self.graph.tos(&id).iter().map(|n| n.to_string()).collect();
        self.nexts = StatefulList::with_items(nexts);
    }

    fn update_matches(&mut self, matcher: fn(&str, &str, &Option<Graph>) -> Option<(String, Vec<usize>)>, key: &str, graph: &Option<Graph>) {
        let matches: Vec<(String, Vec<usize>)> = self
            .matches
            .items
            .par_iter()
            .filter_map(|(id, _)| matcher(id, key, graph))
            .collect();

        self.matches = StatefulList::with_items(matches);
    }

    fn update_cache(&mut self, matcher: fn(&str, &str, &Option<Graph>) -> Option<(String, Vec<usize>)>, key: &str, graph: &Option<Graph>) {
        let cache: Vec<(String, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .filter_map(|item| matcher(item, key, graph))
            .collect();

        self.cache = StatefulList::with_items(cache);
    }

    fn match_fuzzy(id: &str, key: &str, _graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        let matcher = SkimMatcherV2::default();

        matcher.fuzzy_indices(id, key).map(|(_, idxs)| (id.to_string(), idxs))
    }

    pub fn update_fuzzy(&mut self, mut key: String) {
        self.update_matches(Self::match_fuzzy, &key, &None);

        key.pop();
        self.update_cache(Self::match_fuzzy, &key, &None);
    }

    pub fn update_fuzzy_fwd(&mut self, key: String) {
        self.cache = StatefulList::with_items(self.matches.items.clone());
        self.update_matches(Self::match_fuzzy, &key, &None);
    }

    pub fn update_fuzzy_bwd(&mut self, mut key: String) {
        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();
        self.update_cache(Self::match_fuzzy, &key, &None);
    }

    fn match_regex(id: &str, key: &str, graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        if let Ok(matcher) = Regex::new(&key) {
            let graph = graph.as_ref().unwrap();
            let node = graph.search(id).unwrap();
            let raw = node.to_dot(0);

            if matcher.is_match(&raw) {
                Some((id.to_string(), Vec::new()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn update_regex(&mut self, mut key: String) {
        self.update_matches(Self::match_regex, &key, &Some(self.graph.clone()));

        key.pop();
        self.update_cache(Self::match_regex, &key, &Some(self.graph.clone()));
    }

    pub fn update_regex_fwd(&mut self, key: String) {
        self.cache = StatefulList::with_items(self.matches.items.clone());
        self.update_matches(Self::match_regex, &key, &Some(self.graph.clone()));
    }

    pub fn update_regex_bwd(&mut self, mut key: String) {
        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();
        self.update_cache(Self::match_regex, &key, &Some(self.graph.clone()));
    }

    fn match_filter(id: &str, key: &str, _graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        if id.starts_with(&key) {
            let highlight: Vec<usize> = (0..key.len()).collect();
            Some((id.to_string(), highlight))
        } else {
            None
        }
    }

    pub fn update_filter(&mut self, mut key: String) {
        self.update_matches(Self::match_filter, &key, &None);

        key.pop();
        self.update_cache(Self::match_filter, &key, &None);
    }
    
    pub fn update_filter_fwd(&mut self, key: String) {
        self.cache = StatefulList::with_items(self.matches.items.clone());
        self.update_matches(Self::match_filter, &key, &None);
    }

    pub fn update_filter_bwd(&mut self, mut key: String) {
        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();
        self.update_cache(Self::match_filter, &key, &None);
    }

    pub fn update_trie(&mut self) {
        let nodes: Vec<String> = self.matches.items.par_iter().map(|(id, _)| id.clone()).collect();
        self.trie = Trie::new(&nodes);
    }

    pub fn progress_current(&self) -> String {
        let idx = self.current.state.selected().unwrap();
        let len = self.current.items.len();
        let percentage = (idx as f32 / len as f32) * 100_f32;

        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    }

    pub fn progress_matches(&self) -> String {
        if let Some(idx) = self.matches.state.selected() {
            let len = self.matches.items.len();
            let percentage = (idx as f32 / len as f32) * 100_f32;

            format!("Searching... [{} / {} ({:.3}%)]", idx, len, percentage)
        } else {
            "No Match...".to_string()
        }
    }
}
