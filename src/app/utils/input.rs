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

    pub fn push(&mut self, c: char) {
        self.key.push(c);
        self.cursor += 1;
    }

    pub fn pop(&mut self) {
        if !self.key.is_empty() {
            self.key.pop();
            self.cursor -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.key = String::from("");
        self.cursor = 0;
    }
}
