use crate::app::{
    error::{DotViewerError, Res},
    utils::{List, Tree, Trie},
};
use dot_graph::Graph;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use regex::Regex;

type Matcher = fn(&str, &str, &Graph) -> Option<(String, Vec<usize>)>;

/// `View` holds a "view" of the graph that `dot-viewer` is dealing with.
///
/// Named as an analogy to the database concept of "view",
/// it holds a smaller portion of the original graph.
pub struct View {
    /// Title of the view
    pub title: String,

    /// Graph that the view is representing (a portion of the original graph)
    pub graph: Graph,

    /// Topologically sorted list of all nodes in the view
    pub current: List<String>,

    /// List of previous nodes of the currently selected node
    pub prevs: List<String>,
    /// List of next nodes of the currently selected node
    pub nexts: List<String>,

    /// List of matching nodes given some input, with highlight index
    pub matches: List<(String, Vec<usize>)>,

    /// Trie for user input autocompletion
    pub trie: Trie,

    /// Tree holding the subgraph tree of the view
    pub subtree: Tree,
}

impl View {
    /// Constructs a new `View`, given a `title` and a `graph`, which is a portion of the original
    /// graph.
    pub fn new(title: String, graph: Graph) -> View {
        let nodes: Vec<String> = graph.topsort();
        let subtree = Tree::with_graph(&graph);

        let mut view = View {
            title,
            graph,
            current: List::with_items(nodes.clone()),
            prevs: List::with_items(Vec::new()),
            nexts: List::with_items(Vec::new()),
            matches: List::with_items(Vec::new()),
            trie: Trie::new(&nodes),
            subtree,
        };

        view.update_adjacent().expect("there is always a selected current node on initialization");

        view
    }

    /// Navigate to the currently selected node with `id`.
    /// The current node list will be focused on the selected node.
    pub fn goto(&mut self, id: &str) -> Res {
        let idx = self.current.find(id.to_string());

        idx.map_or(Err(DotViewerError::ViewerError(format!("no such node {:?}", id))), |idx| {
            self.current.select(idx);
            self.update_adjacent()?;

            Ok(None)
        })
    }

    /// Apply prefix filter on the view given prefix `key`.
    /// Returns `Ok` with a new `View` if the prefix yields a valid subgraph.
    pub fn filter(&mut self, prefix: &str) -> Result<View, DotViewerError> {
        let graph = self.graph.filter(prefix);

        graph.map_or(
            Err(DotViewerError::ViewerError(format!("no match for prefix {}", prefix))),
            |graph| {
                let view = Self::new(format!("{} - {}", self.title, prefix), graph);
                Ok(view)
            },
        )
    }

    /// Extract a subgraph from the view.
    /// Returns `Ok` with a new `View` if the selected subgraph id is valid.
    pub fn subgraph(&mut self) -> Result<View, DotViewerError> {
        self.subtree.selected().map_or(
            Err(DotViewerError::ViewerError("no subgraph selected".to_string())),
            |key| {
                self.graph.subgraph(&key).map_or_else(
                    |e| Err(DotViewerError::ViewerError(e.to_string())),
                    |graph| {
                        graph.map_or(
                            Err(DotViewerError::ViewerError("empty graph".to_string())),
                            |graph| {
                                let view = Self::new(format!("{} - {}", self.title, key), graph);
                                Ok(view)
                            },
                        )
                    },
                )
            },
        )
    }

    /// Autocomplete a given keyword, coming from `tab` keybinding.
    pub fn autocomplete(&mut self, key: &str) -> Option<String> {
        self.trie.autocomplete(key)
    }

    /// Update prevs and nexts lists based on the selected current node.
    pub fn update_adjacent(&mut self) -> Result<(), DotViewerError> {
        let id = self.current_id().unwrap();

        let prevs = self.graph.froms(&id)?.iter().map(|n| n.to_string()).collect();
        self.prevs = List::with_items(prevs);

        let nexts = self.graph.tos(&id)?.iter().map(|n| n.to_string()).collect();
        self.nexts = List::with_items(nexts);

        Ok(())
    }

    /// Update matches based on the given matching function `match` with input `key`.
    fn update_matches(&mut self, matcher: Matcher, key: &str) {
        let matches: Vec<(String, Vec<usize>)> =
            self.current.items.par_iter().filter_map(|id| matcher(id, key, &self.graph)).collect();

        self.matches = List::with_items(matches);
    }

    /// Update matches in fuzzy search mode.
    /// Fuzzy matcher matches input against node ids.
    pub fn update_fuzzy(&mut self, key: String) {
        self.update_matches(match_fuzzy, &key);
    }

    /// Update matches in regex search mode.
    /// Regex matcher matches input against node represented in raw dot format string.
    pub fn update_regex(&mut self, key: String) {
        self.update_matches(match_regex, &key);
    }

    /// Update matches in prefix filter mode.
    pub fn update_filter(&mut self, key: String) {
        self.update_matches(match_prefix, &key);
    }

    /// Update trie based on the current matches.
    pub fn update_trie(&mut self) {
        let nodes: Vec<String> = self.matches.items.par_iter().map(|(id, _)| id.clone()).collect();
        self.trie = Trie::new(&nodes);
    }

    pub fn current_id(&self) -> Option<String> {
        self.current.selected()
    }

    pub fn matched_id(&self) -> Option<String> {
        self.matches.selected().map(|(item, _)| item)
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

fn match_fuzzy(id: &str, key: &str, _graph: &Graph) -> Option<(String, Vec<usize>)> {
    let matcher = SkimMatcherV2::default();

    matcher.fuzzy_indices(id, key).map(|(_, idxs)| (id.to_string(), idxs))
}

fn match_regex(id: &str, key: &str, graph: &Graph) -> Option<(String, Vec<usize>)> {
    if let Ok(matcher) = Regex::new(key) {
        let node = graph.search_node(&id.to_string()).unwrap();

        let mut buffer = Vec::new();
        node.to_dot(0, &mut buffer).expect("to_dot should succeed");
        let raw = std::str::from_utf8(&buffer).unwrap();

        matcher.is_match(raw).then_some((id.to_string(), Vec::new()))
    } else {
        None
    }
}

fn match_prefix(id: &str, key: &str, _graph: &Graph) -> Option<(String, Vec<usize>)> {
    let highlight: Vec<usize> = (0..key.len()).collect();
    id.starts_with(key).then_some((id.to_string(), highlight))
}
