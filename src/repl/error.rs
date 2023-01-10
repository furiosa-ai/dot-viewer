#[derive(Debug)]
pub enum ReplError {
    UnknownCommandError(String, Vec<String>),
    ArgumentError,
    ParseError,
    NoGraphError,
    NoNodeError,
    FileError(String),
}

impl std::error::Error for ReplError {}

impl std::fmt::Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReplError::UnknownCommandError(command, arguments) => write!(f, "Unknown command {} with arguments {:?}", command, arguments),
            ReplError::ArgumentError => write!(f, "Wrong arguments"),
            ReplError::ParseError => write!(f, "Parse error"),
            ReplError::NoGraphError => write!(f, "There is no graph open. Please open a graph"),
            ReplError::NoNodeError => write!(f, "No such node in graph"),
            ReplError::FileError(filename) => write!(f, "Failed to open or write on {}", filename.as_str()),
        }
    }
}
