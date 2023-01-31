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

    pub fn autocomplete(&mut self, key: &str) -> Result<String, DotViewerError> {
        self.trie.autocomplete(key).map_or(
            Err(DotViewerError::GraphError(format!(
                "no autocomplete for key {}",
                key
            ))),
            |key| Ok(key)
        )
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

    pub fn update_fuzzy_fwd(&mut self, key: String) {
        let matcher = SkimMatcherV2::default();

        self.cache = StatefulList::with_items(self.matches.items.clone());

        let matches: Vec<(String, Vec<usize>)> = self
            .matches
            .items
            .par_iter()
            .filter_map(|(id, _)| {
                if let Some((_, idxs)) = matcher.fuzzy_indices(id, &key) {
                    Some((id.clone(), idxs))
                } else {
                    None
                }
            })
            .collect();

        self.matches = StatefulList::with_items(matches);
    }

    pub fn update_fuzzy_bwd(&mut self, mut key: String) {
        let matcher = SkimMatcherV2::default();

        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();

        let cache: Vec<(String, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .filter_map(|id| {
                if let Some((_, idxs)) = matcher.fuzzy_indices(id, &key) {
                    Some((id.clone(), idxs))
                } else {
                    None
                }
            })
            .collect();

        self.cache = StatefulList::with_items(cache);
    }

    pub fn update_regex_fwd(&mut self, key: String) {
        self.cache = StatefulList::with_items(self.matches.items.clone());

        if let Ok(matcher) = Regex::new(&key) {
            let matches: Vec<(String, Vec<usize>)> = self
                .current
                .items
                .par_iter()
                .filter_map(|id| {
                    let node = self.graph.search(id).unwrap();
                    let raw = node.to_dot(0);

                    if matcher.is_match(&raw) {
                        Some((id.clone(), Vec::new()))
                    } else {
                        None
                    }
                })
                .collect();

            self.matches = StatefulList::with_items(matches);
        }
    }

    pub fn update_regex_bwd(&mut self, mut key: String) {
        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();

        if let Ok(matcher) = Regex::new(&key) {
            let cache: Vec<(String, Vec<usize>)> = self
                .current
                .items
                .par_iter()
                .filter_map(|id| {
                    let node = self.graph.search(id).unwrap();
                    let raw = node.to_dot(0);

                    if matcher.is_match(&raw) {
                        Some((id.clone(), Vec::new()))
                    } else {
                        None
                    }
                })
                .collect();

            self.cache = StatefulList::with_items(cache);
        }
    }

    pub fn update_filter(&mut self, mut key: String) {
        let matches: Vec<(String, Vec<usize>)> = self
            .matches
            .items
            .par_iter()
            .filter_map(|(id, _)| {
                if id.starts_with(&key) {
                    let highlight: Vec<usize> = (0..key.len()).collect();
                    Some((id.clone(), highlight))
                } else {
                    None
                }
            })
            .collect();

        self.matches = StatefulList::with_items(matches);

        key.pop();

        let cache: Vec<(String, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .filter_map(|id| {
                if id.starts_with(&key) {
                    let highlight: Vec<usize> = (0..key.len()).collect();
                    Some((id.clone(), highlight))
                } else {
                    None
                }
            })
            .collect();

        self.cache = StatefulList::with_items(cache);
    }

    pub fn update_filter_fwd(&mut self, key: String) {
        self.cache = StatefulList::with_items(self.matches.items.clone());

        let matches: Vec<(String, Vec<usize>)> = self
            .matches
            .items
            .par_iter()
            .filter_map(|(id, _)| {
                if id.starts_with(&key) {
                    let highlight: Vec<usize> = (0..key.len()).collect();
                    Some((id.clone(), highlight))
                } else {
                    None
                }
            })
            .collect();

        self.matches = StatefulList::with_items(matches);
    }

    pub fn update_filter_bwd(&mut self, mut key: String) {
        self.matches = StatefulList::with_items(self.cache.items.clone());

        key.pop();

        let cache: Vec<(String, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .filter_map(|id| {
                if id.starts_with(&key) {
                    let highlight: Vec<usize> = (0..key.len()).collect();
                    Some((id.clone(), highlight))
                } else {
                    None
                }
            })
            .collect();

        self.cache = StatefulList::with_items(cache);
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
