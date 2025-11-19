use crate::types::Subcommand;
use std::collections::BTreeSet;

pub struct SubcommandParser;

impl SubcommandParser {
    pub fn parse(content: &str) -> Vec<Subcommand> {
        let lines: Vec<&str> = content.lines().collect();
        let mut subcommands = BTreeSet::new();

        for window in lines.windows(2) {
            if let Some(subcommand) = Self::parse_line_pair(window[0], window[1]) {
                subcommands.insert(subcommand);
            }
        }

        for line in &lines {
            if let Some(subcommand) = Self::parse_single_line(line) {
                subcommands.insert(subcommand);
            }
        }

        subcommands.into_iter().collect()
    }

    fn parse_line_pair(first: &str, second: &str) -> Option<Subcommand> {
        let trimmed_first = first.trim();

        if trimmed_first.is_empty() || trimmed_first.starts_with('-') {
            return None;
        }

        let first_word = trimmed_first.split_whitespace().next()?;

        if !Self::is_valid_subcommand_name(first_word) {
            return None;
        }

        let desc = second.trim().to_string();

        if desc.is_empty() || desc.starts_with('-') {
            return None;
        }

        Some(Subcommand {
            cmd: first_word.to_string(),
            desc: desc.split('\n').next().unwrap_or("").to_string(),
        })
    }

    fn parse_single_line(line: &str) -> Option<Subcommand> {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('-') {
            return None;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let name = parts[0];
        if !Self::is_valid_subcommand_name(name) {
            return None;
        }

        let desc = parts[1..].join(" ");

        if desc.is_empty() {
            return None;
        }

        Some(Subcommand {
            cmd: name.to_string(),
            desc,
        })
    }

    fn is_valid_subcommand_name(name: &str) -> bool {
        if name.starts_with('-') || name.is_empty() {
            return false;
        }

        name.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subcommands() {
        let content = "run       Run a command\nbuild     Build a project";
        let subs = SubcommandParser::parse(content);
        assert!(subs.iter().any(|s| s.cmd == "run"));
        assert!(subs.iter().any(|s| s.cmd == "build"));
    }

    #[test]
    fn test_is_valid_subcommand_name() {
        assert!(SubcommandParser::is_valid_subcommand_name("run"));
        assert!(SubcommandParser::is_valid_subcommand_name("sub-cmd"));
        assert!(!SubcommandParser::is_valid_subcommand_name("-v"));
        assert!(!SubcommandParser::is_valid_subcommand_name(""));
    }
}
