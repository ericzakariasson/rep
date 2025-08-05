use std::fmt;

#[derive(Debug)]
pub enum RepError {
    InvalidArguments(String),
    FileNotFound(String),
    GlobPatternError(String),
    IoError(String),
    NoFilesMatched,
    RegexError(String),
}

impl fmt::Display for RepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepError::InvalidArguments(msg) => {
                write!(f, "❌ Invalid command line arguments\n   {}", msg)
            }
            RepError::FileNotFound(path) => {
                writeln!(f, "❌ File not found: {}", path)?;
                writeln!(f)?;
                writeln!(f, "💡 Suggestions:")?;
                writeln!(f, "   • Check if the file path is correct")?;
                writeln!(f, "   • Verify you have read permissions")?;
                write!(f, "   • Use absolute path if needed: {}", std::env::current_dir()
                    .map(|p| p.join(path).to_string_lossy().to_string())
                    .unwrap_or_else(|_| format!("/full/path/to/{}", path)))
            }
            RepError::GlobPatternError(pattern) => {
                writeln!(f, "❌ Invalid file pattern: {}", pattern)?;
                writeln!(f)?;
                writeln!(f, "💡 Valid pattern examples:")?;
                writeln!(f, "   • *.txt      - all .txt files")?;
                writeln!(f, "   • src/*.rs   - all .rs files in src/")?;
                writeln!(f, "   • **/*.md    - all .md files recursively")?;
                write!(f, "   • file.txt   - specific file")
            }
            RepError::IoError(msg) => {
                writeln!(f, "❌ File system error: {}", msg)?;
                writeln!(f)?;
                writeln!(f, "💡 Possible causes:")?;
                writeln!(f, "   • Insufficient permissions")?;
                writeln!(f, "   • File is locked by another process")?;
                writeln!(f, "   • Disk space or I/O errors")?;
                write!(f, "   • Network issues (if accessing remote files)")
            }
            RepError::NoFilesMatched => {
                writeln!(f, "❌ No files found matching the specified patterns")?;
                writeln!(f)?;
                writeln!(f, "💡 Try:")?;
                writeln!(f, "   • Check if the file patterns are correct")?;
                writeln!(f, "   • Use ls or find to verify files exist")?;
                writeln!(f, "   • Try broader patterns like '*' or '**/*'")?;
                write!(f, "   • Ensure you're in the correct directory")
            }
            RepError::RegexError(msg) => {
                writeln!(f, "❌ Invalid search pattern: {}", msg)?;
                writeln!(f)?;
                writeln!(f, "💡 Pattern tips:")?;
                writeln!(f, "   • Use quotes around patterns with special characters")?;
                writeln!(f, "   • Escape special chars: \\. \\* \\+ \\? \\^ \\$ \\| \\( \\) \\[ \\]")?;
                write!(f, "   • Try simpler patterns if regex seems complex")
            }
        }
    }
}

impl std::error::Error for RepError {}

pub type Result<T> = std::result::Result<T, RepError>; 