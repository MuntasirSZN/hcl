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
}
