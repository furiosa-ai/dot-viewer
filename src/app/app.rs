use crate::app::{
    error::{DotViewerError, Res},
    modes::{InputMode, MainMode, Mode, NavMode},
    utils::{Input, List, Tabs},
    view::View,
};
use dot_graph::{parser, Graph};

pub struct App {
    pub quit: bool,
    pub mode: Mode,
    pub result: Res,

    pub tabs: Tabs<View>,
    pub input: Input,
}

impl App {
    pub fn new(path: &str) -> Result<App, DotViewerError> {
        let graph = parser::parse(path)?;
        let view = View::new(graph.id.clone(), graph);
        let tabs = Tabs::with_tabs(vec![view])?;
        let input = Input::new();

        Ok(App {
            quit: false,
            mode: Mode::Main(MainMode::Navigate(NavMode::Current)),
            result: Ok(None),
            tabs,
            input,
        })
    }

    pub fn selected_id(&mut self) -> Option<String> {
        match &self.mode {
            Mode::Main(main) => match main {
                MainMode::Navigate(nav) => {
                    let view = self.tabs.selected();

                    match nav {
                        NavMode::Current => view.current.selected(),
                        NavMode::Prevs => view.prevs.selected(),
                        NavMode::Nexts => view.nexts.selected(),
                    }
                }
                MainMode::Input(_) => {
                    let view = self.tabs.selected();

                    view.matches.selected().map(|(id, _)| id)
                }
            },
            Mode::Popup => None,
        }
    }

    pub fn goto(&mut self) -> Res {
        let id = self.selected_id();

        id.map_or(Err(DotViewerError::ViewerError("no node selected".to_string())), |id| {
            let view = self.tabs.selected();
            view.goto(&id)
        })
    }

    pub fn filter(&mut self) -> Res {
        let view_current = self.tabs.selected();
        let view_new = view_current.filter(&self.input.key())?;
        self.tabs.open(view_new);

        Ok(None)
    }

    pub fn subgraph(&mut self) -> Res {
        let view_current = self.tabs.selected();
        let view_new = view_current.subgraph()?;
        self.tabs.open(view_new);

        Ok(None)
    }

    pub fn export(&mut self) -> Res {
        let view = self.tabs.selected();
        let graph = &view.graph;

        let filename: String = view.title.chars().filter(|c| !c.is_whitespace()).collect();

        write(filename, graph).map(Some).map_err(|e| DotViewerError::IOError(e.to_string()))
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
        let view = self.tabs.selected();
        let graph = &view.graph;
        let node = &view.current_id().unwrap();

        let filename = format!("{}-{}", node.clone(), depth);

        graph.neighbors(node, depth).map_or_else(
            |e| Err(DotViewerError::ViewerError(e.to_string())),
            |neighbor_graph| {
                neighbor_graph.map_or(
                    Err(DotViewerError::ViewerError("empty graph".to_string())),
                    |neighbor_graph| {
                        write(filename, &neighbor_graph).map_or_else(
                            |e| Err(DotViewerError::IOError(e.to_string())),
                            |res| Ok(Some(res)),
                        )
                    },
                )
            },
        )
    }

    pub fn to_nav_mode(&mut self) {
        self.mode = Mode::Main(MainMode::Navigate(NavMode::Current));
        self.input.clear();
    }

    pub fn to_input_mode(&mut self, imode: InputMode) {
        self.mode = Mode::Main(MainMode::Input(imode));

        let view = self.tabs.selected();

        let init: Vec<(String, Vec<usize>)> =
            view.current.items.iter().map(|id| (id.clone(), Vec::new())).collect();
        view.matches = List::with_items(init);
    }

    pub fn to_popup_mode(&mut self) {
        self.mode = Mode::Popup;
    }
}

fn write(filename: String, graph: &Graph) -> Result<String, std::io::Error> {
    std::fs::create_dir_all("./exports")?;
    let mut file_export = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("./exports/{}.dot", filename))?;
    graph.to_dot(&mut file_export)?;

    let mut file_current = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./exports/current.dot")?;
    graph.to_dot(&mut file_current)?;

    Ok(format!("file successfully written to {}", filename))
}
