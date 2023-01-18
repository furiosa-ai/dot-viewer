pub mod terminal;
pub mod ui;
pub mod app;

#[cfg(test)]
mod tests {
    #[test]
    fn autocomplete() {
        let graph = dot_graph::parser::parse("graph.dot"); 
        let nodes: Vec<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();  
        let trie = crate::app::utils::trie::SearchTrie::new(&nodes);

        assert_eq!(trie.autocomplete("g"), Some("graph".to_string()));
    }
}
