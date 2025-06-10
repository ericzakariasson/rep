use crate::flags::{Flag, parse_flags};
use crate::error::{RepError, Result};

#[derive(Debug, PartialEq)]
pub struct ParsedArgs {
    pub flags: Vec<Flag>,
    pub pattern: String,
    pub file_patterns: Vec<String>,
}

impl ParsedArgs {
    fn usage_string(program_name: &str) -> String {
        format!("Usage: {} [OPTIONS] PATTERN FILE...\n\nOptions:\n  -n    Show line numbers\n  -i    Case-insensitive search\n  -c    Count matches only\n  -v    Invert match (show non-matching lines)", program_name)
    }
}

pub fn parse_args(args: &[String]) -> Result<ParsedArgs> {
    if args.is_empty() {
        return Err(RepError::InvalidArguments(
            "No program name found".to_string()
        ));
    }
    
    let program_name = &args[0];
    
    if args.len() < 3 {
        let missing = if args.len() == 1 {
            "both a search pattern and file(s)"
        } else {
            "file(s) to search in"
        };
        return Err(RepError::InvalidArguments(
            format!("Missing {}.\n\n{}", missing, ParsedArgs::usage_string(program_name))
        ));
    }
    
    let flags = parse_flags(args);
    let non_flag_args = extract_non_flag_args(args);
    
    if non_flag_args.len() < 2 {
        return Err(RepError::InvalidArguments(
            format!("Not enough arguments after flags.\n\n{}", ParsedArgs::usage_string(program_name))
        ));
    }
    
    let pattern = non_flag_args[0].clone();
    let file_patterns = non_flag_args[1..].to_vec();
    
    Ok(ParsedArgs {
        flags,
        pattern,
        file_patterns,
    })
}

fn extract_non_flag_args(args: &[String]) -> Vec<String> {
    args.iter()
        .skip(1)
        .filter(|arg| Flag::from_arg(arg).is_none())
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flags::Flag;

    #[test]
    fn test_parse_args_basic() {
        let args = vec![
            "program".to_string(),
            "pattern".to_string(),
            "file.txt".to_string()
        ];
        let result = parse_args(&args).unwrap();
        
        assert_eq!(result.flags, vec![]);
        assert_eq!(result.pattern, "pattern");
        assert_eq!(result.file_patterns, vec!["file.txt"]);
    }

    #[test]
    fn test_parse_args_with_flags() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "-i".to_string(),
            "pattern".to_string(),
            "file.txt".to_string()
        ];
        let result = parse_args(&args).unwrap();
        
        assert_eq!(result.flags, vec![Flag::LineNumbers, Flag::CaseInsensitive]);
        assert_eq!(result.pattern, "pattern");
        assert_eq!(result.file_patterns, vec!["file.txt"]);
    }

    #[test]
    fn test_parse_args_multiple_files() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "pattern".to_string(),
            "file1.txt".to_string(),
            "file2.txt".to_string()
        ];
        let result = parse_args(&args).unwrap();
        
        assert_eq!(result.flags, vec![Flag::LineNumbers]);
        assert_eq!(result.pattern, "pattern");
        assert_eq!(result.file_patterns, vec!["file1.txt", "file2.txt"]);
    }

    #[test]
    fn test_parse_args_mixed_flag_order() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "pattern".to_string(),
            "-i".to_string(),
            "file.txt".to_string()
        ];
        let result = parse_args(&args).unwrap();
        
        assert_eq!(result.flags, vec![Flag::LineNumbers, Flag::CaseInsensitive]);
        assert_eq!(result.pattern, "pattern");
        assert_eq!(result.file_patterns, vec!["file.txt"]);
    }

    #[test]
    fn test_parse_args_insufficient_args() {
        let args = vec!["program".to_string()];
        let result = parse_args(&args);
        assert!(result.is_err());
        
        let args = vec!["program".to_string(), "pattern".to_string()];
        let result = parse_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_empty() {
        let args = vec![];
        let result = parse_args(&args);
        assert!(result.is_err());
    }
} 