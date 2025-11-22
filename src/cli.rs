use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum Shell {
    /// Bash shell completion
    Bash,
    /// Fish shell completion
    Fish,
    /// Zsh shell completion
    Zsh,
    /// PowerShell completion
    #[value(name = "powershell")]
    PowerShell,
    /// Elvish shell completion
    Elvish,
    /// Nushell completion
    Nushell,
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Parse help or manpage texts and generate shell completion scripts",
    long_about = "hcl extracts CLI options from help text and exports them as shell completion scripts or JSON.",
)]
pub struct Cli {
    /// Extract CLI options from the help texts or man pages associated with the command
    #[arg(long, short = 'c', conflicts_with_all = ["file", "subcommand", "loadjson"])]
    pub command: Option<String>,

    /// Extract CLI options from a file
    #[arg(long, short = 'f', conflicts_with_all = ["command", "subcommand", "loadjson"])]
    pub file: Option<String>,

    /// Extract CLI options from a subcommand (format: command-subcommand, e.g., git-log)
    #[arg(long, short = 's', conflicts_with_all = ["command", "file", "loadjson"])]
    pub subcommand: Option<String>,

    /// Load JSON file in Command schema
    #[arg(long, conflicts_with_all = ["command", "file", "subcommand"])]
    pub loadjson: Option<String>,

    /// Output format: bash, zsh, fish, json, native, elvish, nushell
    #[arg(long, value_parser = ["bash", "zsh", "fish", "json", "native", "elvish", "nushell"], default_value = "native")]
    pub format: String,

    /// Output in JSON (same as --format=json)
    #[arg(long)]
    pub json: bool,

    /// Skip scanning manpage and focus on help text
    #[arg(long)]
    pub skip_man: bool,

    /// List subcommands (debug)
    #[arg(long, conflicts_with = "loadjson")]
    pub list_subcommands: bool,

    /// Run preprocessing only (debug)
    #[arg(long, conflicts_with = "loadjson")]
    pub debug: bool,

    /// Set upper bound of the depth of subcommand level
    #[arg(long, default_value = "4")]
    pub depth: usize,

    /// Generate shell completions
    #[arg(long, value_name = "SHELL")]
    pub completions: Option<Shell>,

    /// Write completion script to RC file (~/.bashrc, ~/.zshrc, etc.)
    /// Automatically detects shell and appends to appropriate rc file
    #[arg(long)]
    pub write: bool,

    /// Use bash-completion extended format for bash output
    /// (encodes descriptions as name:Description and calls __ltrim_colon_completions if available)
    #[arg(long)]
    pub bash_completion_compat: bool,
}

impl Cli {
    /// Get the effective format, considering --json flag as legacy
    pub fn effective_format(&self) -> &str {
        if self.json { "json" } else { &self.format }
    }

    /// Get the input file/command, prioritizing loadjson
    pub fn get_input(&self) -> Option<&str> {
        self.loadjson
            .as_deref()
            .or(self.file.as_deref())
            .or(self.command.as_deref())
    }

    /// Check if preprocess only mode (renamed from debug for clarity)
    pub fn is_preprocess_only(&self) -> bool {
        self.debug
    }
}
