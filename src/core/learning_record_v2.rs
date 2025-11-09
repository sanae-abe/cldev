//! Learning Record V2 - Structured learning session data
//!
//! Provides YAML-based learning records with rich metadata for tracking
//! development sessions, problems, solutions, and learnings.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// ============================================================================
// Enums
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    Learning,
    Urgent,
    Fix,
    Debug,
    Feature,
    Refactor,
    Optimize,
    Research,
}

impl SessionType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionType::Learning => "learning",
            SessionType::Urgent => "urgent",
            SessionType::Fix => "fix",
            SessionType::Debug => "debug",
            SessionType::Feature => "feature",
            SessionType::Refactor => "refactor",
            SessionType::Optimize => "optimize",
            SessionType::Research => "research",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Reusability {
    Low,
    Medium,
    High,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FileRole {
    Primary,
    Secondary,
    Related,
}

// ============================================================================
// Structs
// ============================================================================

/// Session metadata
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMeta {
    pub id: String,
    pub session_type: SessionType,
    pub priority: Priority,
    pub timestamp: DateTime<Local>,
    pub duration_minutes: Option<i64>,
    pub resolved: bool,
}

/// Problem description
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub error_signatures: Vec<ErrorSignature>,
}

/// Error signature for pattern matching
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSignature {
    pub error_type: String,
    pub pattern: String,
    pub stack_trace_hash: Option<String>,
}

/// Solution details
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub summary: String,
    pub root_cause: Option<String>,
    pub steps: Vec<String>,
    pub verification: Vec<String>,
}

/// File affected by this session
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAffected {
    pub path: String,
    pub role: FileRole,
    pub changes_summary: Option<String>,
    pub hotspot_score: f64,
}

/// Context information
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub tags: Vec<String>,
    pub files_affected: Vec<FileAffected>,
    pub dependencies: Vec<Dependency>,
    pub environment: Option<String>,
}

/// Dependency information
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub relevance: String,
}

/// Learning insight
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Learning {
    pub insight: String,
    pub category: String,
    pub reusability: Reusability,
    pub applicable_to: Vec<String>,
}

/// Main learning record V2 structure
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecordV2 {
    pub session_meta: SessionMeta,
    pub problem: Problem,
    pub solution: Option<Solution>,
    pub context: Context,
    pub learnings: Vec<Learning>,
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for creating learning records
#[allow(dead_code)]
pub struct LearningRecordBuilder {
    session_meta: SessionMeta,
    problem: Problem,
    solution: Option<Solution>,
    context: Context,
    learnings: Vec<Learning>,
}

#[allow(dead_code)]
impl LearningRecordBuilder {
    /// Create new builder
    pub fn new(
        session_type: SessionType,
        priority: Priority,
        title: impl Into<String>,
        description: impl Into<String>,
        severity: Severity,
    ) -> Self {
        let now = Local::now();
        let session_type_str = session_type.as_str();
        let id = format!("{}_{}", session_type_str, now.format("%Y%m%d_%H%M%S_%3f"));

        Self {
            session_meta: SessionMeta {
                id,
                session_type,
                priority,
                timestamp: now,
                duration_minutes: None,
                resolved: false,
            },
            problem: Problem {
                title: title.into(),
                description: description.into(),
                severity,
                error_signatures: Vec::new(),
            },
            solution: None,
            context: Context {
                tags: Vec::new(),
                files_affected: Vec::new(),
                dependencies: Vec::new(),
                environment: None,
            },
            learnings: Vec::new(),
        }
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.context.tags = tags;
        self
    }

    /// Add a single tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.context.tags.push(tag.into());
        self
    }

    /// Add affected files
    pub fn files(mut self, files: Vec<FileAffected>) -> Self {
        self.context.files_affected = files;
        self
    }

    /// Add solution
    pub fn solution(mut self, solution: Solution) -> Self {
        self.solution = Some(solution);
        self
    }

    /// Mark as resolved
    pub fn resolved(mut self, duration_minutes: i64) -> Self {
        self.session_meta.resolved = true;
        self.session_meta.duration_minutes = Some(duration_minutes);
        self
    }

    /// Add learning
    pub fn learning(mut self, learning: Learning) -> Self {
        self.learnings.push(learning);
        self
    }

    /// Build the record
    pub fn build(self) -> LearningRecordV2 {
        LearningRecordV2 {
            session_meta: self.session_meta,
            problem: self.problem,
            solution: self.solution,
            context: self.context,
            learnings: self.learnings,
        }
    }

    /// Build and save to file
    pub fn save(self) -> crate::core::Result<(LearningRecordV2, std::path::PathBuf)> {
        use crate::core::CldevError;
        use std::fs;
        use std::path::PathBuf;

        let record = self.build();

        // Get records directory
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let records_dir = home.join(".claude").join("learning-records");
        if !records_dir.exists() {
            fs::create_dir_all(&records_dir)?;
        }

        // Save as YAML with frontmatter
        let path = records_dir.join(format!("{}.md", record.session_meta.id));
        let yaml_content = serde_yaml::to_string(&record)
            .map_err(|e| CldevError::Config(format!("Failed to serialize record: {}", e)))?;

        let content = format!(
            "---\n{}---\n\n# Session Notes\n\nAdd your notes here...\n",
            yaml_content
        );
        fs::write(&path, content)?;

        // Update database index
        use crate::core::LearningDatabase;
        let db_path = records_dir.join("learning.db");
        let mut db = LearningDatabase::new(db_path, records_dir.clone())?;
        db.upsert_session(&record, path.to_string_lossy().to_string())?;

        Ok((record, path))
    }
}

impl LearningRecordV2 {
    /// Load a learning record from markdown file by ID
    #[allow(dead_code)]
    pub fn load(id: &str) -> crate::core::Result<Self> {
        use crate::core::CldevError;
        use std::path::PathBuf;

        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let path = home
            .join(".claude")
            .join("learning-records")
            .join(format!("{}.md", id));

        if !path.exists() {
            return Err(CldevError::config(format!(
                "Learning record not found: {}",
                id
            )));
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| CldevError::config(format!("Failed to read learning record: {}", e)))?;

        Self::parse_markdown(&content)
    }

    /// List all learning record IDs
    #[allow(dead_code)]
    pub fn list_all() -> crate::core::Result<Vec<String>> {
        use crate::core::CldevError;
        use std::path::PathBuf;

        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let dir = home.join(".claude").join("learning-records");
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut ids = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
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

    /// Parse markdown file with YAML frontmatter
    #[allow(dead_code)]
    fn parse_markdown(content: &str) -> crate::core::Result<Self> {
        use crate::core::CldevError;

        // Extract YAML frontmatter between --- delimiters
        if !content.starts_with("---\n") {
            return Err(CldevError::config(
                "Invalid format: missing YAML frontmatter",
            ));
        }

        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() < 3 {
            return Err(CldevError::config("Invalid format: incomplete frontmatter"));
        }

        let yaml_content = parts[1];
        let record: Self = serde_yaml::from_str(yaml_content)
            .map_err(|e| CldevError::Config(format!("Failed to parse YAML frontmatter: {}", e)))?;

        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let record = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Test Issue",
            "Test description",
            Severity::Error,
        )
        .tag("test")
        .build();

        assert_eq!(record.problem.title, "Test Issue");
        assert_eq!(record.context.tags.len(), 1);
        assert_eq!(record.session_meta.priority, Priority::High);
    }

    #[test]
    fn test_serialization() {
        let record = LearningRecordBuilder::new(
            SessionType::Feature,
            Priority::Medium,
            "New Feature",
            "Feature description",
            Severity::Info,
        )
        .build();

        let yaml = serde_yaml::to_string(&record).unwrap();
        let deserialized: LearningRecordV2 = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(record.problem.title, deserialized.problem.title);
    }
}
