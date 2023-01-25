use crate::app::{
    error::ViewerError,
    utils::{
        list::StatefulList,
        tabs::StatefulTabs,
    }
};
use dot_graph::{
    parser::parse,
    structs::Graph,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Navigate(Navigate),
    Input(Input),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Navigate {
    Current,
    Prevs,
    Nexts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Search,
    Filter,
}

pub type Res = Result<Option<String>, ViewerError>;

pub struct App {
    pub quit: bool,
    pub mode: Mode,

    pub tabs: StatefulTabs<Viewer>,

    pub input: String, 
    pub errormsg: Option<String>,
    pub history: Vec<String>,

    pub result: Res,
}

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
            errormsg: None,
            result: Ok(None),
        }
    }

    pub fn to_nav_mode(&mut self) {
        self.mode = Mode::Navigate(Navigate::Current);
        self.input = "".to_string();
    }

    pub fn to_input_mode(&mut self, input: Input) {
        self.mode = Mode::Input(input.clone());

        let viewer = self.tabs.selected();
        let init: Vec<(String, Vec<usize>)> = viewer.current.items.iter().map(|id| (id.clone(), Vec::new())).collect();
        match &input {
            Input::Search => {
                viewer.search = StatefulList::with_items(init.clone());
                viewer.cache = StatefulList::with_items(init.clone());
            },
            Input::Filter => viewer.filter = StatefulList::with_items(init.clone()),
        }
    }
}
