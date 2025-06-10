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
            RepError::InvalidArguments(msg) => {
                write!(f, "❌ Invalid command: {}\n\n", msg)?;
                write!(f, "💡 Tip: Make sure you provide both a search pattern and at least one file.\n")?;
                write!(f, "   Example: rep \"search text\" file.txt")
            }
            RepError::FileNotFound(path) => {
                write!(f, "❌ Cannot find file: '{}'\n\n", path)?;
                write!(f, "💡 Tips:\n")?;
                write!(f, "   • Check if the file path is correct\n")?;
                write!(f, "   • Make sure you have read permissions\n")?;
                write!(f, "   • Try using the full path to the file")
            }
            RepError::GlobPatternError(pattern) => {
                write!(f, "❌ Invalid file pattern: {}\n\n", pattern)?;
                write!(f, "💡 Tips for glob patterns:\n")?;
                write!(f, "   • Use *.txt to match all .txt files\n")?;
                write!(f, "   • Use **/*.rs to match .rs files in all subdirectories\n")?;
                write!(f, "   • Escape special characters with backslash")
            }
            RepError::IoError(msg) => {
                write!(f, "❌ File operation failed: {}\n\n", msg)?;
                write!(f, "💡 Common causes:\n")?;
                write!(f, "   • Insufficient permissions\n")?;
                write!(f, "   • File is locked by another process\n")?;
                write!(f, "   • Disk space or quota issues")
            }
            RepError::NoFilesMatched => {
                write!(f, "❌ No files found matching your pattern\n\n")?;
                write!(f, "💡 Tips:\n")?;
                write!(f, "   • Check if files exist in the current directory\n")?;
                write!(f, "   • Try a broader pattern (e.g., *.txt instead of specific names)\n")?;
                write!(f, "   • Use 'ls' to see available files")
            }
        }
    }
}

impl std::error::Error for RepError {}

pub type Result<T> = std::result::Result<T, RepError>; 