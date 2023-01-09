use std::collections::BTreeSet;
use dot_parser::{ ast, canonical };
use crate::graph::graph::{ Graph, Node, Edge };

pub fn parse(dot: &str) -> Graph {
    let ast = ast::Graph::read_dot(dot).expect("parse error");
    let canonical = canonical::Graph::from(ast);

    parse_canonical(canonical)
}

fn parse_canonical(canonical: canonical::Graph<(&str, &str)>) -> Graph {
    let nodes: Vec<Node> = Vec::from_iter(canonical.nodes.set.values())
        .iter()
        .map(|node| {
            Node::new(node.id)
        })
        .collect();

    let edges: Vec<Edge> = canonical.edges.set
        .iter()
        .map(|edge| {
            Edge::new(
                Node::new(edge.from),
                Node::new(edge.to)
            )
        })
        .collect();

    Graph::new(BTreeSet::from_iter(nodes), edges)
}
