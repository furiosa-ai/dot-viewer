use crate::app::utils::{
    list::StatefulList,
    tabs::StatefulTabs,
};
use dot_graph::{
    parser::parse,
    structs::Graph,
};

#[derive(Debug, Clone)]
pub enum Mode {
    Navigate,
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    Current,
    Prevs,
    Nexts,
    Search,
    Filter,
}

pub struct App {
    pub quit: bool,
    pub mode: Mode,

    pub tabs: StatefulTabs<Viewer>,

    pub input: String, 
    pub errormsg: Option<String>,
    pub history: Vec<String>,
}

pub struct Viewer {
    pub title: String,

    pub graph: Graph,

    pub focus: Focus,

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
            mode: Mode::Navigate,
            tabs,
            input: String::from(""),
            history: Vec::new(),
            errormsg: None,
        }
    }

    pub fn to_nav_mode(&mut self) {
        self.mode = Mode::Navigate;
        self.input = "".to_string();

        let viewer = self.tabs.selected();
        viewer.focus = Focus::Current;
    }

    pub fn to_input_mode(&mut self, focus: Focus) {
        self.mode = Mode::Input;

        let viewer = self.tabs.selected();
        let init: Vec<(String, Vec<usize>)> = viewer.current.items.iter().map(|id| (id.clone(), Vec::new())).collect();
        viewer.focus = focus.clone();
        match focus {
            Focus::Search => {
                viewer.search = StatefulList::with_items(init.clone());
                viewer.cache = StatefulList::with_items(init.clone());
            },
            Focus::Filter => viewer.filter = StatefulList::with_items(init.clone()),
            _ => {}
        }
    }
}

impl Viewer {
    pub fn new(title: String, graph: Graph) -> Viewer {
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();  

        let mut viewer = Viewer {
            title,
            graph,
            focus: Focus::Current,
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

    pub fn idx(&self) -> Option<usize> {
        self.current.state.selected()
    }

    pub fn count(&self) -> usize {
        self.current.items.len()
    } 
}
