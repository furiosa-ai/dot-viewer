#![allow(dead_code)]

use dot_graph::Graph;

use tui_tree_widget::{TreeItem, TreeState};

use rayon::prelude::*;

pub(crate) struct Item {
    id: String,
    children: Vec<Item>,
}

// https://github.com/EdJoPaTo/tui-rs-tree-widget/blob/main/examples/util/mod.rs
pub(crate) struct Tree {
    pub state: TreeState,
    pub tree: Vec<TreeItem<'static>>,
    items: Vec<Item>,
}

impl Tree {
    pub(crate) fn from_graph(graph: &Graph) -> Self {
        let root = graph.search_subgraph(graph.id()).unwrap().id();

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

    pub(crate) fn selected(&self) -> Option<String> {
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

    pub(crate) fn first(&mut self) {
        self.state.select_first();
    }

    pub(crate) fn last(&mut self) {
        self.state.select_last(&self.tree);
    }

    pub(crate) fn down(&mut self) {
        self.state.key_down(&self.tree);
    }

    pub(crate) fn up(&mut self) {
        self.state.key_up(&self.tree);
    }

    pub(crate) fn left(&mut self) {
        self.state.key_left();
    }

    pub(crate) fn right(&mut self) {
        self.state.key_right();
    }

    pub(crate) fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}

fn to_item(root: &String, graph: &Graph) -> Item {
    let id = root.clone();

    let children = graph.collect_subgraphs(root).expect("root should exist in the graph");
    let mut children: Vec<Item> = children
        .par_iter()
        .map(|&id| {
            let child = graph.search_subgraph(id).unwrap();
            to_item(child.id(), graph)
        })
        .collect();
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
    let children: Vec<TreeItem<'static>> =
        children.iter().map(|node| to_tree(node, graph)).collect();

    TreeItem::new(id, children)
}
