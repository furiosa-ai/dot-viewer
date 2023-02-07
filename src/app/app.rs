use crate::app::{
    error::{DotViewerError, Res},
    modes::{InputMode, MainMode, Mode, NavMode},
    utils::{Input, List, Tabs},
    viewer::Viewer,
};
use dot_graph::parser::parse;
use std::io::Write;

pub struct App {
    pub quit: bool,
    pub mode: Mode,
    pub result: Res,

    pub tabs: Tabs<Viewer>,
    pub input: Input,
}

impl App {
    pub fn new(path: &str) -> Result<App, DotViewerError> {
        let graph = parse(path)?;
        let viewer = Viewer::new("DAG".to_string(), graph);
        let tabs = Tabs::with_tabs(vec![viewer])?;
        let input = Input::new();

        Ok(App {
            quit: false,
            mode: Mode::Main(MainMode::Navigate(NavMode::Current)),
            result: Ok(None),
            tabs,
            input,
        })
    }

    pub fn selected(&mut self) -> Option<String> {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => {
                    let viewer = self.tabs.selected();

                    match nav {
                        NavMode::Current => viewer.current.selected(),
                        NavMode::Prevs => viewer.prevs.selected(),
                        NavMode::Nexts => viewer.nexts.selected(),
                    }
                }
                MainMode::Input(_) => {
                    let viewer = self.tabs.selected();

                    viewer.matches.selected().map(|(item, _)| item)
                }
            },
            Mode::Popup => None,
        }
    }

    pub fn goto(&mut self) -> Res {
        let id = self.selected();
        id.map_or(
            Err(DotViewerError::ViewerError("no node selected".to_string())),
            |id| {
                let viewer = self.tabs.selected();
                viewer.goto(&id)
            },
        )
    }

    pub fn filter(&mut self) -> Res {
        let viewer = self.tabs.selected();
        let viewer = viewer.filter(&self.input.key())?;
        self.tabs.open(viewer);

        Ok(None)
    }

    pub fn subgraph(&mut self) -> Res {
        let viewer = self.tabs.selected();
        let viewer = viewer.subgraph()?;
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

        Self::write(filename, graph.to_dot())
            .map(Some)
            .map_err(|e| DotViewerError::IOError(e.to_string()))
    }

    pub fn xdot(&mut self) -> Res {
        if !std::path::Path::new("./exports/current.dot").exists() {
            return Err(DotViewerError::XdotError);
        }

        let xdot = std::process::Command::new("xdot")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg("./exports/current.dot")
            .spawn();

        xdot.map(|_| None).map_err(|_| DotViewerError::XdotError)
    }

    pub fn neighbors(&mut self, depth: usize) -> Res {
        let viewer = self.tabs.selected();
        let graph = &viewer.graph;
        let node = &viewer.current().unwrap();

        let filename = format!("{}-{}", node.clone(), depth);

        graph.neighbors(node, depth).map_or_else(
            |e| Err(DotViewerError::ViewerError(e.to_string())),
            |neighbors| match neighbors {
                Some(neighbors) => {
                    let contents = neighbors.to_dot();
                    Self::write(filename, contents)
                        .map(Some)
                        .map_err(|e| DotViewerError::IOError(e.to_string()))
                }
                None => Err(DotViewerError::ViewerError("empty graph".to_string())),
            },
        )
    }

    pub fn to_nav_mode(&mut self) {
        self.mode = Mode::Main(MainMode::Navigate(NavMode::Current));
        self.input.clear();
    }

    pub fn to_input_mode(&mut self, mode: InputMode) {
        self.mode = Mode::Main(MainMode::Input(mode));

        let viewer = self.tabs.selected();

        let init: Vec<(String, Vec<usize>)> = viewer
            .current
            .items
            .iter()
            .map(|id| (id.clone(), Vec::new()))
            .collect();
        viewer.matches = List::with_items(init);
    }

    pub fn to_popup_mode(&mut self) {
        self.mode = Mode::Popup;
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
