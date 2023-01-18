use std::str;
use trie_rs::{ Trie, TrieBuilder };

pub struct SearchTrie {
    pub trie: Trie<u8>,
}

impl SearchTrie {
    pub fn new(ids: &Vec<String>) -> SearchTrie {
        let mut builder = TrieBuilder::new();
        for id in ids {
            builder.push(id.clone());
        }
        let trie = builder.build();

        SearchTrie { trie }
    }

    pub fn autocomplete(&self, str: &str) -> Option<String> {
        let predictions = self.predict(str);
        Self::longest_common_prefix(predictions)
    }

    fn predict(&self, str: &str) -> Vec<String> {
        let result = self.trie.predictive_search(str);
        let result: Vec<String> = result.iter().map(|s| str::from_utf8(s).unwrap().to_string()).collect();

        result
    }

    // TODO faster search needed
    // https://leetcode.com/problems/longest-common-prefix/solutions/1134124/faster-than-100-in-memory-and-runtime-by-rust/
    fn longest_common_prefix(strs: Vec<String>) -> Option<String> {
        if strs.len() == 0 || strs[0].len() == 0 {
            return None;
        }

        let mut idx = 0;
        for i in 0..strs[0].len() {
            let c = strs[0].chars().nth(i);
            for str in &strs {
                if let Some(x) = str.chars().nth(i) {
                    if c != Some(x) {
                        return Some(strs[0][..i].to_string());
                    }
                } else {
                    return Some(strs[0][..i].to_string());
                }
            }
            idx = i;
        }

        Some(strs[0][..=idx].to_string())
    }
}
