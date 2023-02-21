#![allow(dead_code)]

#[derive(Default)]
pub(crate) struct Input {
    pub key: String,
    pub cursor: usize,
    history: Vec<String>,
}

impl Input {
    pub fn new() -> Input {
        Input::default()
    }

    pub fn set(&mut self, key: String) {
        self.key = key;
        self.cursor = self.key.len();
    }

    pub fn front(&mut self) {
        if self.cursor < self.key.len() {
            self.cursor += 1;
        }
    }

    pub fn back(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn insert(&mut self, c: char) {
        self.key.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.key.remove(self.cursor);
        }
    }

    pub fn clear(&mut self) {
        self.history.push(self.key.clone());
        self.key = String::from("");
        self.cursor = 0;
    }
}
