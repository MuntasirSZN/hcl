# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in hcl, please **do not** open a public GitHub issue.

Instead, please email security details to:

📧 **muntasir.joypurhat@gmail.com**

### What to Include

Please include:

- Description of the vulnerability
- Steps to reproduce (if applicable)
- Potential impact
- Suggested fix (if you have one)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix & Release**: Within 2 weeks (or timeline discussed with reporter)
- **Public Disclosure**: After fix is released

### Process

1. Your report is received and acknowledged
1. We assess the severity and impact
1. We work on a fix in a private branch
1. We verify the fix resolves the issue
1. We release a patched version
1. We publish a security advisory
1. We credit you in the advisory (if desired)

## Security Best Practices

When using hcl:

1. **Keep Updated**: Regularly update to the latest version

   ```bash
   cargo install --upgrade hcl
   ```

1. **Input Validation**: Be cautious with untrusted help text

   - hcl processes help text locally
   - No data is sent to external servers

1. **File Permissions**: Protect generated completion scripts

   ```bash
   chmod 600 ~/.local/share/fish/vendor_completions.d/hcl.fish
   ```

1. **Command Execution**: Review commands before installation

   - hcl only reads command help text
   - It doesn't execute commands without explicit user action

## Supported Versions

| Version | Supported | Notes |
|---------|-------------------|------------------------|
| 0.1.x | ✅ Latest | Current release |
| < 0.1.0 | ❌ Not supported | Please upgrade |

## Security Considerations

### What hcl Does

- ✅ Reads local files
- ✅ Executes `command --help` with user permissions
- ✅ Executes `man command` with user permissions
- ✅ Parses help text locally
- ✅ Generates completion scripts
- ✅ Exports JSON output

### What hcl Does NOT Do

- ❌ Make network requests
- ❌ Write outside specified locations
- ❌ Require elevated privileges
- ❌ Store any data
- ❌ Phone home or track usage
- ❌ Execute arbitrary commands from help text

### Safe Usage

hcl is designed to be used safely:

```bash
# Safe - reads local man page
hcl --command ls --format fish

# Safe - reads from file you control
hcl --file my-help.txt --format json

# Safe - reads from JSON you control
hcl --json my-commands.json --format bash

# Caution - only use for commands you trust
hcl --command suspicious-command --format json
```

## Dependencies

We keep dependencies minimal and regularly audited:

```bash
# Check for known vulnerabilities
cargo audit

# Update dependencies
cargo upgrade
```

Current core dependencies:

- `clap` - CLI parsing (maintained, frequently updated)
- `serde` & `serde_json` - Serialization (well-maintained)
- `regex` - Pattern matching (audited)
- `lazy_static` - Static initialization (minimal, mature)
- `anyhow` - Error handling (lightweight, stable)

## Build Security

- Builds use Rust's memory safety guarantees
- No unsafe code in core functionality
- Release builds use LTO and optimization
- Artifacts are reproducible (with same Rust version)

## Disclosure Examples

Examples of reportable security issues:

- ✅ Arbitrary file read via path traversal
- ✅ Command injection in shell generators
- ✅ Denial of service via malformed input
- ✅ Memory safety issues (unsafe code)
- ✅ Dependency vulnerabilities
- ✅ Privilege escalation paths

Examples of non-security issues:

- ❌ Performance problems
- ❌ Parsing failures on edge cases
- ❌ Missing features
- ❌ UI/UX concerns

## Acknowledgments

We appreciate responsible disclosure and will acknowledge:

- Security researchers who report vulnerabilities
- Contributors who fix security issues
- Community members who help improve security

______________________________________________________________________

**Last Updated**: 2025
**Version**: 0.1.0
