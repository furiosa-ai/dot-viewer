use crate::app::utils::{
    list::StatefulList,
    trie::SearchTrie,
};
use dot_graph::{
    parser::parse,
    structs::Graph,
};
use crate::app::utils::tabs::StatefulTabs;

// TODO merge Mode and Focus into one enum?
#[derive(Debug, Clone)]
pub enum Mode {
    Navigate,
    Search,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    Current,
    Prevs,
    Nexts,
    Search,
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
    pub trie: SearchTrie,

    pub focus: Focus,
    pub current: StatefulList<String>,
    pub prevs: StatefulList<String>,
    pub nexts: StatefulList<String>,
    pub search: StatefulList<String>,
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

        let viewer = &mut self.tabs.selected();
        viewer.focus = Focus::Current;
    }

    pub fn to_search_mode(&mut self) {
        self.mode = Mode::Search;

        let viewer = &mut self.tabs.selected();
        viewer.focus = Focus::Search;
    }
}

impl Viewer {
    pub fn new(title: String, graph: Graph) -> Viewer {
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();  
        let trie = SearchTrie::new(&nodes);

        let mut viewer = Viewer {
            title,
            graph,
            trie,
            focus: Focus::Current,
            current: StatefulList::with_items(nodes),
            prevs: StatefulList::with_items(Vec::new()),
            nexts: StatefulList::with_items(Vec::new()),
            search: StatefulList::with_items(Vec::new()),
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
