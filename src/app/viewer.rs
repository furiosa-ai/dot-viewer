use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };
use dot_graph::structs::Graph;
use crate::app::{
    app::{ App, Viewer, Mode, Navigate, Input },
    utils::list::StatefulList,
};

impl App {
    pub fn goto(&mut self) {
        let id = self.selected();
        let viewer = self.tabs.selected();
        viewer.goto(id);
    }

    pub fn filter(&mut self) {
        let viewer = self.tabs.selected();
        let viewer = viewer.filter(self.input.clone());
        match viewer {
            Ok(viewer) => self.tabs.open(viewer),
            Err(msg) => self.errormsg = Some(msg),
        }
    }

    pub fn selected(&mut self) -> Option<String> {
        match &self.mode {
            Mode::Navigate(nav) => {
                let viewer = self.tabs.selected();

                match nav {
                    Navigate::Current => viewer.current.selected(),
                    Navigate::Prevs => viewer.prevs.selected(),
                    Navigate::Nexts => viewer.nexts.selected(),
                }
            },
            Mode::Input(input) => {
                let viewer = self.tabs.selected();

                let item = match input {
                    Input::Search => viewer.search.selected(),
                    Input::Filter => viewer.filter.selected(),
                };

                match item {
                    Some((item, _)) => Some(item),
                    None => None
                }
            },
        }
    }
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

    pub fn progress_current(&self) -> String {
        let idx = self.current.state.selected().unwrap();
        let len = self.current.items.len();
        let percentage = (idx as f32 / len as f32) * 100 as f32;

        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    }

    pub fn progress_search(&self) -> String {
        let idx = self.search.state.selected().unwrap();
        let len = self.search.items.len();
        let percentage = (idx as f32 / len as f32) * 100 as f32;

        format!("Searching... [{} / {} ({:.3}%)]", idx, len, percentage)
    }

    pub fn current(&self) -> Option<String> {
        self.current.selected()
    }

    pub fn goto(&mut self, id: Option<String>) -> Option<String> {
        match id {
            Some(id) => {
                let idx = self.current.find(id.to_string());
                match idx {
                    Some(idx) => {
                        self.current.select(idx);
                        self.update_adjacent();
                        None
                    },
                    None => Some(format!("Err: no such node {:?}", id))
                }
            },
            None => Some("Err: no node selected".to_string()),
        }
    }

    pub fn filter(&mut self, key: String) -> Result<Viewer, String> {
        if self.filter.items.is_empty() {
            return Err(format!("Err: no match for key {:?}", key));
        }

        let ids = self.filter.items.iter().map(|item| item.0.clone()).collect();
        let graph = self.graph.sub(&ids);
        let viewer = Self::new(format!("{} > {}", self.title, key), graph);

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
