use std::collections::{ BTreeMap, BTreeSet };

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        Graph { nodes, edges }
    }

    pub fn contains(&self, node: &str) -> bool {
        self.nodes.contains(&Node::new(node))
    }
} 

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    pub id: String,
}

impl Node {
    pub fn new(id: &str) -> Node {
        Node { id: String::from(id) }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: Node,
    pub to: Node
}

impl Edge {
    pub fn new(from: Node, to: Node) -> Edge {
        Edge { from, to }
    }
}

#[derive(Debug)]
pub struct EdgeMap {
    pub forward: BTreeMap<Node, Vec<Node>>,
    pub backward: BTreeMap<Node, Vec<Node>>
}

impl EdgeMap {
    pub fn new(graph: &Graph) -> EdgeMap {
        let mut forward: BTreeMap<Node, Vec<Node>> = BTreeMap::new();
        let mut backward: BTreeMap<Node, Vec<Node>> = BTreeMap::new();

        for edge in &graph.edges {
            // forward construction
            match forward.get_mut(&edge.from) {
                Some(tos) => tos.push(edge.to.clone()),
                None => {
                    forward.insert(edge.from.clone(), vec![edge.to.clone()]);
                    ()
                }
            };

            // backward construction
            match backward.get_mut(&edge.to) {
                Some(froms) => froms.push(edge.from.clone()),
                None => {
                    backward.insert(edge.to.clone(), vec![edge.from.clone()]);
                    ()
                }
            };
        }

        EdgeMap { forward, backward }
    }

    pub fn render(&self, center: &Node, depth_limit: u8) -> String {
        let forward = Self::render_map(center, &self.forward, depth_limit); 
        let backward = Self::render_map(center, &self.backward, depth_limit);
       
        // render
        let mut out = String::from("");
        for (depth, node) in backward {
            out.push_str(&format!("({}) {}\n", depth, &node.id));
        }
        out.push_str("\n/\\ prevs /\\\n\n");
        out.push_str(&format!("{}\n\n", &center.id));
        out.push_str("\\/ nexts \\/\n\n");
        for (depth, node) in forward.iter().rev() {
            out.push_str(&format!("({}) {}\n", depth, &node.id));
        }

        out
    }

    fn render_map<'a>(start: &'a Node, direction: &'a BTreeMap<Node, Vec<Node>>, depth_limit: u8) -> Vec<(u8, &'a Node)> {
        let mut nodes: BTreeSet<(u8, &Node)> = BTreeSet::new();
        let mut frontier: BTreeSet<(u8, &Node)> = BTreeSet::new();

        // initialize frontier 
        frontier.insert((0, start));

        // run search
        while !frontier.is_empty() {
            let current = frontier.pop_first().unwrap();

            if current.0 > depth_limit {
                continue;
            }

            let node_set: BTreeSet<&Node> = nodes.iter().map(|node| node.1).collect();
            if node_set.contains(current.1) {
                continue; 
            }                 

            // add to nodes
            if current.1 != start {
                nodes.insert(current);
            }

            // update frontier
            let nexts = match direction.get(current.1) {
                Some(nexts) => nexts.iter().map(|next| (current.0 + 1, next)).collect(),
                None => Vec::new()
            };
            for next in nexts {
                frontier.insert(next);
            }
        }

        let mut nodes = Vec::from_iter(nodes);
        nodes.sort_by(|&(a, _), &(b, _)| b.cmp(&a));
        
        nodes
    }
}
