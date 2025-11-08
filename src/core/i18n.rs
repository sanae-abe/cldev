//! Internationalization (i18n) support for cldev CLI
//!
//! Provides JSON-based multi-language support with automatic language detection
//! and message formatting with variable substitution.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt;

use super::error::{CldevError, Result};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English
    #[serde(rename = "en")]
    #[default]
    English,
    /// Japanese
    #[serde(rename = "ja")]
    Japanese,
    /// Chinese (Simplified)
    #[serde(rename = "zh")]
    Chinese,
}

impl Language {
    /// Detect language from environment variables
    pub fn detect() -> Self {
        // Check LANG environment variable
        if let Ok(lang) = env::var("LANG") {
            let lang_lower = lang.to_lowercase();
            if lang_lower.starts_with("ja") {
                return Language::Japanese;
            }
            if lang_lower.starts_with("zh") {
                return Language::Chinese;
            }
        }

        // Check LC_ALL as fallback
        if let Ok(lang) = env::var("LC_ALL") {
            let lang_lower = lang.to_lowercase();
            if lang_lower.starts_with("ja") {
                return Language::Japanese;
            }
            if lang_lower.starts_with("zh") {
                return Language::Chinese;
            }
        }

        // Default to English
        Language::English
    }

    /// Get language code as string
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Japanese => "ja",
            Language::Chinese => "zh",
        }
    }

    /// Parse language from string
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "en" | "english" => Some(Language::English),
            "ja" | "japanese" | "jp" => Some(Language::Japanese),
            "zh" | "chinese" | "zh-cn" | "zh-hans" => Some(Language::Chinese),
            _ => None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Message catalog containing translations for all languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCatalog(HashMap<String, HashMap<String, String>>);

impl MessageCatalog {
    /// Load message catalog from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        let messages: HashMap<String, HashMap<String, String>> = serde_json::from_str(json)
            .map_err(|e| CldevError::Config(format!("Failed to parse i18n messages: {}", e)))?;
        Ok(MessageCatalog(messages))
    }

    /// Get default message catalog (embedded in binary)
    pub fn new_default() -> Self {
        // Embedded default messages
        let json = include_str!("../i18n/messages.json");
        Self::from_json(json).expect("Default messages should be valid")
    }

    /// Get message for a specific language
    pub fn get(&self, key: &str, lang: Language) -> Option<&str> {
        self.0
            .get(lang.code())
            .and_then(|msgs| msgs.get(key))
            .map(|s| s.as_str())
    }

    /// Check if a message key exists
    pub fn has_key(&self, key: &str) -> bool {
        self.0.values().any(|msgs| msgs.contains_key(key))
    }

    /// Get all available languages
    pub fn languages(&self) -> Vec<Language> {
        self.0
            .keys()
            .filter_map(|code| Language::from_code(code))
            .collect()
    }
}

/// Internationalization handler
pub struct I18n {
    catalog: MessageCatalog,
    current_language: Language,
}

impl I18n {
    /// Create a new I18n instance with the default catalog
    pub fn new() -> Self {
        Self {
            catalog: MessageCatalog::new_default(),
            current_language: Language::detect(),
        }
    }

    /// Create I18n with a custom message catalog
    pub fn with_catalog(catalog: MessageCatalog) -> Self {
        Self {
            catalog,
            current_language: Language::detect(),
        }
    }

    /// Create I18n with a specific language
    pub fn with_language(language: Language) -> Self {
        Self {
            catalog: MessageCatalog::new_default(),
            current_language: language,
        }
    }

    /// Get the current language
    pub fn language(&self) -> Language {
        self.current_language
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get a message by key
    ///
    /// Returns the message in the current language, falling back to English
    /// if not found. If the key doesn't exist at all, returns the key itself.
    pub fn get(&self, key: &str) -> String {
        // Try current language first
        if let Some(msg) = self.catalog.get(key, self.current_language) {
            return msg.to_string();
        }

        // Fallback to English
        if self.current_language != Language::English {
            if let Some(msg) = self.catalog.get(key, Language::English) {
                return msg.to_string();
            }
        }

        // If nothing found, return the key itself as fallback
        key.to_string()
    }

    /// Get a message with variable substitution
    ///
    /// Variables are specified in the format `{variable_name}` and replaced
    /// with values from the provided HashMap.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use cldev::core::i18n::I18n;
    ///
    /// let i18n = I18n::new();
    /// let mut vars = HashMap::new();
    /// vars.insert("command", "cldev config check");
    /// let msg = i18n.get_with_vars("next-step", &vars);
    /// ```
    pub fn get_with_vars(&self, key: &str, vars: &HashMap<&str, &str>) -> String {
        let template = self.get(key);
        let mut result = template;

        for (var_name, var_value) in vars {
            let placeholder = format!("{{{}}}", var_name);
            result = result.replace(&placeholder, var_value);
        }

        result
    }

    /// Format a message with a single variable
    pub fn format(&self, key: &str, var_name: &str, var_value: &str) -> String {
        let mut vars = HashMap::new();
        vars.insert(var_name, var_value);
        self.get_with_vars(key, &vars)
    }

    /// Get all available languages in the catalog
    pub fn available_languages(&self) -> Vec<Language> {
        self.catalog.languages()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_catalog() -> MessageCatalog {
        let json = r#"{
            "en": {
                "hello": "Hello",
                "greeting": "Hello, {name}!",
                "multi-var": "{a} and {b}"
            },
            "ja": {
                "hello": "こんにちは",
                "greeting": "こんにちは、{name}！"
            }
        }"#;
        MessageCatalog::from_json(json).unwrap()
    }

    #[test]
    fn test_language_detection() {
        // Language detection depends on environment, so just check it doesn't panic
        let lang = Language::detect();
        assert!(
            lang == Language::English || lang == Language::Japanese || lang == Language::Chinese
        );
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Japanese.code(), "ja");
        assert_eq!(Language::Chinese.code(), "zh");
    }

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("ja"), Some(Language::Japanese));
        assert_eq!(Language::from_code("japanese"), Some(Language::Japanese));
        assert_eq!(Language::from_code("zh"), Some(Language::Chinese));
        assert_eq!(Language::from_code("chinese"), Some(Language::Chinese));
        assert_eq!(Language::from_code("zh-cn"), Some(Language::Chinese));
        assert_eq!(Language::from_code("invalid"), None);
    }

    #[test]
    fn test_message_catalog_get() {
        let catalog = test_catalog();

        assert_eq!(catalog.get("hello", Language::English), Some("Hello"));
        assert_eq!(catalog.get("hello", Language::Japanese), Some("こんにちは"));
        assert_eq!(catalog.get("nonexistent", Language::English), None);
    }

    #[test]
    fn test_message_catalog_has_key() {
        let catalog = test_catalog();

        assert!(catalog.has_key("hello"));
        assert!(catalog.has_key("greeting"));
        assert!(!catalog.has_key("nonexistent"));
    }

    #[test]
    fn test_i18n_get() {
        let i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        assert_eq!(i18n.get("hello"), "Hello");
    }

    #[test]
    fn test_i18n_get_with_vars() {
        let i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        let mut vars = HashMap::new();
        vars.insert("name", "Alice");
        assert_eq!(i18n.get_with_vars("greeting", &vars), "Hello, Alice!");
    }

    #[test]
    fn test_i18n_format() {
        let i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        assert_eq!(i18n.format("greeting", "name", "Bob"), "Hello, Bob!");
    }

    #[test]
    fn test_i18n_multiple_vars() {
        let i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        let mut vars = HashMap::new();
        vars.insert("a", "first");
        vars.insert("b", "second");
        assert_eq!(i18n.get_with_vars("multi-var", &vars), "first and second");
    }

    #[test]
    fn test_i18n_fallback_to_english() {
        let mut i18n = I18n::with_catalog(test_catalog());
        i18n.set_language(Language::Japanese);

        // "multi-var" only exists in English, should fallback
        let mut vars = HashMap::new();
        vars.insert("a", "first");
        vars.insert("b", "second");
        assert_eq!(i18n.get_with_vars("multi-var", &vars), "first and second");
    }

    #[test]
    fn test_i18n_fallback_to_key() {
        let i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        // Non-existent key should return the key itself
        assert_eq!(i18n.get("nonexistent-key"), "nonexistent-key");
    }

    #[test]
    fn test_language_change() {
        let mut i18n = I18n {
            catalog: test_catalog(),
            current_language: Language::English,
        };

        assert_eq!(i18n.get("hello"), "Hello");

        i18n.set_language(Language::Japanese);
        assert_eq!(i18n.get("hello"), "こんにちは");
    }
}
