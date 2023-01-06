use std::fmt;

#[derive(Debug)]
pub enum ViewerError {
    ReplError(repl_rs::Error),
    GotoError(String),
}

impl From<repl_rs::Error> for ViewerError {
    fn from(e: repl_rs::Error) -> Self {
        ViewerError::ReplError(e)
    }
}

impl fmt::Display for ViewerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewerError::ReplError(e) => write!(f, "{}", e),
            ViewerError::GotoError(s) => write!(f, "No Such Node: {}", s),
        }
    }
}

impl std::error::Error for ViewerError {}
