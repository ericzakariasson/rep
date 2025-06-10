use std::fmt;

#[derive(Debug)]
pub enum RepError {
    InvalidArguments(String),
    FileNotFound(String),
    GlobPatternError(String),
    IoError(String),
    NoFilesMatched,
    Help(String),
}

impl fmt::Display for RepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepError::InvalidArguments(msg) => {
                write!(f, "❌ Invalid arguments: {}\n\n💡 Tip: Use 'rep --help' to see usage instructions", msg)
            },
            RepError::FileNotFound(path) => {
                write!(f, "❌ Cannot find file: '{}'\n\n💡 Tips:\n  • Check if the file path is correct\n  • Make sure you have read permissions\n  • Use quotes for paths with spaces", path)
            },
            RepError::GlobPatternError(pattern) => {
                write!(f, "❌ Invalid file pattern: {}\n\n💡 Tips:\n  • Use * to match multiple characters (e.g., *.txt)\n  • Use ? to match a single character\n  • Use [abc] to match any of a, b, or c\n  • Escape special characters with \\", pattern)
            },
            RepError::IoError(msg) => {
                write!(f, "❌ File operation failed: {}\n\n💡 Tips:\n  • Check if you have permission to read the file\n  • Ensure the file is not locked by another process\n  • Verify there's enough disk space", msg)
            },
            RepError::NoFilesMatched => {
                write!(f, "❌ No files found matching your pattern\n\n💡 Tips:\n  • Check if the files exist in the current directory\n  • Try using a simpler pattern (e.g., *.txt instead of complex globs)\n  • Use 'ls' or 'dir' to see available files\n  • Ensure you're in the correct directory")
            },
            RepError::Help(msg) => {
                // Help is not an error, so we don't use error formatting
                write!(f, "{}", msg)
            },
        }
    }
}

impl std::error::Error for RepError {}

pub type Result<T> = std::result::Result<T, RepError>; 