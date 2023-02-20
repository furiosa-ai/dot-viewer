#![allow(dead_code)]

#[derive(Default)]
pub(crate) struct Input {
    pub key: String,
    pub history: Vec<String>,
    pub cursor: usize,
}

impl Input {
    pub(crate) fn new() -> Input {
        Input::default()
    }

    pub(crate) fn key(&self) -> String {
        self.key.clone()
    }

    pub(crate) fn set(&mut self, key: String) {
        self.key = key;
        self.cursor = self.key.len();
    }

    pub(crate) fn front(&mut self) {
        if self.cursor < self.key.len() {
            self.cursor += 1;
        }
    }

    pub(crate) fn back(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub(crate) fn insert(&mut self, c: char) {
        self.key.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub(crate) fn delete(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.key.remove(self.cursor);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.history.push(self.key.clone());
        self.key = String::from("");
        self.cursor = 0;
    }
}
