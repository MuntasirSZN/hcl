use hcl::types::OptNameType;
use hcl::{Command, Opt, OptName, Parser, ZshGenerator};

#[test]
fn test_parse_ls_help_snapshot() {
    let ls_help = r#"
OPTIONS:
  -a, --all                 do not ignore entries starting with .
  -A, --almost-all          do not list implied . and ..
  -b, --escape              print C-style escapes for nongraphic characters
"#;

    let opts = Parser::parse_line(ls_help);
    insta::assert_yaml_snapshot!(opts.len());
}

#[test]
fn test_zsh_generator_with_descriptions_snapshot() {
    let cmd = Command {
        name: "test".to_string(),
        description: "Test command".to_string(),
        usage: "test [OPTIONS]".to_string(),
        options: vec![Opt {
            names: vec![
                OptName::new("-v".to_string(), OptNameType::ShortType),
                OptName::new("--verbose".to_string(), OptNameType::LongType),
            ],
            argument: String::new(),
            description: "Enable verbose mode".to_string(),
        }],
        subcommands: vec![],
        version: String::new(),
    };

    let output = ZshGenerator::generate(&cmd);
    insta::assert_snapshot!(output);
}

#[test]
fn test_parse_docker_help_snapshot() {
    let docker_help = r#"
Options:
  -d, --detach              Run container in background
  --name string             Assign a name to the container
  -p, --publish list        Publish a container's port(s) to the host
"#;

    let opts = Parser::parse_line(docker_help);
    insta::assert_yaml_snapshot!(opts.len());
}
