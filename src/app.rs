use crate::utils::list::StatefulList;
use dot_graph::{
    parser::parse,
    structs::Graph,
};

pub struct App {
    pub quit: bool,

    pub graph: Graph,
    pub nodes: StatefulList<String>,
}

impl App {
    pub fn new(path: &str) -> App{
        let graph = parse(path); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();

        App {
            quit: false,
            graph,
            nodes: StatefulList::with_items(nodes),
        }
    }

    pub fn on_up(&mut self) {
        self.nodes.previous();
    }

    pub fn on_down(&mut self) {
        self.nodes.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.quit = true;
            },
            _ => {},
        }
    }

    pub fn on_tick(&mut self) {
    }
}
