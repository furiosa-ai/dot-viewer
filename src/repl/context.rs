use crate::graph::graph::{ Graph, Node };

// TODO hold CenterGraph in Context to prevent redundant computation on each repl
// TODO center and depth_limit can be replaced with centergraph
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
