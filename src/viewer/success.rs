use std::fmt;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Success {
    ExportSuccess(String),
    XdotSuccess,
    Silent,
}

impl Default for Success {
    fn default() -> Self {
        Self::Silent
    }
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::ExportSuccess(filename) => write!(f, "successfully exported to {filename}"),
            Self::XdotSuccess => write!(f, "launched xdot"),
            Self::Silent => Ok(()),
        }
    }
}
