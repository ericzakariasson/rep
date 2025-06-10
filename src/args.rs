use crate::flags::Flag;
use crate::error::{RepError, Result};

#[derive(Debug, PartialEq)]
pub struct ParsedArgs {
    pub flags: Vec<Flag>,
    pub pattern: String,
    pub file_patterns: Vec<String>,
}

impl ParsedArgs {
    fn usage_string(program_name: &str) -> String {
        format!("Usage: {} [-n] [-i] [-c] [-v] <pattern> <filename>", program_name)
    }
}

pub fn parse_args(args: &[String]) -> Result<ParsedArgs> {
    if args.is_empty() {
        return Err(RepError::InvalidArguments(
            "No arguments provided".to_string()
        ));
    }
    
    let program_name = &args[0];
    
    // Accumulate flags and non-flag arguments while performing validation
    let mut flags = Vec::new();
    let mut non_flag_args: Vec<String> = Vec::new();

    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            match Flag::from_arg(arg) {
                Some(flag) => flags.push(flag),
                None => return Err(RepError::UnknownFlag(
                    format!("{}\n{}", arg, ParsedArgs::usage_string(program_name))
                )),
            }
        } else {
            non_flag_args.push(arg.clone());
        }
    }

    if non_flag_args.len() < 2 {
        return Err(RepError::InvalidArguments(ParsedArgs::usage_string(program_name)));
    }

    let pattern = non_flag_args[0].clone();
    let file_patterns = non_flag_args[1..].to_vec();
    
    Ok(ParsedArgs {
        flags,
        pattern,
        file_patterns,
    })
}

#[allow(dead_code)]
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