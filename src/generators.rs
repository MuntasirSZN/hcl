use crate::types::{Command, Opt, OptName, OptNameType};
use std::collections::BTreeSet;

pub struct FishGenerator;

impl FishGenerator {
    pub fn generate(cmd: &Command) -> String {
        let mut lines = Vec::new();
        Self::generate_rec(&mut lines, &[], cmd);
        lines.join("\n")
    }

    fn generate_rec(lines: &mut Vec<String>, path: &[&str], cmd: &Command) {
        let mut current_path = path.to_vec();
        current_path.push(&cmd.name);

        for opt in &cmd.options {
            for name in &opt.names {
                if !Self::should_skip_option(name) {
                    lines.push(Self::make_option_line(&current_path, name, opt));
                }
            }
        }

        for subcmd in &cmd.subcommands {
            Self::generate_rec(lines, &current_path, subcmd);
        }
    }

    fn should_skip_option(name: &OptName) -> bool {
        matches!(
            name.opt_type,
            OptNameType::SingleDashAlone | OptNameType::DoubleDashAlone
        )
    }

    fn make_option_line(path: &[&str], name: &OptName, opt: &Opt) -> String {
        let dashless = name.raw.trim_start_matches('-');
        let quoted = format!("'{}'", dashless);
        let flag = Self::opt_type_to_flag(name.opt_type);
        let arg_flag = Self::opt_arg_to_flag(opt);
        let desc = Self::truncate_after_period(&opt.description);

        format!(
            "complete -c {} {} {} {} -d '{}'",
            path.join("_"),
            flag,
            quoted,
            arg_flag,
            desc.replace('\'', "\\'")
        )
    }

    fn opt_type_to_flag(opt_type: OptNameType) -> String {
        match opt_type {
            OptNameType::LongType => "-l".to_string(),
            OptNameType::ShortType => "-s".to_string(),
            OptNameType::OldType => "-o".to_string(),
            _ => String::new(),
        }
    }

    fn opt_arg_to_flag(opt: &Opt) -> &'static str {
        let arg_lower = opt.argument.to_lowercase();
        let desc_lower = opt.description.to_lowercase();

        if opt.argument.is_empty() {
            ""
        } else if arg_lower.contains("file")
            || arg_lower.contains("dir")
            || arg_lower.contains("path")
            || arg_lower.contains("archive")
            || desc_lower.contains("file")
            || desc_lower.contains("dir")
            || desc_lower.contains("path")
        {
            "-r"
        } else {
            "-x"
        }
    }

    fn truncate_after_period(line: &str) -> String {
        line.split('.').next().unwrap_or("").to_string()
    }
}

pub struct ZshGenerator;

impl ZshGenerator {
    pub fn generate(cmd: &Command) -> String {
        let mut lines = vec![
            "#compdef _hcl hcl".to_string(),
            "".to_string(),
            "_hcl() {".to_string(),
            "  local -a options".to_string(),
            "".to_string(),
        ];

        for opt in &cmd.options {
            let opt_lines = Self::generate_opt(opt);
            lines.extend(opt_lines);
        }

        lines.push("  _arguments -s -S $options".to_string());
        lines.push("}".to_string());
        lines.push("".to_string());
        lines.push("_hcl \"$@\"".to_string());

        lines.join("\n")
    }

    fn generate_opt(opt: &Opt) -> Vec<String> {
        let mut lines = Vec::new();
        let desc = Self::truncate_after_period(&opt.description);

        for name in &opt.names {
            if matches!(
                name.opt_type,
                OptNameType::SingleDashAlone | OptNameType::DoubleDashAlone
            ) {
                continue;
            }

            let spec = if opt.argument.is_empty() {
                format!("'{}[{}]'", name.raw, desc)
            } else {
                format!("'{}[{} {}]'", name.raw, opt.argument, desc)
            };

            lines.push(format!("  options+=({})", spec));
        }

        lines
    }

    fn truncate_after_period(line: &str) -> String {
        line.split('.').next().unwrap_or("").to_string()
    }
}

pub struct BashGenerator;

impl BashGenerator {
    pub fn generate(cmd: &Command) -> String {
        let mut lines = vec![
            format!("_hcl_{}()", cmd.name),
            "{".to_string(),
            "  local cur prev opts".to_string(),
            "  COMPREPLY=()".to_string(),
            "  cur=\"${COMP_WORDS[COMP_CWORD]}\"".to_string(),
            "  prev=\"${COMP_WORDS[COMP_CWORD-1]}\"".to_string(),
            "".to_string(),
        ];

        let all_opts: Vec<String> = cmd
            .options
            .iter()
            .flat_map(|opt| {
                opt.names
                    .iter()
                    .filter_map(|name| {
                        if !matches!(
                            name.opt_type,
                            OptNameType::SingleDashAlone | OptNameType::DoubleDashAlone
                        ) {
                            Some(name.raw.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        lines.push(format!("  opts=\"{}\"", all_opts.join(" ")));
        lines.push("".to_string());
        lines.push("  COMPREPLY=($(compgen -W \"${opts}\" -- ${cur}))".to_string());
        lines.push("}".to_string());
        lines.push("".to_string());
        lines.push(format!(
            "complete -o bashdefault -o default -o nospace -F _hcl_{} {}",
            cmd.name, cmd.name
        ));

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_after_period() {
        let text = "This is a description. With more text.";
        assert_eq!(
            FishGenerator::truncate_after_period(text),
            "This is a description"
        );
    }
}
