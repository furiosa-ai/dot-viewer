use crossterm::event::KeyCode;
use dot_graph::DotGraphError;
use thiserror::Error;

pub type Res = Result<Option<String>, DotViewerError>;

#[derive(Error, Debug)]
pub enum DotViewerError {
    #[error("Err: parse failed in dot-graph, {}", .0)]
    ParseError(DotGraphError),
    #[error("Err: graph manipulation failed with, `{0}`")]
    GraphError(String),
    #[error("Err: no keybinding for {:?}", .0)]
    KeyError(KeyCode),
    #[error("Err: file io failed with, `{0}`")]
    IOError(String),
    #[error("Err: failed to launch xdot.py")]
    XdotError,
    #[error("Err: tab manipulation failed with, `{0}`")]
    TabError(String),
}
