use std::error::Error;
use std::fmt;
use std::io;

/// Guard Gallivant Error
/// 
/// Custom error handling for the Guard Gallivant program.
/// 
/// variants:
/// - IoError: An error from the io library
/// - ParseError: An error from the parser
/// - FindPositionsError: An error from the find positions function

#[derive(Debug)]
pub enum GuardGallivantError {
    IoError(io::Error),
    ParseError(String),
    FindPositionsError(String),
}

impl From<io::Error> for GuardGallivantError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl Error for GuardGallivantError {}

impl fmt::Display for GuardGallivantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::FindPositionsError(msg) => write!(f, "Find positions error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
