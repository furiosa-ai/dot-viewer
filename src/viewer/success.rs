use std::fmt;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Success {
    ExportSuccess(String),
    XdotSuccess,
    Silent,
}

impl Default for Success {
    fn default() -> Success {
        Success::Silent
    }
}

impl fmt::Display for Success {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Success::ExportSuccess(filename) => write!(f, "successfully exported to {filename}"),
            Success::XdotSuccess => write!(f, "launched xdot"),
            Success::Silent => Ok(()),
        }
    }
}
