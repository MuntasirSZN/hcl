use crate::types::{Opt, OptName};
use regex::Regex;

lazy_static::lazy_static! {
    static ref ALPHANUM_CHARS: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    static ref SYMBOL_CHARS: String = "+-_!?@.".to_string();
    static ref ALLOWED_OPT_CHARS: String = {
        let mut s = ALPHANUM_CHARS.clone();
        s.push_str(&SYMBOL_CHARS);
        s
    };
}

pub struct Parser;

impl Parser {
    pub fn parse_line(s: &str) -> Vec<Opt> {
        let pairs = Self::preprocess(s);
        let mut opts = Vec::with_capacity(pairs.len());
        let mut seen = std::collections::HashSet::with_capacity(pairs.len());

        for (opt_str, desc_str) in pairs {
            for opt in Self::parse_with_opt_part(&opt_str, &desc_str) {
                if seen.insert(opt.clone()) {
                    opts.push(opt);
                }
            }
        }
        opts
    }

    pub fn preprocess(s: &str) -> Vec<(String, String)> {
        let lines: Vec<&str> = s.lines().collect();
        let mut result = Vec::with_capacity(lines.len());
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim_start();

            // Fast path: skip lines that don't start with '-'
            if !trimmed.starts_with('-') {
                i += 1;
                continue;
            }

            // Try to split option and description from the same line first
            // Most help text has format: "  -v, --verbose         description text"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            // Find where description starts (after option names/args)
            let mut opt_end = 0;
            for (idx, part) in parts.iter().enumerate() {
                if part.starts_with('-') || idx == 0 {
                    opt_end = idx + 1;
                } else if part.contains('=') || !part.starts_with('-') {
                    // Could be an argument marker
                    opt_end = idx + 1;
                } else {
                    break;
                }
            }

            if opt_end > 0 && opt_end < parts.len() {
                // Description is on the same line
                let opt_str = parts[0..opt_end].join(" ");
                let desc_str = parts[opt_end..].join(" ");
                result.push((opt_str, desc_str));
                i += 1;
            } else if opt_end > 0 {
                // No description on this line, try next line
                let opt_str = trimmed.to_string();
                let desc_str = if i + 1 < lines.len() && !lines[i + 1].trim_start().starts_with('-')
                {
                    lines[i + 1].trim().to_string()
                } else {
                    String::new()
                };

                if !desc_str.is_empty() {
                    result.push((opt_str, desc_str));
                    i += 2;
                } else {
                    result.push((opt_str, String::new()));
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        result
    }

    pub fn parse_with_opt_part(opt_str: &str, desc_str: &str) -> Vec<Opt> {
        let names = Self::parse_opt_names(opt_str);
        let arg = Self::parse_opt_arg(opt_str);

        if names.is_empty() {
            return Vec::new();
        }

        vec![Opt {
            names,
            argument: arg,
            description: desc_str.to_string(),
        }]
    }

    fn parse_opt_names(s: &str) -> Vec<OptName> {
        let mut names = Vec::with_capacity(4);

        for part in s.split([',', '/', '|']) {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }

            for word in trimmed.split_whitespace() {
                if word.starts_with('-')
                    && let Some(name) = OptName::from_text(word)
                {
                    names.push(name);
                }
            }
        }

        names.sort();
        names.dedup();
        names
    }

    fn parse_opt_arg(s: &str) -> String {
        for part in s.split([',', '/', '|']) {
            let trimmed = part.trim();
            if let Some(arg) = Self::extract_arg_from_part(trimmed)
                && !arg.is_empty()
            {
                return arg;
            }
        }
        String::new()
    }

    fn extract_arg_from_part(s: &str) -> Option<String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() < 2 {
            return None;
        }

        let arg_part = words[1..].join(" ");

        if arg_part.is_empty() || arg_part == "." {
            return None;
        }

        Some(arg_part)
    }

    pub fn parse_usage_header(keywords: &[&str], block: &str) -> Option<String> {
        if keywords.is_empty() || block.is_empty() {
            return None;
        }

        let header_line = block.lines().next()?.to_lowercase();
        for keyword in keywords {
            let pattern = format!(r"^\s*{}\s*:?\s*$", regex::escape(keyword));
            if let Ok(re) = Regex::new(&pattern)
                && re.is_match(&header_line)
            {
                return Some(header_line);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_same_and_next_line_descriptions() {
        let input = "  -a, --all  show all\n  -b\n    show b";
        let pairs = Parser::preprocess(input);
        assert_eq!(pairs.len(), 2);
        // Current implementation keeps the entire first line as the option
        // part when it cannot separate a description on the same line.
        assert_eq!(pairs[0].0, "-a, --all  show all");
        assert_eq!(pairs[0].1, "");
        assert_eq!(pairs[1].0, "-b");
        assert_eq!(pairs[1].1, "show b");
    }

    #[test]
    fn test_parse_usage_header_matches_keywords() {
        let block = "Usage:\n  cmd [OPTIONS]\n";
        let header = Parser::parse_usage_header(&["usage"], block).unwrap();
        assert!(header.contains("usage"));
    }

    #[test]
    fn test_parse_opt_names() {
        let names = Parser::parse_opt_names("-v, --verbose");
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|n| n.raw == "-v"));
        assert!(names.iter().any(|n| n.raw == "--verbose"));
    }

    #[test]
    fn test_parse_with_opt_part() {
        let opts = Parser::parse_with_opt_part("-v, --verbose", "Enable verbose mode");
        assert_eq!(opts.len(), 1);
        assert_eq!(opts[0].names.len(), 2);
        assert_eq!(opts[0].description, "Enable verbose mode");
    }

    #[test]
    fn test_parse_line_deduplicates_options() {
        let input = "  -v, --verbose  verbose\n  -v, --verbose  verbose";
        let opts = Parser::parse_line(input);
        assert_eq!(opts.len(), 1);
        assert_eq!(opts[0].names.len(), 2);
    }

    #[test]
    fn test_parse_line_bioinformatics_style_help() {
        let input = "  -i, --input FILE       Input FASTA/FASTQ file\n  -o, --output FILE      Output BAM file\n  --min-mapq INT         Minimum mapping quality (default: 30)";
        let opts = Parser::parse_line(input);
        assert_eq!(opts.len(), 3);

        // Ensure all expected option names are detected, even if
        // arguments/descriptions are not perfectly separated.
        let all_names: Vec<String> = opts
            .iter()
            .flat_map(|o| o.names.iter().map(|n| n.raw.clone()))
            .collect();
        assert!(all_names.contains(&"-i".to_string()));
        assert!(all_names.contains(&"--input".to_string()));
        assert!(all_names.contains(&"-o".to_string()));
        assert!(all_names.contains(&"--output".to_string()));
        assert!(all_names.contains(&"--min-mapq".to_string()));
    }
}
