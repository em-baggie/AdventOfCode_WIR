use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum BridgeRepairError {
    IoError(io::Error),
    RegexError(regex::Error),
}

impl From<io::Error> for BridgeRepairError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<regex::Error> for BridgeRepairError {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

impl Error for BridgeRepairError {}

impl fmt::Display for BridgeRepairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::RegexError(e) => write!(f, "Regex error: {}", e),
        }
    }
}
