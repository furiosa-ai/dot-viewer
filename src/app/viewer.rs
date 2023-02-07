use crate::app::{
    error::{DotViewerError, Res},
    utils::{List, Tree, Trie},
};
use dot_graph::Graph;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use regex::Regex;

type Matcher = fn(&str, &str, &Option<Graph>) -> Option<(String, Vec<usize>)>;

pub struct Viewer {
    pub title: String,

    pub graph: Graph,

    pub current: List<String>,
    pub prevs: List<String>,
    pub nexts: List<String>,

    pub matches: List<(String, Vec<usize>)>,
    pub trie: Trie,

    pub tree: Tree,
}

impl Viewer {
    pub fn new(title: String, graph: Graph) -> Viewer {
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();
        let tree = Tree::with_graph(&graph);

        let mut viewer = Viewer {
            title,
            graph,
            current: List::with_items(nodes.clone()),
            prevs: List::with_items(Vec::new()),
            nexts: List::with_items(Vec::new()),
            matches: List::with_items(Vec::new()),
            trie: Trie::new(&nodes),
            tree,
        };

        let _ = viewer.update_adjacent();

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
            Err(DotViewerError::ViewerError(format!(
                "no such node {:?}",
                id
            ))),
            |idx| {
                self.current.select(idx);
                self.update_adjacent()?;

                Ok(None)
            },
        )
    }

    pub fn filter(&mut self, key: &str) -> Result<Viewer, DotViewerError> {
        let graph = self.graph.filter(key);

        graph.map_or(
            Err(DotViewerError::ViewerError(format!(
                "no match for prefix {}",
                key
            ))),
            |graph| {
                let viewer = Self::new(format!("{} - {}", self.title, key), graph);
                Ok(viewer)
            },
        )
    }

    pub fn subgraph(&mut self) -> Result<Viewer, DotViewerError> {
        self.tree.selected().map_or(
            Err(DotViewerError::ViewerError(
                "no subgraph selected".to_string(),
            )),
            |key| {
                self.graph.subgraph(&key).map_or_else(
                    |e| Err(DotViewerError::ViewerError(e.to_string())),
                    |graph| {
                        graph.map_or(
                            Err(DotViewerError::ViewerError("empty graph".to_string())),
                            |graph| {
                                let viewer = Self::new(key, graph);
                                Ok(viewer)
                            },
                        )
                    },
                )
            },
        )
    }

    pub fn autocomplete(&mut self, key: &str) -> Option<String> {
        self.trie.autocomplete(key)
    }

    pub fn update_adjacent(&mut self) -> Result<(), DotViewerError> {
        let id = self.current().unwrap();

        let prevs = self
            .graph
            .froms(&id)?
            .iter()
            .map(|n| n.to_string())
            .collect();
        self.prevs = List::with_items(prevs);

        let nexts = self.graph.tos(&id)?.iter().map(|n| n.to_string()).collect();
        self.nexts = List::with_items(nexts);

        Ok(())
    }

    fn update_matches(&mut self, matcher: Matcher, key: &str, graph: &Option<Graph>) {
        let matches: Vec<(String, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .filter_map(|id| matcher(id, key, graph))
            .collect();

        self.matches = List::with_items(matches);
    }

    fn match_fuzzy(id: &str, key: &str, _graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        let matcher = SkimMatcherV2::default();

        matcher
            .fuzzy_indices(id, key)
            .map(|(_, idxs)| (id.to_string(), idxs))
    }

    pub fn update_fuzzy(&mut self, key: String) {
        self.update_matches(Self::match_fuzzy, &key, &None);
    }

    fn match_regex(id: &str, key: &str, graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        if let Ok(matcher) = Regex::new(key) {
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

    pub fn update_regex(&mut self, key: String) {
        self.update_matches(Self::match_regex, &key, &Some(self.graph.clone()));
    }

    fn match_filter(id: &str, key: &str, _graph: &Option<Graph>) -> Option<(String, Vec<usize>)> {
        if id.starts_with(key) {
            let highlight: Vec<usize> = (0..key.len()).collect();
            Some((id.to_string(), highlight))
        } else {
            None
        }
    }

    pub fn update_filter(&mut self, key: String) {
        self.update_matches(Self::match_filter, &key, &None);
    }

    pub fn update_trie(&mut self) {
        let nodes: Vec<String> = self
            .matches
            .items
            .par_iter()
            .map(|(id, _)| id.clone())
            .collect();
        self.trie = Trie::new(&nodes);
    }

    pub fn progress_current(&self) -> String {
        let idx = self.current.state.selected().unwrap();
        let len = self.current.items.len();
        let percentage = (idx as f32 / len as f32) * 100_f32;

        format!("[{} / {} ({:.3}%)]", idx + 1, len, percentage)
    }

    pub fn progress_matches(&self) -> String {
        if let Some(idx) = self.matches.state.selected() {
            let len = self.matches.items.len();
            let percentage = (idx as f32 / len as f32) * 100_f32;

            format!("[{} / {} ({:.3}%)]", idx + 1, len, percentage)
        } else {
            "No Match...".to_string()
        }
    }
}
