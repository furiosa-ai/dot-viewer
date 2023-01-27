use crate::app::{
    app::{App, Input, Mode, Navigate, Res, Viewer},
    utils::list::StatefulList,
};
use dot_graph::structs::Graph;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::io::Write;

use super::error::ViewerError;

impl App {
    pub fn goto(&mut self) -> Res {
        let id = self.selected();
        let viewer = self.tabs.selected();
        viewer.goto(id)
    }

    pub fn filter(&mut self) -> Res {
        let viewer = self.tabs.selected();
        let viewer = viewer.filter(self.input.clone())?;
        self.tabs.open(viewer);

        Ok(None)
    }

    pub fn export(&mut self) -> Res {
        let viewer = self.tabs.selected();
        let graph = &viewer.graph;

        let filename: String = viewer
            .title
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        match Self::write(filename, graph.to_dot()) {
            Ok(succ) => Ok(Some(succ)),
            Err(msg) => Err(ViewerError::IOError(msg.to_string())),
        }
    }

    pub fn xdot(&mut self) -> Res {
        let exists = std::path::Path::new("./exports/current.dot").exists();

        if exists {
            let xdot = std::process::Command::new("xdot")
                .arg("./exports/current.dot")
                .spawn();

            match xdot {
                Ok(_) => Ok(None),
                Err(_) => Err(ViewerError::XdotError)
            }
        } else {
            Err(ViewerError::IOError("no exports/current.dot".to_string()))
        }
    }

    pub fn neighbors(&mut self, depth: usize) -> Res {
        let viewer = self.tabs.selected();
        let graph = &viewer.graph;
        let node = &viewer.current().unwrap();

        let filename = format!("{}-{}", node.clone(), depth);
        let neighbors = graph.neighbors(node, depth);
        match neighbors {
            Some(neighbors) => {
                let contents = neighbors.to_dot();
                match Self::write(filename, contents) {
                    Ok(succ) => Ok(Some(succ)),
                    Err(msg) => Err(ViewerError::IOError(msg.to_string())),
                }
            }
            None => Err(ViewerError::TODOError("empty graph".to_string())),
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
            }
            Mode::Input(input) => {
                let viewer = self.tabs.selected();

                let item = match input {
                    Input::Search => viewer.search.selected(),
                    Input::Filter => viewer.filter.selected(),
                };

                item.map(|(item, _)| item)
            }
        }
    }

    fn write(filename: String, contents: String) -> Result<String, std::io::Error> {
        std::fs::create_dir_all("./exports")?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(format!("./exports/{}.dot", filename))?;
        file.write_all(contents.as_bytes())?;

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("./exports/current.dot")?;
        file.write_all(contents.as_bytes())?;

        Ok(format!("file successfully written to {}", filename))
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

    pub fn current(&self) -> Option<String> {
        self.current.selected()
    }

    pub fn goto(&mut self, id: Option<String>) -> Res {
        match id {
            Some(id) => {
                let idx = self.current.find(id.to_string());
                match idx {
                    Some(idx) => {
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
                    }
                    None => Err(ViewerError::GoToError(format!("no such node {:?}", id))),
                }
            }
            None => Err(ViewerError::GoToError("no node selected".to_string())),
        }
    }

    pub fn filter(&mut self, key: String) -> Result<Viewer, ViewerError> {
        let graph = self.graph.filter(&key);

        match graph {
            Some(graph) => {
                let viewer = Self::new(format!("{} - {}", self.title, key), graph);
                Ok(viewer)
            }
            None => Err(ViewerError::FilterError(format!(
                "no match for prefix {}",
                key
            ))),
        }
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
