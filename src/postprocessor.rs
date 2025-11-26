use crate::types::{Command, Opt};
use memchr::memmem;
use std::collections::HashSet;

pub struct Postprocessor;

impl Postprocessor {
    pub fn fix_command(mut cmd: Command) -> Command {
        cmd.options = Self::deduplicate_options(cmd.options);
        cmd.options = Self::filter_invalid_options(cmd.options);
        cmd.subcommands = cmd.subcommands.into_iter().map(Self::fix_command).collect();

        cmd
    }

    fn deduplicate_options(options: Vec<Opt>) -> Vec<Opt> {
        let mut seen = HashSet::new();
        let mut result = Vec::with_capacity(options.len());

        for opt in options {
            let key = (
                opt.names
                    .iter()
                    .map(|n| n.raw.clone())
                    .collect::<Vec<_>>()
                    .join("|"),
                opt.argument.clone(),
            );

            if !seen.contains(&key) {
                seen.insert(key);
                result.push(opt);
            }
        }

        result
    }

    fn filter_invalid_options(options: Vec<Opt>) -> Vec<Opt> {
        options
            .into_iter()
            .filter(|opt| {
                !opt.names.is_empty() && !opt.names[0].raw.is_empty() && !opt.description.is_empty()
            })
            .collect()
    }

    pub fn remove_bullets(text: &str) -> String {
        // Pre-allocate with capacity hint
        let mut result = String::with_capacity(text.len());
        let mut first = true;

        for line in text.lines() {
            if !first {
                result.push('\n');
            }
            first = false;

            let trimmed = line.trim_start();
            let prefix_len = line.len() - trimmed.len();
            let bytes = trimmed.as_bytes();

            // Fast path: check first byte for bullet characters
            if bytes.len() >= 2 {
                let is_bullet = match bytes[0] {
                    b'*' | b'-' => bytes[1].is_ascii_whitespace(),
                    // UTF-8 bullet point (•) starts with 0xE2
                    0xE2 if bytes.len() >= 4 && bytes[1] == 0x80 && bytes[2] == 0xA2 => {
                        bytes[3].is_ascii_whitespace()
                    }
                    _ => false,
                };

                if is_bullet {
                    result.push_str(&line[..prefix_len]);
                    // Skip bullet and whitespace
                    let skip = if bytes[0] == 0xE2 { 4 } else { 2 };
                    result.push_str(trimmed[skip..].trim_start());
                    continue;
                }
            }
            result.push_str(line);
        }

        result
    }

    pub fn unicode_spaces_to_ascii(text: &str) -> String {
        // Fast path: if no unicode spaces found, return borrowed string
        let nbsp = "\u{00A0}";
        let em_space = "\u{2003}";
        let en_space = "\u{2002}";

        let has_nbsp = memmem::find(text.as_bytes(), nbsp.as_bytes()).is_some();
        let has_em = memmem::find(text.as_bytes(), em_space.as_bytes()).is_some();
        let has_en = memmem::find(text.as_bytes(), en_space.as_bytes()).is_some();

        if !has_nbsp && !has_em && !has_en {
            return text.to_string();
        }

        // Pre-allocate result
        let mut result = String::with_capacity(text.len());

        for c in text.chars() {
            match c {
                '\u{00A0}' => result.push(' '),
                '\u{2003}' => result.push_str("   "),
                '\u{2002}' => result.push_str("  "),
                _ => result.push(c),
            }
        }

        result
    }

    pub fn convert_tabs_to_spaces(text: &str, spaces: usize) -> String {
        // Fast path: no tabs
        if !text.contains('\t') {
            return text.to_string();
        }
        text.replace('\t', &" ".repeat(spaces))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OptName;
    use crate::types::OptNameType;

    #[test]
    fn test_deduplicate_options() {
        let opts = vec![
            Opt {
                names: vec![OptName::new("-v".to_string(), OptNameType::ShortType)],
                argument: String::new(),
                description: "verbose".to_string(),
            },
            Opt {
                names: vec![OptName::new("-v".to_string(), OptNameType::ShortType)],
                argument: String::new(),
                description: "verbose".to_string(),
            },
        ];

        let result = Postprocessor::deduplicate_options(opts);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_remove_bullets() {
        let text = "• Item one\n* Item two\n- Item three";
        let result = Postprocessor::remove_bullets(text);
        assert!(!result.contains("•"));
    }

    #[test]
    fn test_unicode_and_tabs_helpers() {
        // Text with various unicode spaces and a tab
        let text = "\u{00A0}foo\u{2002}bar\u{2003}baz\tend";
        let ascii = Postprocessor::unicode_spaces_to_ascii(text);

        // Non-breaking/en-space/em-space should be replaced with ASCII spaces
        assert_eq!(ascii, " foo  bar   baz\tend");

        let with_spaces = Postprocessor::convert_tabs_to_spaces(&ascii, 4);
        assert!(!with_spaces.contains('\t'));
        assert!(with_spaces.ends_with("    end"));
    }

    #[test]
    fn test_fix_command_filters_and_deduplicates() {
        let valid_opt = Opt {
            names: vec![OptName::new("-v".to_string(), OptNameType::ShortType)],
            argument: String::new(),
            description: "verbose".to_string(),
        };

        let invalid_opt = Opt {
            names: vec![],
            argument: String::new(),
            description: String::new(),
        };

        let cmd = Command {
            name: "root".to_string(),
            description: String::new(),
            usage: String::new(),
            options: vec![valid_opt.clone(), valid_opt.clone(), invalid_opt],
            subcommands: vec![Command {
                name: "child".to_string(),
                description: String::new(),
                usage: String::new(),
                options: vec![valid_opt.clone()],
                subcommands: vec![],
                version: String::new(),
            }],
            version: String::new(),
        };

        let fixed = Postprocessor::fix_command(cmd);
        assert_eq!(fixed.options.len(), 1);
        assert_eq!(fixed.subcommands.len(), 1);
        assert_eq!(fixed.subcommands[0].options.len(), 1);
    }
}
