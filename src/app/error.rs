use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum ViewerError {
    KeyError(KeyCode),
}

impl std::error::Error for ViewerError {}

impl std::fmt::Display for ViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ViewerError::KeyError(key) => write!(f, "Err: Wrong key {:?}", key),
        }
    }
}
