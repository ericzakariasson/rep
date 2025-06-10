use crate::flags::Flag;

pub struct SearchResult {
    pub matches: Vec<MatchedLine>,
    pub total_count: usize,
}

#[derive(Debug, Clone)]
pub struct MatchedLine {
    pub line_number: usize,
    pub content: String,
}

pub struct SearchConfig {
    pub case_insensitive: bool,
    pub invert_match: bool,
}

impl SearchConfig {
    pub fn from_flags(flags: &[Flag]) -> Self {
        Self {
            case_insensitive: flags.contains(&Flag::CaseInsensitive),
            invert_match: flags.contains(&Flag::Invert),
        }
    }
}

pub fn search(content: &str, pattern: &str, config: &SearchConfig) -> SearchResult {
    let matches: Vec<MatchedLine> = content
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            let is_match = if config.case_insensitive {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };
            
            let should_include = if config.invert_match {
                !is_match
            } else {
                is_match
            };
            
            if should_include {
                Some(MatchedLine {
                    line_number,
                    content: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    
    let total_count = matches.len();
    
    SearchResult {
        matches,
        total_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_search() {
        let content = "line one\nline two\nline three";
        let config = SearchConfig {
            case_insensitive: false,
            invert_match: false,
        };
        
        let result = search(content, "two", &config);
        assert_eq!(result.total_count, 1);
        assert_eq!(result.matches[0].line_number, 1);
        assert_eq!(result.matches[0].content, "line two");
    }

    #[test]
    fn test_case_insensitive_search() {
        let content = "Line One\nLINE TWO\nline three";
        let config = SearchConfig {
            case_insensitive: true,
            invert_match: false,
        };
        
        let result = search(content, "line", &config);
        assert_eq!(result.total_count, 3);
    }

    #[test]
    fn test_inverted_search() {
        let content = "line one\nline two\nline three";
        let config = SearchConfig {
            case_insensitive: false,
            invert_match: true,
        };
        
        let result = search(content, "two", &config);
        assert_eq!(result.total_count, 2);
        assert_eq!(result.matches[0].content, "line one");
        assert_eq!(result.matches[1].content, "line three");
    }

    #[test]
    fn test_no_matches() {
        let content = "line one\nline two\nline three";
        let config = SearchConfig {
            case_insensitive: false,
            invert_match: false,
        };
        
        let result = search(content, "four", &config);
        assert_eq!(result.total_count, 0);
        assert!(result.matches.is_empty());
    }
} 