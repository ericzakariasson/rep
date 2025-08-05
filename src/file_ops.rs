use std::path::PathBuf;
use std::fs;
use glob::glob;
use crate::error::{RepError, Result};

pub fn expand_file_patterns(patterns: &[String]) -> Result<Vec<PathBuf>> {
    if patterns.is_empty() {
        return Err(RepError::InvalidArguments(
            "No file patterns provided. Please specify at least one file or pattern.".to_string()
        ));
    }

    let mut file_paths = Vec::new();
    
    for pattern in patterns {
        match glob(pattern) {
            Ok(paths) => {
                let mut pattern_matched = false;
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            pattern_matched = true;
                            file_paths.push(path);
                        },
                        Err(e) => return Err(RepError::GlobPatternError(
                            format!("Error processing glob pattern '{}': {}", pattern, e)
                        )),
                    }
                }
                
                if !pattern_matched {
                    eprintln!("⚠️  Warning: Pattern '{}' matched no files", pattern);
                }
            }
            Err(e) => {
                return Err(RepError::GlobPatternError(
                    format!("Invalid glob pattern '{}': {}", pattern, e)
                ));
            }
        }
    }
    
    if file_paths.is_empty() {
        Err(RepError::NoFilesMatched)
    } else {
        Ok(file_paths)
    }
}

pub fn read_file_contents(path: &PathBuf) -> Result<String> {
    // Check if path exists first for better error messages
    if !path.exists() {
        return Err(RepError::FileNotFound(path.to_string_lossy().to_string()));
    }
    
    // Check if it's a directory
    if path.is_dir() {
        return Err(RepError::IoError(
            format!("'{}' is a directory, not a file", path.to_string_lossy())
        ));
    }
    
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            let filename = path.to_string_lossy();
            let error_msg = match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied when reading file '{}'. Check file permissions.", filename)
                },
                std::io::ErrorKind::NotFound => {
                    format!("File '{}' not found", filename)
                },
                std::io::ErrorKind::InvalidData => {
                    format!("File '{}' contains invalid UTF-8 data. Try a different file or use a binary-safe tool.", filename)
                },
                _ => {
                    format!("Error reading file '{}': {}", filename, e)
                }
            };
            
            Err(RepError::IoError(error_msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_file_patterns_simple() {
        let patterns = vec!["nonexistent*.txt".to_string()];
        let result = expand_file_patterns(&patterns);
        assert!(result.is_err());
    }

    #[test]
    fn test_expand_file_patterns_empty() {
        let patterns = vec![];
        let result = expand_file_patterns(&patterns);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_nonexistent_file() {
        let path = PathBuf::from("nonexistent_file_that_should_not_exist.txt");
        let result = read_file_contents(&path);
        assert!(result.is_err());
    }
} 