use crate::app::{
    error::{Res, DotViewerError},
    utils::list::StatefulList,
};
use dot_graph::Graph;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub struct Viewer {
    pub title: String,

    pub graph: Graph,

    pub current: StatefulList<String>,
    pub prevs: StatefulList<String>,
    pub nexts: StatefulList<String>,

    pub search: StatefulList<(String, Vec<usize>)>,
    pub cache: StatefulList<(String, Vec<usize>)>,

    pub filter: StatefulList<(String, Vec<usize>)>,
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
            search: StatefulList::with_items(Vec::new()),
            cache: StatefulList::with_items(Vec::new()),
            filter: StatefulList::with_items(Vec::new()),
        };

        viewer.update_adjacent();

        viewer
    }

    pub fn current(&self) -> Option<String> {
        self.current.selected()
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
            })
    }

    pub fn filter(&mut self, key: String) -> Result<Viewer, DotViewerError> {
        let graph = self.graph.filter(&key);

        graph.map_or(
            Err(DotViewerError::GraphError(format!("no match for prefix {}", key))),
            |graph| {
                let viewer = Self::new(format!("{} - {}", self.title, key), graph);
                Ok(viewer)
            })
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
            let res = matcher.fuzzy_indices(id, &key);
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
            let res = matcher.fuzzy_indices(id, &key);
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

    pub fn progress_current(&self) -> String {
        let idx = self.current.state.selected().unwrap();
        let len = self.current.items.len();
        let percentage = (idx as f32 / len as f32) * 100_f32;

        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    }

    pub fn progress_search(&self) -> String {
        if let Some(idx) = self.search.state.selected() {
            let len = self.search.items.len();
            let percentage = (idx as f32 / len as f32) * 100_f32;

            format!("Searching... [{} / {} ({:.3}%)]", idx, len, percentage)
        } else {
            "No Match...".to_string()
        }
    }
}
