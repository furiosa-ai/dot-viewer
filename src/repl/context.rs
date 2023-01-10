use crate::graph::graph::{ Graph, CenterGraph, Node };

#[derive(Clone)]
pub struct Context {
    pub filename: String,
    pub graph: Graph,
    pub centergraph: CenterGraph,
}

impl Context {
    pub fn to_string(&self) -> String {
        format!("center : {}\ndepth: {}\n", &self.centergraph.center.id, self.centergraph.depth_limit)
    }

    pub fn center(&self, center: &Node) -> Context {
        Context { 
            filename: self.filename.clone(), 
            graph: self.graph.clone(), 
            centergraph: self.graph.centergraph(center, self.centergraph.depth_limit)
        }
    }

    pub fn depth_limit(&self, depth_limit: u8) -> Context {
        Context { 
            filename: self.filename.clone(), 
            graph: self.graph.clone(), 
            centergraph: self.graph.centergraph(&self.centergraph.center, depth_limit)
        }
    }

}
