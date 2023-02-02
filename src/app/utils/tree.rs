use dot_graph::Graph;
use rayon::prelude::*;
use tui_tree_widget::{TreeItem, TreeState};

pub struct Tree {
    pub state: TreeState,
    pub items: Vec<TreeItem<'static>>,
}

impl Tree {
    #[allow(dead_code)]
    pub fn with_graph(graph: &Graph) -> Self {
        let &root = graph.slookup.get_by_left(&graph.id).unwrap();
        let root = Self::to_item(root, graph);

        Self::with_items(vec![root])
    }

    fn to_item(root: usize, graph: &Graph) -> TreeItem<'static> {
        let id = graph.slookup.get_by_right(&root).unwrap();

        if let Some(subgraphs) = graph.subtree.get(&root) {
            let subgraphs: Vec<TreeItem> = subgraphs.par_iter().map(|&subgraph| Self::to_item(subgraph, graph)).collect();

            TreeItem::new(id.to_string(), subgraphs)
        } else {
            TreeItem::new_leaf(id.to_string())
        }
    }

    pub fn with_items(items: Vec<TreeItem<'static>>) -> Self {
        let mut tree = Self {
            state: TreeState::default(),
            items,
        };

        if !tree.items.is_empty() {
            tree.state.select_first();
        }
        
        tree
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
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
