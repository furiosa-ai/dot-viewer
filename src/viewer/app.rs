use crate::viewer::{
    error::{DotViewerError, DotViewerResult},
    help::Help,
    modes::{InputMode, MainMode, Mode, NavMode, PopupMode},
    success::SuccessState,
    utils::{Input, List, Tabs},
    view::View,
};

use dot_graph::{parser, Graph};

/// `App` holds `dot-viewer` application states.
///
/// `tui-rs` simply redraws the entire screen in a loop while accepting keyboard inputs.
/// Thus `App` should keep track of the application context in its fields.
pub(crate) struct App {
    /// Whether to quit the application or not, by `q` keybinding
    pub quit: bool,

    /// Current mode the application is in
    pub mode: Mode,

    /// Result of the last command that was made
    pub result: DotViewerResult<SuccessState>,

    /// Tabs to be shown in the main screen
    pub tabs: Tabs<View>,

    /// Input form to be shown in the main screen
    pub input: Input,

    /// Keybinding helps
    pub help: Help,
}

impl App {
    /// Constructs a new `App`, given a `path` to a dot format DAG.
    pub(crate) fn new(path: &str) -> DotViewerResult<App> {
        let quit = false;

        let mode = Mode::Main(MainMode::Navigate(NavMode::Current));

        let result: DotViewerResult<SuccessState> = Ok(SuccessState::default());

        let graph = parser::parse(path)?;
        let view = View::new(graph.id().clone(), graph);
        let tabs = Tabs::with_tabs(vec![view])?;

        let input = Input::default();

        //let help = HELP.iter().map(|row| row.iter().map(|s| s.to_string()).collect()).collect();
        let help = Help::new();

        Ok(App { quit, mode, result, tabs, input, help })
    }

    /// Navigate to the currently selected node.
    /// The current node list will be focused on the selected node.
    pub(crate) fn goto(&mut self) -> DotViewerResult<()> {
        let id = self.selected_id();

        id.map_or(Err(DotViewerError::ViewerError("no node selected".to_string())), |id| {
            let view = self.tabs.selected();
            view.goto(&id)
        })
    }

    /// Apply prefix filter on the current view.
    /// Based on the currently typed input, it applies a prefix filter on the current view,
    /// and opens a new tab with the filtered view.
    pub(crate) fn filter(&mut self) -> DotViewerResult<()> {
        let view_current = self.tabs.selected();
        let view_new = view_current.filter(&self.input.key())?;
        self.tabs.open(view_new);

        Ok(())
    }

    /// Extract a subgraph from the current view.
    /// When a subgraph id is selected in the subgraph tree,
    /// it opens a new tab containing only the selected subgraph.
    pub(crate) fn subgraph(&mut self) -> DotViewerResult<()> {
        let view_current = self.tabs.selected();
        let view_new = view_current.subgraph()?;
        self.tabs.open(view_new);

        Ok(())
    }

    /// Export a neigbor graph from the currently selected node to dot,
    /// given the neighbor depth by `0-9` keybindings.
    pub(crate) fn neighbors(&mut self, depth: usize) -> DotViewerResult<SuccessState> {
        let view = self.tabs.selected();
        let graph = &view.graph;
        let node = &view.current_id().expect("current node is always selected");

        let filename = format!("{node}-{depth}");

        graph.neighbors(node, depth).map_or_else(
            |e| Err(DotViewerError::ViewerError(e.to_string())),
            |neighbor_graph| {
                if neighbor_graph.is_empty() {
                    return Err(DotViewerError::ViewerError("empty graph".to_string()));
                }

                write_graph(filename, &neighbor_graph)
            },
        )
    }

    /// Export the current view to dot.
    pub(crate) fn export(&mut self) -> DotViewerResult<SuccessState> {
        let viewer = self.tabs.selected();
        let graph = &viewer.graph;

        let filename: String = viewer.title.chars().filter(|c| !c.is_whitespace()).collect();

        write_graph(filename, graph)
    }

    /// Launch `xdot.py`, coming from `x` keybinding.
    pub(crate) fn xdot(&mut self) -> DotViewerResult<SuccessState> {
        if !std::path::Path::new("./exports/current.dot").exists() {
            return Err(DotViewerError::XdotError);
        }

        let xdot = std::process::Command::new("xdot")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg("./exports/current.dot")
            .spawn();

        xdot.map(|_| SuccessState::XdotSuccess).map_err(|_| DotViewerError::XdotError)
    }

    pub(crate) fn set_nav_mode(&mut self) {
        self.mode = Mode::Main(MainMode::Navigate(NavMode::Current));
        self.input.clear();
    }

    pub(crate) fn set_input_mode(&mut self, imode: InputMode) {
        self.mode = Mode::Main(MainMode::Input(imode));

        let view = self.tabs.selected();

        let init: Vec<(String, Vec<usize>)> =
            view.current.items.iter().map(|id| (id.clone(), Vec::new())).collect();
        view.matches = List::with_items(init);
    }

    pub(crate) fn set_popup_mode(&mut self, pmode: PopupMode) {
        self.mode = Mode::Popup(pmode);
    }

    pub(crate) fn selected_id(&mut self) -> Option<String> {
        let viewer = self.tabs.selected();

        match &self.mode {
            Mode::Main(mmode) => match mmode {
                MainMode::Navigate(nmode) => match nmode {
                    NavMode::Current => viewer.current.selected(),
                    NavMode::Prevs => viewer.prevs.selected(),
                    NavMode::Nexts => viewer.nexts.selected(),
                },
                MainMode::Input(_) => viewer.matches.selected().map(|(id, _)| id),
            },
            Mode::Popup(_) => None,
        }
    }
}

fn write_graph(filename: String, graph: &Graph) -> DotViewerResult<SuccessState> {
    std::fs::create_dir_all("./exports")?;
    let mut file_export = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("./exports/{filename}.dot"))?;
    graph.to_dot(&mut file_export)?;

    let mut file_current = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./exports/current.dot")?;
    graph.to_dot(&mut file_current)?;

    Ok(SuccessState::ExportSuccess(filename))
}
