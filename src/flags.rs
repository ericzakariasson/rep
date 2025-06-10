#[derive(Debug, PartialEq, Clone)]
pub enum Flag {
    LineNumbers,
    CaseInsensitive,
    Invert,
    Count,
    WordMatch,
    Verbose,
}

impl Flag {
    pub fn from_arg(arg: &str) -> Option<Self> {
        match arg {
            "-n" => Some(Flag::LineNumbers),
            "-i" => Some(Flag::CaseInsensitive),
            "-c" => Some(Flag::Count),
            "-v" => Some(Flag::Invert),
            "-w" => Some(Flag::WordMatch),
            "-V" => Some(Flag::Verbose),
            _ => None,
        }
    }
}

pub fn parse_flags(args: &[String]) -> Vec<Flag> {
    args.iter()
        .skip(1)
        .filter_map(|arg| Flag::from_arg(arg))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_from_arg() {
        assert_eq!(Flag::from_arg("-n"), Some(Flag::LineNumbers));
        assert_eq!(Flag::from_arg("-i"), Some(Flag::CaseInsensitive));
        assert_eq!(Flag::from_arg("-c"), Some(Flag::Count));
        assert_eq!(Flag::from_arg("-v"), Some(Flag::Invert));
        assert_eq!(Flag::from_arg("-w"), Some(Flag::WordMatch));
        assert_eq!(Flag::from_arg("-V"), Some(Flag::Verbose));
        assert_eq!(Flag::from_arg("-x"), None);
        assert_eq!(Flag::from_arg("pattern"), None);
    }

    #[test]
    fn test_parse_flags_single() {
        let args = vec!["program".to_string(), "-n".to_string(), "pattern".to_string()];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::LineNumbers]);
    }

    #[test]
    fn test_parse_flags_multiple() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "-i".to_string(),
            "pattern".to_string()
        ];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::LineNumbers, Flag::CaseInsensitive]);
    }

    #[test]
    fn test_parse_flags_with_count() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "-c".to_string(),
            "pattern".to_string()
        ];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::LineNumbers, Flag::Count]);
    }

    #[test]
    fn test_parse_flags_mixed_order() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "pattern".to_string(),
            "-i".to_string(),
            "file.txt".to_string()
        ];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::LineNumbers, Flag::CaseInsensitive]);
    }

    #[test]
    fn test_parse_flags_with_verbose() {
        let args = vec![
            "program".to_string(),
            "-V".to_string(),
            "-n".to_string(),
            "pattern".to_string(),
            "file.txt".to_string()
        ];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::Verbose, Flag::LineNumbers]);
    }

    #[test]
    fn test_empty_flags() {
        let args = vec!["program".to_string(), "pattern".to_string(), "file.txt".to_string()];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![]);
    }

    #[test]
    fn test_duplicate_flags() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "-n".to_string(),
            "-i".to_string()
        ];
        let flags = parse_flags(&args);
        assert_eq!(flags, vec![Flag::LineNumbers, Flag::LineNumbers, Flag::CaseInsensitive]);
    }

    #[test]
    fn test_flag_combinations() {
        let flags = vec![Flag::LineNumbers, Flag::CaseInsensitive];
        
        assert_eq!(flags.contains(&Flag::LineNumbers), true);
        assert_eq!(flags.contains(&Flag::CaseInsensitive), true);
        assert_eq!(flags.contains(&Flag::Invert), false);
    }
} 