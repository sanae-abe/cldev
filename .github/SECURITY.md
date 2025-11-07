# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

### GitHub Security Advisories (Preferred)

1. Go to https://github.com/sanae-abe/cldev/security/advisories/new
2. Fill out the security advisory form
3. Click "Submit report"

### Email

Send an email to: security@cldev.dev (or sanae.abe@example.com)

Please include:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: 30-90 days

## Security Update Process

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
2. **Assessment**: We will assess the vulnerability and determine its impact
3. **Fix Development**: We will develop a fix
4. **Testing**: We will test the fix thoroughly
5. **Release**: We will release a security patch
6. **Disclosure**: We will publicly disclose the vulnerability after the patch is released

## Public Disclosure

We follow responsible disclosure practices:

- We will coordinate with you on the disclosure timeline
- We will publicly acknowledge your contribution (unless you prefer to remain anonymous)
- We will publish a security advisory after the fix is released

## Security Best Practices for Users

### Installation

```bash
# Always verify checksums
sha256sum cldev-linux-x86_64.tar.gz

# Use official sources
cargo install cldev
# or download from GitHub releases
```

### Configuration

```bash
# Protect your configuration file
chmod 600 ~/.config/cldev/config.toml

# Never commit credentials
# Add to .gitignore:
.env
*.key
*.pem
credentials.json
```

### Updates

```bash
# Keep cldev updated
cargo install cldev --force

# Check for security advisories
cargo audit
```

## Security Features

### Encryption

- AES-256-GCM encryption for sensitive data
- Secure key derivation using PBKDF2
- Random IV generation for each encryption operation

### Authentication

- Secure credential storage
- Token-based authentication
- Automatic token refresh

### Input Validation

- All user inputs are validated
- Path traversal protection
- Command injection prevention

### Dependencies

- Regular security audits using `cargo audit`
- Automated dependency updates via Dependabot
- License compliance checks

## Known Security Considerations

### Configuration File Security

Configuration files may contain sensitive information:

```toml
# ~/.config/cldev/config.toml
[security]
# Sensitive data is encrypted
encrypted_credentials = "..."
```

**Recommendation**: Set appropriate file permissions (600 or 400)

### Environment Variables

Some features may read environment variables:

```bash
# Be cautious with environment variables
export CLDEV_API_KEY="your-key-here"
```

**Recommendation**: Use encrypted credential storage instead

### External Tools

cldev integrates with external tools:

```bash
# Ensure external tools are from trusted sources
cldev tool install <tool-name>
```

**Recommendation**: Only install tools from official sources

## Security Audit History

| Date       | Auditor      | Scope           | Findings | Status   |
|------------|--------------|-----------------|----------|----------|
| 2025-11-07 | Internal     | Initial Release | 0        | Resolved |

## Compliance

cldev follows industry security standards:

- OWASP Top 10 mitigation
- CWE/SANS Top 25 awareness
- Secure coding practices
- Regular dependency audits

## Security Tools Used

- `cargo audit` - Dependency vulnerability scanning
- `cargo deny` - License and security policy enforcement
- `cargo geiger` - Unsafe code detection
- `cargo-outdated` - Dependency update tracking

## Contact

For security-related questions or concerns:

- Security Email: security@cldev.dev
- General Contact: sanae.abe@example.com
- GitHub Security: https://github.com/sanae-abe/cldev/security

## Credits

We thank the following security researchers for their contributions:

<!-- List will be updated as vulnerabilities are reported and fixed -->

---

**Last Updated**: 2025-11-07

**Generated with Claude Code**

Co-Authored-By: Claude <noreply@anthropic.com>
