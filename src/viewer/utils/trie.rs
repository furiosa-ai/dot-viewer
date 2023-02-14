#![allow(dead_code)]

use std::str;
use trie_rs::TrieBuilder;

pub(crate) struct Trie {
    pub(crate) items: Vec<String>,
    pub(crate) trie: trie_rs::Trie<u8>,
}

impl Trie {
    pub(crate) fn new(ids: &[String]) -> Trie {
        let items = ids.to_vec();

        let mut builder = TrieBuilder::new();
        for id in ids {
            builder.push(id.clone());
        }
        let trie = builder.build();

        Trie { items, trie }
    }

    pub(crate) fn autocomplete(&self, key: &str) -> Option<String> {
        let predictions = if key.is_empty() { self.items.clone() } else { self.predict(key) };

        Self::longest_common_prefix(predictions)
    }

    fn predict(&self, key: &str) -> Vec<String> {
        let result = self.trie.predictive_search(key);
        let result: Vec<String> =
            result.iter().map(|s| str::from_utf8(s).unwrap().to_string()).collect();

        result
    }

    // https://leetcode.com/problems/longest-common-prefix/solutions/1134124/faster-than-100-in-memory-and-runtime-by-rust/
    fn longest_common_prefix(strs: Vec<String>) -> Option<String> {
        if strs.is_empty() || strs[0].is_empty() {
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
