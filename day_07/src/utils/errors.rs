use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum BridgeRepairError {
    IoError(io::Error),
    RegexError(regex::Error),
    ParseError(ParseIntError),
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

impl From<ParseIntError> for BridgeRepairError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(error)
    }
}

impl Error for BridgeRepairError {}

impl fmt::Display for BridgeRepairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::RegexError(e) => write!(f, "Regex error: {}", e),
            Self::ParseError(e) => write!(f, "Error parsing input: {}", e),
        }
    }
}
