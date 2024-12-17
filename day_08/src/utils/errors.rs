use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AntiNodeError {
    IoError(io::Error),
    EmptyFile,
}

impl From<io::Error> for AntiNodeError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl Error for AntiNodeError {}

impl fmt::Display for AntiNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::EmptyFile => write!(f, "Input file is empty"),
        }
    }
}
