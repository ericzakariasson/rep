use std::path::PathBuf;
use std::fs;
use glob::glob;
use crate::error::{RepError, Result};

pub fn expand_file_patterns(patterns: &[String]) -> Result<Vec<PathBuf>> {
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
                            format!("Failed to process '{}': {}", pattern, e)
                        )),
                    }
                }
                // Check if this specific pattern matched any files
                if !pattern_matched && !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
                    // This looks like a specific filename that doesn't exist
                    return Err(RepError::FileNotFound(pattern.clone()));
                }
            }
            Err(e) => {
                return Err(RepError::GlobPatternError(
                    format!("'{}' is not a valid file pattern: {}", pattern, e)
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
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            let filename = path.to_string_lossy();
            let error_msg = match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("'{}' does not exist", filename)
                }
                std::io::ErrorKind::PermissionDenied => {
                    format!("Permission denied: cannot read '{}'", filename)
                }
                _ => {
                    format!("Cannot read '{}': {}", filename, e)
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