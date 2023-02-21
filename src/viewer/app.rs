use crate::viewer::{
    command::{Command, CommandTrie},
    error::{DotViewerError, DotViewerResult},
    help,
    modes::{Mode, PopupMode, SearchMode},
    success::Success,
    utils::{Input, List, Table, Tabs},
    view::View,
};

use dot_graph::{parser, Graph};

use crossterm::event::KeyCode;

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
    pub result: DotViewerResult<Success>,

    /// Tabs to be shown in the main screen
    pub tabs: Tabs<View>,

    /// Input form to be shown in the main screen
    pub input: Input,

    /// Most recent key event
    pub lookback: Option<KeyCode>,

    /// Autocomplete support for commands
    pub trie: CommandTrie,

    /// Keybinding helps
    pub help: Table,
}

impl App {
    /// Constructs a new `App`, given a `path` to a dot format DAG.
    pub fn new(path: &str) -> DotViewerResult<App> {
        let quit = false;

        let mode = Mode::Normal;

        let result: DotViewerResult<Success> = Ok(Success::default());

        let graph = parser::parse(path)?;

        let view = View::new(graph.id().clone(), graph)?;
        let tabs = Tabs::from_iter(vec![view]);

        let input = Input::default();

        let lookback = None;

        let trie = CommandTrie::new();

        let help = Table::new(help::HEADER, help::ROWS);

        Ok(App { quit, mode, result, tabs, input, lookback, trie, help })
    }

    /// Navigate to the next match.
    pub fn goto_next_match(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();
        view.matches.next();
        view.goto_match()
    }

    /// Navigate to the previous match.
    pub fn goto_prev_match(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();
        view.matches.previous();
        view.goto_match()
    }

    /// Navigate to the first.
    pub fn goto_first(&mut self) -> DotViewerResult<()> {
        if let Some(KeyCode::Char('g')) = self.lookback {
            let view = self.tabs.selected();
            view.goto_first()?;
        }

        Ok(())
    }

    /// Navigate to the last.
    pub fn goto_last(&mut self) -> DotViewerResult<()> {
        let view = self.tabs.selected();
        view.goto_last()
    }

    /// Update search matches with trie.
    pub fn update_search(&mut self) {
        match &self.mode {
            Mode::Search(smode) => {
                let view = self.tabs.selected();
                let key = &self.input.key;

                match smode {
                    SearchMode::Fuzzy => view.update_fuzzy(key),
                    SearchMode::Regex => view.update_regex(key),
                }
                view.update_trie();

                // ignore goto errors while updating search matches
                let _ = view.goto_match();
            }
            _ => unreachable!(),
        }
    }

    /// Autocomplete user input.
    pub fn autocomplete_fuzzy(&mut self) {
        let view = self.tabs.selected();

        let key = &self.input.key;
        if let Some(key) = view.autocomplete(key) {
            view.update_fuzzy(&key);
            view.update_trie();
            self.input.set(key);
        }
    }

    /// Autocomplete user input.
    pub fn autocomplete_regex(&mut self) {
        let view = self.tabs.selected();

        let key = &self.input.key;
        if let Some(key) = view.autocomplete(key) {
            view.update_regex(&key);
            view.update_trie();
            self.input.set(key);
        }
    }

    /// Autocomplete user input.
    pub fn autocomplete_command(&mut self) {
        let command = Command::parse(&self.input.key);

        if command == Command::NoMatch {
            self.autocomplete_cmd()
        }
    }

    fn autocomplete_cmd(&mut self) {
        let cmd = &self.input.key;
        if let Some(cmd) = self.trie.trie_cmd.autocomplete(cmd) {
            self.input.set(cmd);
        }
    }

    /// Parse and execute dot-viewer command
    pub fn exec(&mut self) -> DotViewerResult<Success> {
        let command = Command::parse(&self.input.key);

        match command {
            Command::Neighbors(neighbors) => neighbors.depth.map_or(
                Err(DotViewerError::CommandError("No argument supplied for neighbors".to_string())),
                |depth| self.neighbors(depth).map(|_| Success::default()),
            ),
            Command::Export(export) => self.export(export.filename),
            Command::Xdot(xdot) => self.xdot(xdot.filename),
            Command::Filter => self.filter().map(|_| Success::default()),
            Command::Help => {
                self.set_popup_mode(PopupMode::Help);
                Ok(Success::default())
            }
            Command::Subgraph => {
                self.set_popup_mode(PopupMode::Tree);
                Ok(Success::default())
            }
            Command::NoMatch => {
                self.set_normal_mode();

                let key = &self.input.key;
                Err(DotViewerError::CommandError(format!("No such command {key}")))
            }
        }
    }

    /// Extract a subgraph which is a neighbor graph from the currently selected node,
    /// with specified depth.
    /// It opens a new tab with the neighbor graph view.
    pub fn neighbors(&mut self, depth: usize) -> DotViewerResult<()> {
        let view_current = self.tabs.selected();
        let view_new = view_current.neighbors(depth)?;
        self.tabs.open(view_new);

        self.set_normal_mode();

        Ok(())
    }

    /// Export the current view to dot.
    pub fn export(&mut self, filename: Option<String>) -> DotViewerResult<Success> {
        let viewer = self.tabs.selected();
        let graph = &viewer.graph;

        let default: String = viewer.title.chars().filter(|c| !c.is_whitespace()).collect();
        let filename = filename.unwrap_or(format!("{default}.dot"));

        write_graph(filename, graph)
    }

    /// Launch `xdot.py`.
    pub fn xdot(&mut self, filename: Option<String>) -> DotViewerResult<Success> {
        let filename = filename.unwrap_or_else(|| "current.dot".to_string());
        let path = format!("./exports/{filename}");

        if !std::path::Path::new("./exports/current.dot").exists() {
            return Err(DotViewerError::XdotError);
        }

        let xdot = std::process::Command::new("xdot")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg(&path)
            .spawn();

        xdot.map(|_| Success::XdotSuccess).map_err(|_| DotViewerError::XdotError)
    }

    /// Apply filter on the current view, based on the current matches.
    /// Opens a new tab with the filtered view.
    pub fn filter(&mut self) -> DotViewerResult<()> {
        let view_current = self.tabs.selected();
        let view_new = view_current.filter()?;
        self.tabs.open(view_new);

        self.set_normal_mode();

        Ok(())
    }

    /// Extract a subgraph from the current view.
    /// When a subgraph id is selected in the subgraph tree,
    /// it opens a new tab containing only the selected subgraph.
    pub fn subgraph(&mut self) -> DotViewerResult<()> {
        let view_current = self.tabs.selected();
        let view_new = view_current.subgraph()?;
        self.tabs.open(view_new);

        self.set_normal_mode();

        Ok(())
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn set_command_mode(&mut self) {
        self.input.clear();

        self.mode = Mode::Command;
    }

    pub fn set_search_mode(&mut self, smode: SearchMode) {
        self.input.clear();

        self.mode = Mode::Search(smode);

        let view = self.tabs.selected();

        view.matches = List::from_iter(Vec::new());
        view.prevs = List::from_iter(Vec::new());
        view.nexts = List::from_iter(Vec::new());
    }

    pub fn set_popup_mode(&mut self, pmode: PopupMode) {
        self.mode = Mode::Popup(pmode);
    }
}

fn valid_filename(filename: &str) -> bool {
    (!filename.contains('/')) && filename.ends_with(".dot")
}

fn write_graph(filename: String, graph: &Graph) -> DotViewerResult<Success> {
    if !valid_filename(&filename) {
        return Err(DotViewerError::CommandError(format!("invalid dot filename: {filename}")));
    }

    std::fs::create_dir_all("./exports")?;
    let mut file_export = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("./exports/{filename}"))?;
    graph.to_dot(&mut file_export)?;

    let mut file_current = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./exports/current.dot")?;
    graph.to_dot(&mut file_current)?;

    Ok(Success::ExportSuccess(filename))
}
