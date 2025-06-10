use std::fmt;

#[derive(Debug)]
pub enum RepError {
    InvalidArguments(String),
    FileNotFound(String),
    GlobPatternError(String),
    IoError(String),
    NoFilesMatched,
}

impl fmt::Display for RepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
            RepError::FileNotFound(path) => write!(f, "File not found: {}", path),
            RepError::GlobPatternError(pattern) => write!(f, "Invalid glob pattern: {}", pattern),
            RepError::IoError(msg) => write!(f, "IO error: {}", msg),
            RepError::NoFilesMatched => write!(f, "No files found matching patterns"),
        }
    }
}

impl std::error::Error for RepError {}

pub type Result<T> = std::result::Result<T, RepError>; 