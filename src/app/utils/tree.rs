use dot_graph::Graph;
use rayon::prelude::*;
use std::boxed::Box;
use tui_tree_widget::{TreeItem, TreeState};

struct Node {
    id: String,
    children: Vec<Box<Node>>,
}

pub struct Tree {
    pub state: TreeState,
    items: Vec<Box<Node>>,
    pub tree: Vec<TreeItem<'static>>,
}

impl Tree {
    #[allow(dead_code)]
    pub fn with_graph(graph: &Graph) -> Self {
        let &root = graph.slookup.get_by_left(&graph.id).unwrap();

        let tree = Self::to_tree(root, graph);
        let tree = vec![tree];

        let items = Self::to_items(root, graph);
        let items = vec![items];

        let mut tree = Self {
            state: TreeState::default(),
            items,
            tree,
        };

        if !tree.tree.is_empty() {
            tree.state.select_first();
        }

        tree
    }

    fn to_tree(root: usize, graph: &Graph) -> TreeItem<'static> {
        let id = graph.slookup.get_by_right(&root).unwrap().to_string();

        if let Some(subgraphs) = graph.subtree.get(&root) {
            let subgraphs: Vec<TreeItem> = subgraphs
                .par_iter()
                .map(|&subgraph| Self::to_tree(subgraph, graph))
                .collect();

            TreeItem::new(id, subgraphs)
        } else {
            TreeItem::new_leaf(id)
        }
    }

    fn to_items(root: usize, graph: &Graph) -> Box<Node> {
        let id = graph.slookup.get_by_right(&root).unwrap().to_string();

        let node = if let Some(subgraphs) = graph.subtree.get(&root) {
            let children: Vec<Box<Node>> = subgraphs
                .par_iter()
                .map(|&subgraph| Self::to_items(subgraph, graph))
                .collect();

            Node { id, children }
        } else {
            Node {
                id,
                children: Vec::new(),
            }
        };

        Box::new(node)
    }

    pub fn selected(&self) -> String {
        let mut idxs = self.state.selected();

        let idx = idxs.remove(0);
        let mut node = &self.items[idx];
        for idx in idxs {
            node = &node.children[idx];
        }

        node.id.clone()
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
