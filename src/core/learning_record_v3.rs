//! Learning Record V3 - Human-first format
//!
//! Minimal YAML frontmatter (10%) + Markdown body (90%)
//! Designed for natural human writing with AI processing

use crate::core::{CldevError, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Learning Record V3 - Human-first format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecordV3 {
    pub id: String,
    pub created: DateTime<Local>,
    pub auto_generated: bool,
    pub confidence: Option<f64>,
    pub tags: Vec<String>,
    pub status: RecordStatus,
    pub duration_min: Option<i64>,
    pub markdown_body: String,
}

/// Record status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RecordStatus {
    Pending,
    #[serde(rename = "in_progress")]
    InProgress,
    Resolved,
}

impl LearningRecordV3 {
    /// Create a new learning record
    pub fn new(id: String, markdown_body: String) -> Self {
        Self {
            id,
            created: Local::now(),
            auto_generated: false,
            confidence: None,
            tags: Vec::new(),
            status: RecordStatus::Pending,
            duration_min: None,
            markdown_body,
        }
    }

    /// List all V3 learning record IDs
    pub fn list_all() -> Result<Vec<String>> {
        let home = dirs::home_dir()
            .ok_or_else(|| CldevError::Config("Could not determine home directory".to_string()))?;

        let lr_dir = home.join(".cldev").join("learning-records");
        if !lr_dir.exists() {
            return Ok(Vec::new());
        }

        let mut ids = Vec::new();
        for entry in std::fs::read_dir(lr_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    ids.push(stem.to_string());
                }
            }
        }

        Ok(ids)
    }

    /// Load a V3 learning record by ID
    pub fn load(id: &str) -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| CldevError::Config("Could not determine home directory".to_string()))?;

        let file_path = home
            .join(".cldev")
            .join("learning-records")
            .join(format!("{}.md", id));

        if !file_path.exists() {
            return Err(CldevError::Config(format!(
                "Learning record not found: {}",
                id
            )));
        }

        let content = std::fs::read_to_string(&file_path)?;
        Self::from_markdown_file(&content)
    }

    /// Convert to markdown file format
    pub fn to_markdown_file(&self) -> String {
        let yaml = serde_yaml::to_string(&FrontMatter {
            id: self.id.clone(),
            created: self.created.format("%Y-%m-%d %H:%M").to_string(),
            auto_generated: self.auto_generated,
            confidence: self.confidence,
            tags: self.tags.clone(),
            status: self.status.clone(),
            duration_min: self.duration_min,
        })
        .unwrap_or_else(|_| String::from("id: error\n"));

        format!("---\n{}---\n\n{}", yaml, self.markdown_body)
    }

    /// Parse from markdown file format
    pub fn from_markdown_file(content: &str) -> Result<Self> {
        // Extract YAML frontmatter
        if !content.starts_with("---\n") {
            return Err(CldevError::Config(
                "Missing YAML frontmatter delimiter".to_string(),
            ));
        }

        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Err(CldevError::Config("Invalid frontmatter format".to_string()));
        }

        let yaml_content = parts[1];
        let markdown_body = parts[2].trim().to_string();

        let frontmatter: FrontMatter = serde_yaml::from_str(yaml_content)
            .map_err(|e| CldevError::Config(format!("Failed to parse frontmatter: {}", e)))?;

        // Parse created timestamp
        let created = DateTime::parse_from_str(
            &format!("{} +00:00", frontmatter.created),
            "%Y-%m-%d %H:%M %z",
        )
        .or_else(|_| {
            // Fallback: try without time
            DateTime::parse_from_str(
                &format!("{} 00:00 +00:00", frontmatter.created),
                "%Y-%m-%d %H:%M %z",
            )
        })
        .map_err(|e| CldevError::Config(format!("Failed to parse created timestamp: {}", e)))?
        .with_timezone(&Local);

        Ok(Self {
            id: frontmatter.id,
            created,
            auto_generated: frontmatter.auto_generated,
            confidence: frontmatter.confidence,
            tags: frontmatter.tags,
            status: frontmatter.status,
            duration_min: frontmatter.duration_min,
            markdown_body,
        })
    }
}

/// YAML frontmatter structure
#[derive(Debug, Serialize, Deserialize)]
struct FrontMatter {
    id: String,
    created: String,
    auto_generated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<f64>,
    tags: Vec<String>,
    status: RecordStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_min: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_learning_record_v3() {
        let record = LearningRecordV3::new(
            "test-record".to_string(),
            "# Test\n\nThis is a test.".to_string(),
        );
        assert_eq!(record.id, "test-record");
        assert_eq!(record.status, RecordStatus::Pending);
        assert!(!record.auto_generated);
    }

    #[test]
    fn test_to_markdown_file() {
        let mut record = LearningRecordV3::new(
            "test-id".to_string(),
            "# Problem\n\nTest problem.".to_string(),
        );
        record.tags = vec!["rust".to_string(), "test".to_string()];
        record.status = RecordStatus::Resolved;

        let markdown = record.to_markdown_file();
        assert!(markdown.starts_with("---\n"));
        assert!(markdown.contains("id: test-id"));
        assert!(markdown.contains("tags:"));
        assert!(markdown.contains("- rust"));
        assert!(markdown.contains("status: resolved"));
        assert!(markdown.contains("# Problem"));
    }

    #[test]
    fn test_roundtrip() {
        let original = LearningRecordV3::new(
            "roundtrip-test".to_string(),
            "# Test Content\n\nSome **markdown** here.".to_string(),
        );

        let markdown = original.to_markdown_file();
        let parsed = LearningRecordV3::from_markdown_file(&markdown).unwrap();

        assert_eq!(original.id, parsed.id);
        assert_eq!(original.markdown_body, parsed.markdown_body);
        assert_eq!(original.status, parsed.status);
    }

    #[test]
    fn test_parse_with_confidence() {
        let content = r#"---
id: auto-test
created: 2025-01-10 14:30
auto_generated: true
confidence: 0.85
tags:
  - rust
  - error
status: resolved
duration_min: 15
---

# Auto-generated Record

This was automatically created.
"#;

        let record = LearningRecordV3::from_markdown_file(content).unwrap();
        assert_eq!(record.id, "auto-test");
        assert!(record.auto_generated);
        assert_eq!(record.confidence, Some(0.85));
        assert_eq!(record.tags.len(), 2);
        assert_eq!(record.status, RecordStatus::Resolved);
        assert_eq!(record.duration_min, Some(15));
    }

    #[test]
    fn test_minimal_record() {
        let content = r#"---
id: minimal
created: 2025-01-10 10:00
auto_generated: false
tags: []
status: pending
---

# Minimal

Just a note.
"#;

        let record = LearningRecordV3::from_markdown_file(content).unwrap();
        assert_eq!(record.id, "minimal");
        assert!(!record.auto_generated);
        assert_eq!(record.confidence, None);
        assert_eq!(record.duration_min, None);
    }

    #[test]
    fn test_session_end_integration() {
        use crate::core::{
            analyze_session, generate_level2_markdown, ErrorCapture, FileModification,
            ModificationType, SessionContext, TodoAction, TodoSnapshot, TodoStatus,
        };

        let mut ctx = SessionContext::new("integration-test".to_string());

        ctx.add_error(ErrorCapture {
            timestamp: chrono::Local::now(),
            error_type: "CompileError".to_string(),
            message: "test error".to_string(),
            context: None,
            resolved: false,
        });

        ctx.add_file_modification(FileModification {
            file_path: "src/main.rs".to_string(),
            modification_type: ModificationType::Modified,
            lines_added: 15,
            lines_deleted: 3,
            timestamp: chrono::Local::now(),
        });

        let recommendation = analyze_session(&ctx);
        assert!(recommendation.score >= 0.15);

        let record = generate_level2_markdown(&ctx, &recommendation);
        assert!(record.auto_generated);
        assert!(record.markdown_body.contains("CompileError"));
    }

    #[test]
    fn test_multiline_markdown() {
        let content = r#"---
id: multiline
created: 2025-01-10 12:00
auto_generated: false
tags:
  - test
status: in_progress
---

# Title

## Section 1

Content here.

## Section 2

More content.

- List item 1
- List item 2
"#;

        let record = LearningRecordV3::from_markdown_file(content).unwrap();
        assert!(record.markdown_body.contains("## Section 1"));
        assert!(record.markdown_body.contains("- List item 1"));
        assert_eq!(record.status, RecordStatus::InProgress);
    }
}
