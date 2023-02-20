use std::fmt;

#[derive(Debug)]
pub(crate) enum SuccessState {
    ExportSuccess(String),
    XdotSuccess,
    Silent,
}

impl Default for SuccessState {
    fn default() -> Self {
        Self::Silent
    }
}

impl fmt::Display for SuccessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ExportSuccess(filename) => write!(f, "successfully exported to {filename}"),
            Self::XdotSuccess => write!(f, "launched xdot"),
            Self::Silent => Ok(()),
        }
    }
}
