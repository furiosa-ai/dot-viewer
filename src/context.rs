use crate::graph::{ Graph, Node };

pub struct Context<'a> {
    pub graph: &'a Graph,
    pub center: &'a Node,
    pub depth: u8,
}

impl<'a> Context<'a> {
    pub fn to_string(&self) -> String {
        format!("center : {}\ndepth: {}\n", &self.center.id, self.depth)
    }
}
