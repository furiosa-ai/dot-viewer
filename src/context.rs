use crate::graph::{ Graph, Node };

pub struct Context {
    pub graph: Graph,
    pub center: Node,
    pub depth: u8,
}

impl Context {
    pub fn to_string(&self) -> String {
        format!("center : {}\ndepth: {}\n", &self.center.id, self.depth)
    }
}
