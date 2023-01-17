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

    pub input: String, 
    pub errormsg: Option<String>,
    pub history: Vec<String>,

    pub graph: Graph,
    pub all: StatefulList<String>,
    pub prevs: StatefulList<String>,
    pub nexts: StatefulList<String>,
}

impl App {
    pub fn new(path: &str) -> App {
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();  
                
        let mut app = App {
            quit: false,
            mode: Mode::Normal,
            input: String::from(""),
            history: Vec::new(),
            errormsg: None,
            graph,
            all: StatefulList::with_items(nodes),
            prevs: StatefulList::with_items(Vec::new()),
            nexts: StatefulList::with_items(Vec::new()),
        };

        let id = app.all.selected().unwrap();
        app.prevs(id.as_str());
        app.nexts(id.as_str());

        app
    }

    pub fn prevs(&mut self, id: &str) {
        let prevs = self.graph.froms(&id).iter().map(|n| n.to_string()).collect();
        self.prevs = StatefulList::with_items(prevs);
    }

    pub fn nexts(&mut self, id: &str) {
        let nexts = self.graph.tos(&id).iter().map(|n| n.to_string()).collect();
        self.nexts = StatefulList::with_items(nexts);
    }
}
