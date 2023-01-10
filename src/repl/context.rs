use crate::graph::graph::{ Graph, CenterGraph };

// TODO hold CenterGraph in Context to prevent redundant computation on each repl
pub struct Context {
    pub filename: String,
    pub graph: Graph,
    pub centergraph: CenterGraph,
}

impl Context {
    pub fn to_string(&self) -> String {
        format!("center : {}\ndepth: {}\n", &self.centergraph.center.id, self.centergraph.depth_limit)
    }
}
