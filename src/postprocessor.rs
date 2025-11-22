use crate::types::{Command, Opt};
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
        let mut result = Vec::new();

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
        text.lines()
            .map(|line| {
                let trimmed = line.trim_start();
                let mut chars = trimmed.chars();
                let first = chars.next();

                if (first == Some('•') || first == Some('*') || first == Some('-'))
                    && let Some(second) = chars.next()
                    && second.is_whitespace()
                {
                    let prefix = &line[..line.len() - trimmed.len()];
                    let content = chars.as_str().trim_start();
                    return format!("{}{}", prefix, content);
                }
                line.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn unicode_spaces_to_ascii(text: &str) -> String {
        text.replace('\u{00A0}', " ")
            .replace('\u{2003}', "   ")
            .replace('\u{2002}', "  ")
    }

    pub fn convert_tabs_to_spaces(text: &str, spaces: usize) -> String {
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
