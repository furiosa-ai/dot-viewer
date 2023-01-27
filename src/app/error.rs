use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum ViewerError {
    KeyError(KeyCode),
    GoToError(String),
    FilterError(String),
    IOError(String),
    XdotError,
    TODOError(String),
}

impl std::error::Error for ViewerError {}

impl std::fmt::Display for ViewerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ViewerError::KeyError(key) => write!(f, "KeyErr: Wrong key {:?}", key),
            ViewerError::GoToError(msg) => write!(f, "GoToErr: {}", msg),
            ViewerError::FilterError(msg) => write!(f, "FilterErr: {}", msg),
            ViewerError::IOError(msg) => write!(f, "IOErr: {}", msg),
            ViewerError::XdotError => write!(f, "XdotErr: cannot launch xdot"),
            ViewerError::TODOError(msg) => write!(f, "TODOErr: {}", msg),
        }
    }
}
