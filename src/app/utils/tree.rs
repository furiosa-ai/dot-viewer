use dot_graph::{Graph, SubGraph};
use rayon::prelude::*;
use tui_tree_widget::{TreeItem, TreeState};

pub(super) struct Item {
    id: String,
    children: Vec<Item>,
}

// https://github.com/EdJoPaTo/tui-rs-tree-widget/blob/main/examples/util/mod.rs
pub struct Tree {
    pub state: TreeState,
    pub tree: Vec<TreeItem<'static>>,
    pub items: Vec<Item>,
}

impl Tree {
    pub fn with_graph(graph: &Graph) -> Self {
        let root = graph.search_subgraph(graph.id()).unwrap();

        let item = to_item(root, graph);
        let tree = to_tree(&item, graph);

        let items = vec![item];
        let tree = vec![tree];

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

fn to_item(root: &SubGraph, graph: &Graph) -> Item {
    let id = root.id().clone();

    let children = graph.collect_subgraphs(root.id()).expect("root should exist in the graph");
    let mut children: Vec<Item> = children.par_iter().map(|&id| {
        let child = graph.search_subgraph(id).unwrap();
        to_item(child, graph)
    }).collect();
    children.sort_by(|a, b| (a.id).cmp(&b.id));

    Item { id, children }
}

fn to_tree(root: &Item, graph: &Graph) -> TreeItem<'static> {
    let id = &root.id;
    let children = &root.children;

    let subgraph_cnt = children.len();
    let node_cnt = graph.collect_nodes(id).expect("root should exist in the graph").len();
    let edge_cnt = graph.collect_edges(id).expect("root should exist in the graph").len();

    let id = format!("{} (s: {} n: {} e: {})", id, subgraph_cnt, node_cnt, edge_cnt);
    let children: Vec<TreeItem<'static>> = children.iter().map(|node| to_tree(node, graph)).collect();

    TreeItem::new(id, children)
}
