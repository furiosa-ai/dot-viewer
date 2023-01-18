use crate::app::utils::{
    list::StatefulList,
    trie::SearchTrie,
};
use dot_graph::{
    parser::parse,
    structs::Graph,
};

// TODO merge Mode and Focus into one enum?
#[derive(Debug, Clone)]
pub enum Mode {
    Traverse,
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
    pub tab: usize,

    pub input: String, 
    pub errormsg: Option<String>,
    pub history: Vec<String>,

    pub ctxts: Vec<Ctxt>,
}

pub struct Ctxt {
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
        App {
            quit: false,
            mode: Mode::Traverse,
            tab: 0,
            input: String::from(""),
            history: Vec::new(),
            errormsg: None,
            ctxts: vec![Ctxt::new(path)],
        }
    }
}

impl Ctxt {
    pub fn new(path: &str) -> Ctxt {
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();  
        let trie = SearchTrie::new(&nodes);

        let mut ctxt = Ctxt {
            title: String::from("DAG"),
            graph,
            trie,
            focus: Focus::Current,
            current: StatefulList::with_items(nodes),
            prevs: StatefulList::with_items(Vec::new()),
            nexts: StatefulList::with_items(Vec::new()),
            search: StatefulList::with_items(Vec::new()),
        };

        ctxt.update_adjacent();

        ctxt
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
