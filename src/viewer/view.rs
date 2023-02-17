use crate::viewer::{
    error::{DotViewerError, DotViewerResult},
    utils::{List, Tree, Trie},
};

use dot_graph::Graph;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use regex::Regex;

type Matcher = fn(&str, &str, &Graph) -> Option<Vec<usize>>;

/// `View` holds a "view" of the graph that `dot-viewer` is dealing with.
///
/// Named as an analogy to the database concept of "view",
/// it holds a smaller portion of the original graph.
pub(crate) struct View {
    /// Title of the view
    pub title: String,

    /// Graph that the view is representing (a portion of the original graph)
    pub graph: Graph,

    /// Current focus
    pub focus: Focus,
    /// Topologically sorted list of all nodes in the view
    pub current: List<String>,
    /// List of previous nodes of the currently selected node
    pub prevs: List<String>,
    /// List of next nodes of the currently selected node
    pub nexts: List<String>,

    /// Keyword for match
    pub key: String,
    /// List of matching nodes given some input, with highlight index
    pub matches: List<(usize, Vec<usize>)>,

    /// Trie for user input autocompletion
    pub trie: Trie,

    /// Tree holding the subgraph tree of the view
    pub subtree: Tree,
}

#[derive(PartialEq)]
pub(crate) enum Focus {
    Current,
    Prev,
    Next,
}

impl View {
    /// Constructs a new `View`, given a `title` and a `graph`, which is a portion of the original
    /// graph.
    pub(crate) fn new(title: String, graph: Graph) -> View {
        let node_ids: Vec<String> = graph.topsort().iter().map(|&id| id.clone()).collect();

        let trie = Trie::new(&node_ids);

        let focus = Focus::Current;
        let current = List::from_iter(node_ids);
        let prevs = List::from_iter(Vec::new());
        let nexts = List::from_iter(Vec::new());

        let key = String::new();
        let matches = List::from_iter(Vec::new());

        let subtree = Tree::from_graph(&graph);

        let mut view = View { title, graph, focus, current, prevs, nexts, key, matches, trie, subtree };

        view.update_adjacent().expect("there is always a selected current node on initialization");

        view
    }

    /// Navigate to the selected adjacent node.
    pub(crate) fn goto_adjacent(&mut self) -> DotViewerResult<()> {
        let err = Err(DotViewerError::ViewerError("no node selected".to_string())); 

        match &self.focus {
            Focus::Prev => self.prevs.selected().map_or(err, |id| self.goto(&id)),
            Focus::Next => self.nexts.selected().map_or(err, |id| self.goto(&id)),
            _ => err,
        }
    }

    /// Navigate to the matched node.
    pub(crate) fn goto_match(&mut self) -> DotViewerResult<()> {
        self.matched_id().map_or(
            Err(DotViewerError::ViewerError("no node selected".to_string())),
            |id| self.goto(&id)
        )
    }

    /// Navigate to the currently selected node with `id`.
    /// The current node list will be focused on the selected node.
    pub(crate) fn goto(&mut self, id: &str) -> DotViewerResult<()> {
        let idx = self
            .current
            .find(id.to_string())
            .ok_or(DotViewerError::ViewerError(format!("no such node {id:?}")))?;

        self.current.select(idx);
        self.update_adjacent()?;

        Ok(())
    }

    /// Apply prefix filter on the view given prefix `key`.
    /// Returns `Ok` with a new `View` if the prefix yields a valid subgraph.
    pub(crate) fn filter(&mut self) -> DotViewerResult<View> {
        let node_ids: Vec<&String> =
            self.matches.items.iter().map(|(idx, _)| &self.current.items[*idx]).collect();
        let graph = self.graph.filter(&node_ids);

        if graph.is_empty() {
            return Err(DotViewerError::ViewerError(format!("no match for keyword {}", self.key)));
        }

        let view = Self::new(format!("{} - {}", self.title, self.key), graph);
        Ok(view)
    }

    /// Extract a subgraph from the view.
    /// Returns `Ok` with a new `View` if the selected subgraph id is valid.
    pub(crate) fn subgraph(&mut self) -> DotViewerResult<View> {
        let key = self
            .subtree
            .selected()
            .ok_or(DotViewerError::ViewerError("no subgraph selected".to_string()))?;

        let subgraph =
            self.graph.subgraph(&key).map_err(|e| DotViewerError::ViewerError(e.to_string()))?;

        if subgraph.is_empty() {
            return Err(DotViewerError::ViewerError("empty graph".to_string()));
        }

        let title = &self.title;
        let view = Self::new(format!("{title} - {key}"), subgraph);

        Ok(view)
    }

    /// Get neighbors graph from the selected id in the view.
    /// Returns `Ok` with a new `View` if the depth is valid.
    pub(crate) fn neighbors(&mut self, depth: usize) -> DotViewerResult<View> {
        let id = self.current_id();
        let graph = self.graph.neighbors(&id, depth)?;

        if graph.is_empty() {
            return Err(DotViewerError::ViewerError("cannot define a neighbors graph".to_string()));
        }

        let view = Self::new(format!("{} - neighbors-{}-{}", self.title, id, depth), graph);
        Ok(view)
    }

    /// Autocomplete a given keyword, coming from `tab` keybinding.
    pub(crate) fn autocomplete(&mut self, key: &str) -> Option<String> {
        self.trie.autocomplete(key)
    }

    /// Update prevs and nexts lists based on the selected current node.
    pub(crate) fn update_adjacent(&mut self) -> DotViewerResult<()> {
        let id = self.current_id();

        let prevs = self.graph.froms(&id)?;
        let prevs = prevs.iter().map(|n| n.to_string());
        self.prevs = List::from_iter(prevs);

        let nexts = self.graph.tos(&id)?;
        let nexts = nexts.iter().map(|n| n.to_string());
        self.nexts = List::from_iter(nexts);

        Ok(())
    }

    /// Update matches based on the given matching function `match` with input `key`.
    fn update_matches(&mut self, matcher: Matcher, key: &str) {
        let matches: Vec<(usize, Vec<usize>)> = self
            .current
            .items
            .par_iter()
            .enumerate()
            .filter_map(|(idx, id)| matcher(id, key, &self.graph).map(|highlight| (idx, highlight)))
            .collect();

        self.key = key.to_string();
        self.matches = List::from_iter(matches);
    }

    /// Update matches in fuzzy search mode.
    /// Fuzzy matcher matches input against node ids.
    pub(crate) fn update_fuzzy(&mut self, key: &str) {
        self.update_matches(match_fuzzy, key);
    }

    /// Update matches in regex search mode.
    /// Regex matcher matches input against node represented in raw dot format string.
    pub(crate) fn update_regex(&mut self, key: &str) {
        self.update_matches(match_regex, key);
    }

    /// Update trie based on the current matches.
    pub(crate) fn update_trie(&mut self) {
        let nodes: Vec<String> = self
            .matches
            .items
            .par_iter()
            .map(|(idx, _)| self.current.items[*idx].clone())
            .collect();
        self.trie = Trie::new(&nodes);
    }

    pub(crate) fn current_id(&self) -> String {
        self.current.selected().expect("there is always a current id selected in a view")
    }

    pub(crate) fn matched_id(&self) -> Option<String> {
        self.matches.selected().map(|(idx, _)| self.current.items[idx].clone())
    }

    pub(crate) fn progress_current(&self) -> String {
        let idx = self.current.state.selected().unwrap();
        let len = self.current.items.len();
        let percentage = (idx as f32 / len as f32) * 100_f32;

        format!("[{} / {} ({:.3}%)]", idx + 1, len, percentage)
    }

    pub(crate) fn progress_matches(&self) -> String {
        let idx = self.matches.state.selected().unwrap();
        let len = self.matches.items.len();
        let percentage = (idx as f32 / len as f32) * 100_f32;

        format!("[{} / {} ({:.3}%)]", idx + 1, len, percentage)
    }
}

fn match_fuzzy(id: &str, key: &str, _graph: &Graph) -> Option<Vec<usize>> {
    let matcher = SkimMatcherV2::default();

    matcher.fuzzy_indices(id, key).map(|(_, idxs)| idxs)
}

fn match_regex(id: &str, key: &str, graph: &Graph) -> Option<Vec<usize>> {
    if let Ok(matcher) = Regex::new(key) {
        let node = graph.search_node(&id.to_string()).unwrap();

        let mut buffer = Vec::new();
        node.to_dot(0, &mut buffer).expect("to_dot should succeed");
        let raw = std::str::from_utf8(&buffer).unwrap();

        let highlight: Vec<usize> = (0..id.len()).collect();
        matcher.is_match(raw).then_some(highlight)
    } else {
        None
    }
}
