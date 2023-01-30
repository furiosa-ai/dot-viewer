use crate::app::{
    error::{Res, ViewerError},
    modes::{Input, Navigate, Mode},
    utils::{list::StatefulList, tabs::StatefulTabs},
    viewer::Viewer,
};
use dot_graph::parser::parse;
use std::io::Write;

pub struct App {
    pub quit: bool,
    pub mode: Mode,

    pub tabs: StatefulTabs<Viewer>,

    pub input: String,
    pub history: Vec<String>,

    pub result: Res,
}

impl App {
    pub fn new(path: &str) -> App {
        let graph = parse(path);
        let viewer = Viewer::new("DAG".to_string(), graph);
        let tabs = StatefulTabs::with_tabs(vec![viewer]);

        App {
            quit: false,
            mode: Mode::Navigate(Navigate::Current),
            tabs,
            input: String::from(""),
            history: Vec::new(),
            result: Ok(None),
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
                Err(_) => Err(ViewerError::XdotError),
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

    pub fn to_nav_mode(&mut self) {
        self.mode = Mode::Navigate(Navigate::Current);
        self.input = "".to_string();
    }

    pub fn to_input_mode(&mut self, input: Input) {
        self.mode = Mode::Input(input.clone());

        let viewer = self.tabs.selected();
        let init: Vec<(String, Vec<usize>)> = viewer
            .current
            .items
            .iter()
            .map(|id| (id.clone(), Vec::new()))
            .collect();
        match &input {
            Input::Search => {
                viewer.search = StatefulList::with_items(init.clone());
                viewer.cache = StatefulList::with_items(init);
            }
            Input::Filter => viewer.filter = StatefulList::with_items(init),
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
