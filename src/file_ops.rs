use std::path::PathBuf;
use std::fs;
use glob::glob;
use crate::error::{RepError, Result};

pub fn expand_file_patterns(patterns: &[String]) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();
    
    for pattern in patterns {
        match glob(pattern) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => file_paths.push(path),
                        Err(e) => return Err(RepError::GlobPatternError(e.to_string())),
                    }
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
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            let filename = path.to_string_lossy();
            Err(RepError::IoError(
                format!("Error reading file {}: {}", filename, e)
            ))
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