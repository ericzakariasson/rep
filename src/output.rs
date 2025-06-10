use crate::flags::Flag;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OutputMode {
    FullLines,
    Count,
}

impl OutputMode {
    pub fn from_flags(flags: &[Flag]) -> Self {
        for flag in flags.iter().rev() {
            if matches!(flag, Flag::Count) {
                return OutputMode::Count;
            }
        }
        OutputMode::FullLines
    }
}

pub struct OutputConfig {
    pub mode: OutputMode,
    pub show_line_numbers: bool,
    pub show_filename: bool,
}

impl OutputConfig {
    pub fn new(flags: &[Flag], multiple_files: bool) -> Self {
        let mode = OutputMode::from_flags(flags);
        let show_line_numbers = flags.contains(&Flag::LineNumbers) && mode != OutputMode::Count;
        
        Self {
            mode,
            show_line_numbers,
            show_filename: multiple_files,
        }
    }
}

pub fn format_match(line: &str, line_number: usize, filename: Option<&str>, config: &OutputConfig) -> String {
    let mut parts = Vec::new();
    
    if let Some(fname) = filename {
        if config.show_filename {
            parts.push(fname.to_string());
        }
    }
    
    if config.show_line_numbers {
        parts.push((line_number + 1).to_string());
    }
    
    if parts.is_empty() {
        line.to_string()
    } else {
        parts.push(line.to_string());
        parts.join(":")
    }
}

pub fn format_count(count: usize, filename: Option<&str>, show_filename: bool) -> String {
    match (filename, show_filename) {
        (Some(fname), true) => format!("{}:{}", fname, count),
        _ => count.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_mode_from_flags() {
        assert_eq!(OutputMode::from_flags(&[]), OutputMode::FullLines);
        assert_eq!(OutputMode::from_flags(&[Flag::LineNumbers]), OutputMode::FullLines);
        assert_eq!(OutputMode::from_flags(&[Flag::Count]), OutputMode::Count);
        assert_eq!(OutputMode::from_flags(&[Flag::LineNumbers, Flag::Count]), OutputMode::Count);
    }

    #[test]
    fn test_output_config() {
        let config = OutputConfig::new(&[Flag::LineNumbers], false);
        assert_eq!(config.mode, OutputMode::FullLines);
        assert!(config.show_line_numbers);
        assert!(!config.show_filename);

        let config = OutputConfig::new(&[Flag::LineNumbers, Flag::Count], true);
        assert_eq!(config.mode, OutputMode::Count);
        assert!(!config.show_line_numbers);
        assert!(config.show_filename);
    }

    #[test]
    fn test_format_match() {
        let config = OutputConfig {
            mode: OutputMode::FullLines,
            show_line_numbers: false,
            show_filename: false,
        };
        assert_eq!(format_match("test line", 0, None, &config), "test line");

        let config = OutputConfig {
            mode: OutputMode::FullLines,
            show_line_numbers: true,
            show_filename: false,
        };
        assert_eq!(format_match("test line", 0, None, &config), "1:test line");

        let config = OutputConfig {
            mode: OutputMode::FullLines,
            show_line_numbers: true,
            show_filename: true,
        };
        assert_eq!(format_match("test line", 0, Some("file.txt"), &config), "file.txt:1:test line");
    }

    #[test]
    fn test_format_count() {
        assert_eq!(format_count(5, None, false), "5");
        assert_eq!(format_count(5, Some("file.txt"), false), "5");
        assert_eq!(format_count(5, Some("file.txt"), true), "file.txt:5");
    }
} 