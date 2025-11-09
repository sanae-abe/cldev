//! Sanitizer for sensitive information
//!
//! Automatically detects and redacts sensitive information (API keys, passwords, tokens, etc.)
//! from text to prevent accidental exposure in learning records.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SECRET_PATTERNS: Vec<(Regex, &'static str)> = vec![
        // API Keys
        (
            Regex::new(r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['\x22]?([a-zA-Z0-9_\-]{20,})['\x22]?").unwrap(),
            "$1: [REDACTED_API_KEY]"
        ),
        // Passwords
        (
            Regex::new(r"(?i)(password|passwd|pwd)\s*[:=]\s*['\x22]?([^\s'\x22]{8,})['\x22]?").unwrap(),
            "$1: [REDACTED_PASSWORD]"
        ),
        // Bearer Tokens
        (
            Regex::new(r"(?i)(bearer|token)\s+([a-zA-Z0-9_\-\.]{20,})").unwrap(),
            "$1 [REDACTED_TOKEN]"
        ),
        // Email addresses
        (
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
            "[REDACTED_EMAIL]"
        ),
        // IPv4 addresses
        (
            Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap(),
            "[REDACTED_IP]"
        ),
        // AWS Access Key
        (
            Regex::new(r"(?i)(AKIA[0-9A-Z]{16})").unwrap(),
            "[REDACTED_AWS_KEY]"
        ),
        // GitHub/GitLab tokens (ghp_, glpat-)
        (
            Regex::new(r"\b(ghp_[a-zA-Z0-9]{36}|glpat-[a-zA-Z0-9_\-]{20,})\b").unwrap(),
            "[REDACTED_GIT_TOKEN]"
        ),
    ];
}

/// Result of sanitization operation
#[derive(Debug, Clone)]
pub struct SanitizationResult {
    pub sanitized: String,
    pub patterns_found: Vec<String>,
}

/// Sanitize text by redacting sensitive information
pub fn sanitize_text(text: &str) -> SanitizationResult {
    let mut sanitized = text.to_string();
    let mut patterns_found = Vec::new();

    for (pattern, replacement) in SECRET_PATTERNS.iter() {
        if pattern.is_match(&sanitized) {
            patterns_found.push(replacement.to_string());
            sanitized = pattern.replace_all(&sanitized, *replacement).to_string();
        }
    }

    SanitizationResult {
        sanitized,
        patterns_found,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_api_key() {
        let text = "api_key: sk-1234567890abcdefghij";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_API_KEY]"));
        assert!(!result.sanitized.contains("sk-1234567890"));
        assert_eq!(result.patterns_found.len(), 1);
    }

    #[test]
    fn test_sanitize_password() {
        let text = "password = MySecretPass123";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_PASSWORD]"));
        assert!(!result.sanitized.contains("MySecretPass123"));
    }

    #[test]
    fn test_sanitize_bearer_token() {
        let text = "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_TOKEN]"));
        assert!(!result
            .sanitized
            .contains("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"));
    }

    #[test]
    fn test_sanitize_email() {
        let text = "Contact: user@example.com for support";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_EMAIL]"));
        assert!(!result.sanitized.contains("user@example.com"));
    }

    #[test]
    fn test_sanitize_ip_address() {
        let text = "Server IP: 192.168.1.100";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_IP]"));
        assert!(!result.sanitized.contains("192.168.1.100"));
    }

    #[test]
    fn test_sanitize_aws_key() {
        let text = "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_AWS_KEY]"));
        assert!(!result.sanitized.contains("AKIAIOSFODNN7EXAMPLE"));
    }

    #[test]
    fn test_sanitize_github_token() {
        let text = "GITHUB_TOKEN=ghp_1234567890abcdefghijklmnopqrstuvwxyz";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_GIT_TOKEN]"));
        assert!(!result.sanitized.contains("ghp_1234567890"));
    }

    #[test]
    fn test_sanitize_multiple_patterns() {
        let text = "api_key: secret12345678901234567890 and email: test@example.com";
        let result = sanitize_text(text);
        assert!(result.sanitized.contains("[REDACTED_API_KEY]"));
        assert!(result.sanitized.contains("[REDACTED_EMAIL]"));
        assert_eq!(result.patterns_found.len(), 2);
    }

    #[test]
    fn test_no_sensitive_data() {
        let text = "This is a normal log message with no secrets";
        let result = sanitize_text(text);
        assert_eq!(result.sanitized, text);
        assert_eq!(result.patterns_found.len(), 0);
    }
}
