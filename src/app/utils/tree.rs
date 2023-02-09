use dot_graph::{Graph, SubGraph};
use rayon::prelude::*;
use tui_tree_widget::{TreeItem, TreeState};

struct Node {
    id: String,
    children: Vec<Node>,
}

// https://github.com/EdJoPaTo/tui-rs-tree-widget/blob/main/examples/util/mod.rs
pub struct Tree {
    pub state: TreeState, 
    pub tree: Vec<TreeItem<'static>>,
    items: Vec<Node>,
}

impl Tree {
    pub fn with_graph(graph: &Graph) -> Self {
        let root = graph.search_subgraph(&graph.id).unwrap();

        let tree = to_tree(root, graph);
        let tree = vec![tree];

        let item = to_item(root, graph);
        let items = vec![item];

        let mut tree = Self { state: TreeState::default(), items, tree };

        if !tree.tree.is_empty() {
            tree.first();
            tree.toggle();
        }

        tree
    }

    pub fn selected(&self) -> Option<String> {
        let mut idxs = self.state.selected();

        if idxs.is_empty() {
            return None;
        }

        let idx = idxs.remove(0);
        let mut node = &self.items[idx];
        for idx in idxs {
            node = &node.children[idx];
        }

        Some(node.id.clone())
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.tree);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.tree);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.tree);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}

fn to_tree(root: &SubGraph, graph: &Graph) -> TreeItem<'static> {
    let id = root.id.clone();

    let children = graph.children(&root.id).expect("root should exist in the graph");
    let children: Vec<TreeItem> = children.par_iter().map(|&child| to_tree(child, graph)).collect();

    TreeItem::new(id, children)
}

fn to_item(root: &SubGraph, graph: &Graph) -> Node {
    let id = root.id.clone();

    let children = graph.children(&root.id).expect("root should exist in the graph");
    let children: Vec<Node> = children.par_iter().map(|&child| to_item(child, graph)).collect();

    Node { id, children }
}
