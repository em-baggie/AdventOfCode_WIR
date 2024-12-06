use std::error::Error;
use std::fmt;
use std::io;

/// Custom error types for the application - credit to @hollygrimm!!
#[derive(Debug)]
pub enum FindXmasError {
    /// Represents I/O operation failures
    IoError(io::Error),
    /// Represents failure to create Vec<Vec<char>> from input data with rows and columnds of equal lengths
    VecCreationError(String),
}

impl From<io::Error> for FindXmasError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl FindXmasError {
    pub fn from_shape_error(message: String) -> Self {
        Self::VecCreationError(message)
    }
}

impl Error for FindXmasError {}

impl fmt::Display for FindXmasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::VecCreationError(msg) => write!(f, "Failed to create Vec<Vec<char>>: {}", msg),
        }
    }
}
