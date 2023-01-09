use crate::graph::{ Graph, Node };

// TODO hold CenterGraph in Context to prevent redundant computation on each repl
pub struct Context {
    pub filename: String,
    pub graph: Graph,
    pub center: Node,
    pub depth_limit: u8,
}

impl Context {
    pub fn to_string(&self) -> String {
        format!("center : {}\ndepth: {}\n", &self.center.id, self.depth_limit)
    }
}
