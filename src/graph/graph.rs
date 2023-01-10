use std::collections::{ BTreeMap, BTreeSet };

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: BTreeSet<Node>,
    pub forward: EdgeMap, // from => [tos]
    pub backward: EdgeMap, // to => [froms]
}

impl Graph {
    pub fn new(nodes: BTreeSet<Node>, edges: Vec<Edge>) -> Graph {
        Graph { nodes, forward: EdgeMap::forward(&edges), backward: EdgeMap::backward(&edges) }
    }

    pub fn merge(center: &Node, prevs: &Graph, nexts: &Graph) -> Graph {
        let mut nodes: BTreeSet<Node> = prevs.clone().nodes.union(&nexts.nodes).cloned().collect();
        nodes.insert(center.clone());

        Graph {
            nodes,
            forward: EdgeMap::merge(&prevs.backward, &nexts.forward),
            backward: EdgeMap::merge(&prevs.forward, &nexts.backward)
        }
    }

    pub fn contains(&self, node: &str) -> bool {
        self.nodes.contains(&Node::new(node))
    }

    // Build CenterGraph out of current graph provided center and depth limit
    pub fn centergraph(&self, center: &Node, depth_limit: u8) -> CenterGraph {
        let prevs = self.backward.search(center, depth_limit);
        let nexts = self.forward.search(center, depth_limit);

        CenterGraph::merge(prevs, nexts)
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("");
        dot.push_str("digraph G {\n");

        for node in &self.nodes {
            dot.push_str("\t");
            dot.push_str(&node.to_dot());
            dot.push_str("\n");
        }

        dot.push_str(&self.forward.to_dot());

        dot.push_str("}");

        dot
    }
} 

#[derive(Debug, Clone)]
pub struct CenterGraph {
    pub graph: Graph,
    pub center: Node,
    pub vicinity: Vec<(Node, i8)>, // closeness of a node with respect to the center node
    pub fwd: BTreeMap<u8, Node>, 
    pub bwd: BTreeMap<Node, u8>, // bindings for faster goto
    pub depth_limit: u8,
}

impl CenterGraph {
    pub fn new(graph: Graph, center: Node, mut vicinity: Vec<(Node, i8)>, depth_limit: u8) -> CenterGraph {
        vicinity.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

        let mut fwd: BTreeMap<u8, Node> = BTreeMap::new();
        let mut bwd: BTreeMap<Node, u8> = BTreeMap::new();
        for (idx, (node, _)) in vicinity.iter().enumerate() {
            fwd.insert(idx as u8, node.clone());
            bwd.insert(node.clone(), idx as u8);
        }

        CenterGraph { graph, center, vicinity, fwd, bwd, depth_limit }
    }

    pub fn merge(prevs: CenterGraph, nexts: CenterGraph) -> CenterGraph {
        if prevs.center != nexts.center || prevs.depth_limit != nexts.depth_limit {
            panic!("two graphs do not share center");
        }

        let graph = Graph::merge(&prevs.center, &prevs.graph, &nexts.graph);

        let vicinity = [
            prevs.vicinity
                .iter()
                .map(|(node, vicinity)| (node.clone(), -1 * *vicinity))
                .collect(),
            nexts.vicinity
        ].concat();

        CenterGraph::new(graph, prevs.center, vicinity, prevs.depth_limit)
    }

    pub fn to_console(&self) -> String {
        let mut console = String::from("");

        let prevs: Vec<(Node, i8)> = self.vicinity
            .iter()
            .filter(|(_, depth)| *depth < 0)
            .cloned()
            .collect();
        let mut depth_track: i8 = -100;
        for (node, depth) in prevs {
            if depth != depth_track {
                console.push_str(&format!("\ndepth {}\n", depth));
                depth_track = depth;
            }
            console.push_str("-------------------------------\n");
            console.push_str(&format!("| [{}] {: ^25} |\n", self.bwd.get(&node).unwrap(), node.id));
            console.push_str("--------------------------------\n");
        }

        console.push_str("\n/\\ prevs /\\\n\n");
        console.push_str("--------------------------------\n");
        console.push_str(&format!("| {: ^25} |\n", &self.center.id));
        console.push_str("--------------------------------\n");
        console.push_str("\n\\/ nexts \\/\n\n");

        let nexts: Vec<(Node, i8)> = self.vicinity
            .iter()
            .filter(|(_, depth)| *depth > 0)
            .cloned()
            .collect();
        for (node, depth) in nexts {
            if depth != depth_track {
                console.push_str(&format!("\ndepth {}\n", depth));
                depth_track = depth;
            }
            console.push_str("--------------------------------\n");
            console.push_str(&format!("| [{}] {: ^25} |\n", self.bwd.get(&node).unwrap(), node.id));
            console.push_str("--------------------------------\n");
        }

        console
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

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("");

        //dot.push_str("\"");
        dot.push_str(&self.id);
        //dot.push_str("\"");
        dot.push_str(" [shape=box]");
        
        dot 
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

#[derive(Debug, Clone)]
pub struct EdgeMap {
    pub direction: BTreeMap<Node, Vec<Node>>,
}

impl EdgeMap {
    pub fn forward(edges: &Vec<Edge>) -> EdgeMap {
        let mut direction: BTreeMap<Node, Vec<Node>> = BTreeMap::new();

        for edge in edges {
            // forward construction
            match direction.get_mut(&edge.from) {
                Some(tos) => tos.push(edge.to.clone()),
                None => {
                    direction.insert(edge.from.clone(), vec![edge.to.clone()]);
                    ()
                }
            };
        }
        
        EdgeMap { direction }
    }

    pub fn backward(edges: &Vec<Edge>) -> EdgeMap {
        EdgeMap::forward(
            &edges.clone()
                .iter()
                .map(|edge| Edge::new(edge.to.clone(), edge.from.clone()))
                .collect()
        )
    }

    pub fn merge(left: &EdgeMap, right: &EdgeMap) -> EdgeMap {
        let mut merged = left.direction.clone();

        for (node_right, nexts_right) in &right.direction {
            match merged.get_mut(&node_right) {
                Some(nexts) => nexts.append(&mut nexts_right.clone()),
                None => {
                    merged.insert(node_right.clone(), nexts_right.clone());
                    ()
                }
            }
        }

        EdgeMap { direction: merged }
    }

    pub fn search(&self, start: &Node, depth_limit: u8) -> CenterGraph {
        let mut nodes: BTreeSet<Node> = BTreeSet::new();
        let mut vicinity: Vec<(Node, i8)> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        let mut frontier: BTreeSet<(&Node, u8)> = BTreeSet::new();
        frontier.insert((start, 0));

        while !frontier.is_empty() {
            let (node, depth) = frontier.pop_first().unwrap();

            if depth > depth_limit {
                continue;
            }
            if nodes.contains(node) {
                continue;
            }

            let nexts = match self.direction.get(node) {
                Some(nodes) => nodes.iter().map(|node| (node, depth + 1)).collect(),
                None => Vec::new()
            };
            for next in nexts.clone() {
                frontier.insert(next);
            }

            if node != start {
                nodes.insert(node.clone());
                vicinity.push((node.clone(), depth as i8));
            }
            if depth < depth_limit {
                for (next, _) in nexts {
                    edges.push(Edge::new(node.clone(), next.clone()));
                }
            }
        }

        CenterGraph::new(
            Graph::new(nodes, edges),
            start.clone(),
            vicinity,
            depth_limit
        )
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("");

        for (from, tos) in &self.direction {
            for to in tos {
                dot.push_str("\t");
                dot.push_str(&from.id);
                dot.push_str(" -> ");
                dot.push_str(&to.id);
                dot.push_str("\n");
            }
        }

        dot
    }
}
