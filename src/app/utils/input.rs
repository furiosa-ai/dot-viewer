pub struct Input {
    pub key: String,
    pub history: Vec<String>,
    pub cursor: usize,
}

impl Input {
    pub fn new() -> Input {
        Input {
            key: String::from(""),
            history: Vec::new(),
            cursor: 0,
        }
    }

    pub fn key(&self) -> String {
        self.key.clone()
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
        if self.cursor > 0{
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
        self.key = String::from("");
        self.cursor = 0;
    }
}
