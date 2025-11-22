use anyhow::{Result, anyhow};
use std::fs;
use std::process::Command as StdCommand;

pub struct IoHandler;

impl IoHandler {
    pub fn read_file(path: &str) -> Result<String> {
        fs::read_to_string(path).map_err(|e| anyhow!("Failed to read file {}: {}", path, e))
    }

    pub fn read_from_command(cmd: &str) -> Result<String> {
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("Command failed: {}", cmd));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_command_help(cmd: &str) -> Result<String> {
        Self::read_from_command(&format!("{} --help 2>/dev/null || {}", cmd, cmd))
    }

    pub fn get_manpage(cmd: &str) -> Result<String> {
        Self::read_from_command(&format!("man {} 2>/dev/null | col -bx", cmd))
    }

    pub fn normalize_text(text: &str) -> String {
        text.lines()
            .map(|line| line.replace('\t', "        ").replace("  ", " "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn is_man_available(cmd: &str) -> bool {
        StdCommand::new("man")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        let input = "hello\t\tworld";
        let output = IoHandler::normalize_text(input);
        assert!(!output.contains('\t'));
    }

    #[test]
    fn test_read_file_and_error() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().expect("create temp file");
        write!(file, "hello world").expect("write temp file");
        let path = file.path().to_str().unwrap();

        let content = IoHandler::read_file(path).expect("read temp file");
        assert_eq!(content, "hello world");

        let missing = IoHandler::read_file("/this/does/not/exist");
        assert!(missing.is_err());
    }

    #[test]
    fn test_read_from_command_success_and_failure() {
        let out = IoHandler::read_from_command("echo hello").expect("run echo");
        assert!(out.contains("hello"));

        let res = IoHandler::read_from_command("exit 1");
        assert!(res.is_err());
    }

    #[test]
    fn test_get_command_help_and_manpage_and_is_man_available() {
        let help = IoHandler::get_command_help("echo").expect("get help");
        assert!(!help.is_empty());

        let _man_available = IoHandler::is_man_available("echo");

        if _man_available {
            let man = IoHandler::get_manpage("echo").expect("get manpage");
            assert!(!man.is_empty());
        }
    }
}
