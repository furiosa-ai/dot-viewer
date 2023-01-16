use crate::app::utils::list::StatefulList;
use dot_graph::{
    parser::parse,
    structs::Graph,
};

pub enum Mode {
    Normal,
    Command,
}

pub struct App {
    pub quit: bool,

    pub mode: Mode,
    pub command: String,
    pub history: Vec<String>,

    pub graph: Graph,
    pub nodes: StatefulList<String>,
}

impl App {
    pub fn new(path: &str) -> App{
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();

        App {
            quit: false,
            mode: Mode::Normal,
            command: String::from(""),
            history: Vec::new(),
            graph,
            nodes: StatefulList::with_items(nodes),
        }
    }
}
