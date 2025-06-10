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

    #[test]
    fn test_complex_word_matching() {
        // Test complex word matching scenarios similar to grep -w behavior
        let content = "The cat in the hat\n\
                      A cat caught a mouse\n\
                      concatenate this string\n\
                      scattered cats everywhere\n\
                      cat, dog, and bird\n\
                      The CAT is sleeping\n\
                      category: important\n\
                      cat123 is not a word\n\
                      123cat also not a word\n\
                      just cat alone";
        
        // Test 1: Word match should only match whole word "cat", not partial matches
        let config = SearchConfig {
            case_insensitive: false,
            invert_match: false,
        };
        
        let result = search(content, "cat", &config);
        assert_eq!(result.total_count, 4, "Should match only whole word 'cat'");
        
        // Verify the matched lines contain only whole word matches
        let expected_lines = vec![
            "The cat in the hat",
            "A cat caught a mouse", 
            "cat, dog, and bird",
            "just cat alone"
        ];
        
        for (i, expected) in expected_lines.iter().enumerate() {
            assert_eq!(result.matches[i].content, *expected);
        }
        
        // Test 2: Case insensitive word matching
        let config_insensitive = SearchConfig {
            case_insensitive: true,
            invert_match: false,
        };
        
        let result_insensitive = search(content, "cat", &config_insensitive);
        assert_eq!(result_insensitive.total_count, 5, "Should match 'cat' and 'CAT' case-insensitively");
        
        // Test 3: Regular substring search (no word matching) should match more
        let config_substring = SearchConfig {
            case_insensitive: false,
            invert_match: false,
        };
        
        let result_substring = search(content, "cat", &config_substring);
        assert_eq!(result_substring.total_count, 9, "Substring search should match partial words too");
        
        // Test 4: Word matching with inversion
        let config_inverted = SearchConfig {
            case_insensitive: false,
            invert_match: true,
        };
        
        let result_inverted = search(content, "cat", &config_inverted);
        assert_eq!(result_inverted.total_count, 6, "Inverted word match should return lines without whole word 'cat'");
    }
} 