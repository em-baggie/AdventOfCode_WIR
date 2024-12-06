use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CheckUpdatesError {
    IoError(io::Error),
    CreateHashMapError(String),
    CheckUpdateError(String),
}

impl From<io::Error> for CheckUpdatesError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl Error for CheckUpdatesError {}

impl fmt::Display for CheckUpdatesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::CreateHashMapError(msg) => write!(f, "Unable to parse to hashmap: {}", msg),
            Self::CheckUpdateError(msg) => write!(f, "Unable check update: {}", msg),
        }
    }
}
