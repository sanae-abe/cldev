#![allow(dead_code)]

use crate::core::{CldevError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Learning session metadata for tracking development patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSession {
    /// Unique session ID (timestamp-based)
    pub id: String,

    /// Session type (urgent, fix, debug, feature, etc.)
    pub session_type: String,

    /// Timestamp when session started
    pub timestamp: String,

    /// Problem or task description
    pub description: String,

    /// Root cause analysis (if applicable)
    pub root_cause: Option<String>,

    /// Solution summary
    pub solution: Option<String>,

    /// Time taken to resolve (in minutes)
    pub duration_minutes: Option<u32>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Key learnings and insights
    pub learnings: Vec<String>,

    /// Related files or components
    pub files_affected: Vec<String>,

    /// Commands or steps executed
    pub steps_taken: Vec<String>,

    /// Success indicator
    pub resolved: bool,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl LearningSession {
    /// Create a new learning session
    pub fn new(session_type: impl Into<String>, description: impl Into<String>) -> Self {
        let session_type_str = session_type.into();
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        // Include milliseconds to ensure uniqueness when creating multiple sessions quickly
        let id = format!("{}_{}", session_type_str, now.format("%Y%m%d_%H%M%S_%3f"));

        Self {
            id,
            session_type: session_type_str,
            timestamp,
            description: description.into(),
            root_cause: None,
            solution: None,
            duration_minutes: None,
            tags: Vec::new(),
            learnings: Vec::new(),
            files_affected: Vec::new(),
            steps_taken: Vec::new(),
            resolved: false,
            metadata: HashMap::new(),
        }
    }

    /// Add a tag to the session
    pub fn add_tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    pub fn add_tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.tags.extend(tags);
        self
    }

    /// Add a learning insight
    pub fn add_learning(&mut self, learning: impl Into<String>) -> &mut Self {
        self.learnings.push(learning.into());
        self
    }

    /// Add an affected file
    pub fn add_file(&mut self, file: impl Into<String>) -> &mut Self {
        self.files_affected.push(file.into());
        self
    }

    /// Add a step taken
    pub fn add_step(&mut self, step: impl Into<String>) -> &mut Self {
        self.steps_taken.push(step.into());
        self
    }

    /// Set root cause analysis
    pub fn set_root_cause(&mut self, cause: impl Into<String>) -> &mut Self {
        self.root_cause = Some(cause.into());
        self
    }

    /// Set solution summary
    pub fn set_solution(&mut self, solution: impl Into<String>) -> &mut Self {
        self.solution = Some(solution.into());
        self
    }

    /// Mark session as resolved
    pub fn mark_resolved(&mut self, duration_minutes: Option<u32>) -> &mut Self {
        self.resolved = true;
        self.duration_minutes = duration_minutes;
        self
    }

    /// Add metadata entry
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Get the default sessions directory path
    fn sessions_dir() -> Result<PathBuf> {
        // Prioritize HOME env var for testing, fallback to dirs::home_dir()
        let home = std::env::var("HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
            .ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let sessions_path = home.join(".claude").join("learning-sessions");

        if !sessions_path.exists() {
            fs::create_dir_all(&sessions_path)?;
        }

        Ok(sessions_path)
    }

    /// Convert session to Markdown format with YAML frontmatter
    fn to_markdown(&self) -> String {
        // YAML Frontmatter (minimal metadata for machine readability)
        let frontmatter = format!(
            "---\nid: {}\ntype: {}\ndate: {}\nresolved: {}\nduration: {}\ntags: [{}]\n---\n\n",
            self.id,
            self.session_type,
            self.timestamp.split(' ').next().unwrap_or(""),
            self.resolved,
            self.duration_minutes.unwrap_or(0),
            self.tags.join(", ")
        );

        // Session type label
        let type_label = match self.session_type.as_str() {
            "urgent" => "緊急対応",
            "fix" => "バグ修正",
            "debug" => "デバッグ",
            "feature" => "新機能実装",
            "refactor" => "リファクタリング",
            "optimize" => "最適化",
            "research" => "技術調査",
            _ => "学習記録",
        };

        // Markdown body (compact format for Claude Code readability)
        let body = format!(
            "# {} - {}\n\n## 問題\n{}\n\n## 根本原因\n{}\n\n## 解決策\n{}\n\n## 学び\n{}\n\n## 関連ファイル\n{}\n",
            self.description,
            type_label,
            self.description,
            self.root_cause.as_deref().unwrap_or("調査中"),
            self.solution.as_deref().unwrap_or("未解決"),
            if self.learnings.is_empty() {
                "（記録なし）".to_string()
            } else {
                self.learnings
                    .iter()
                    .map(|l| format!("- {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            if self.files_affected.is_empty() {
                "（記録なし）".to_string()
            } else {
                self.files_affected
                    .iter()
                    .map(|f| format!("- `{}`", f))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        );

        format!("{}{}", frontmatter, body)
    }

    /// Save session to disk in Markdown format
    pub fn save(&self) -> Result<PathBuf> {
        let sessions_dir = Self::sessions_dir()?;

        // New naming convention: YYYY-MM-DD-{type}-{slug}.md
        let date = chrono::Local::now().format("%Y-%m-%d");
        let slug = self
            .description
            .chars()
            .take(40)
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-')
            .collect::<String>()
            .to_lowercase()
            .replace(' ', "-")
            .trim_matches('-')
            .to_string();

        let filename = format!("{}-{}-{}.md", date, self.session_type, slug);
        let filepath = sessions_dir.join(filename);

        // Save as Markdown
        let markdown = self.to_markdown();
        fs::write(&filepath, markdown)?;

        Ok(filepath)
    }

    /// Parse legacy markdown format (without YAML frontmatter)
    fn from_legacy_markdown(content: &str) -> Result<Self> {
        // Extract basic metadata from the content
        let lines: Vec<&str> = content.lines().collect();

        // Try to extract title from first heading
        let description = lines
            .iter()
            .find(|line| line.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").trim().to_string())
            .unwrap_or_else(|| "Learning session".to_string());

        // Try to extract date from **日付**: pattern or filename
        let date_str = lines
            .iter()
            .find(|line| line.contains("**日付**:") || line.contains("**Date**:"))
            .and_then(|line| {
                line.split(':')
                    .nth(1)
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

        // Add default time to make it compatible with stats.rs timestamp parsing
        let timestamp = if date_str.contains(" ") {
            date_str // Already has time
        } else {
            format!("{} 00:00:00", date_str) // Add default time
        };

        // Generate a unique ID based on description
        let id = format!(
            "legacy_{}_{}",
            timestamp.replace("-", ""),
            description
                .chars()
                .take(30)
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase()
        );

        // Try to extract session type from filename or content
        let session_type = if description.contains("セキュリティ") || description.contains("security") {
            "security"
        } else if description.contains("監査") || description.contains("audit") {
            "audit"
        } else if description.contains("実装") || description.contains("implementation") {
            "feature"
        } else if description.contains("修正") || description.contains("fix") {
            "fix"
        } else {
            "learning"
        }.to_string();

        Ok(Self {
            id,
            session_type,
            timestamp,
            description,
            root_cause: None,
            solution: None,
            duration_minutes: None,
            tags: Vec::new(),
            learnings: Vec::new(),
            files_affected: Vec::new(),
            steps_taken: Vec::new(),
            resolved: true, // Assume legacy sessions are completed
            metadata: HashMap::new(),
        })
    }

    /// Parse Markdown content and extract YAML frontmatter
    fn from_markdown(content: &str) -> Result<Self> {
        // Check if content starts with YAML frontmatter (---\n at the beginning)
        let has_frontmatter = content.trim_start().starts_with("---\n") || content.trim_start().starts_with("---\r\n");

        if !has_frontmatter {
            // Legacy/custom format without frontmatter - extract from markdown content
            return Self::from_legacy_markdown(content);
        }

        // Split frontmatter and body
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(CldevError::config(
                "Invalid Markdown format: missing YAML frontmatter",
            ));
        }

        let frontmatter = parts[1].trim();
        let body = parts[2].trim();

        // Parse YAML frontmatter
        let mut id = String::new();
        let mut session_type = String::new();
        let mut timestamp = String::new();
        let mut resolved = false;
        let mut duration_minutes: Option<u32> = None;
        let mut tags: Vec<String> = Vec::new();

        for line in frontmatter.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "id" => id = value.to_string(),
                    "type" => session_type = value.to_string(),
                    "date" => timestamp = value.to_string(),
                    "resolved" => resolved = value == "true",
                    "duration" => duration_minutes = value.parse().ok(),
                    "tags" => {
                        // Parse tags: [tag1, tag2, tag3]
                        let tags_str = value.trim_matches(|c| c == '[' || c == ']');
                        tags = tags_str
                            .split(',')
                            .map(|t| t.trim().to_string())
                            .filter(|t| !t.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        // Extract body sections
        let mut description = String::new();
        let mut root_cause: Option<String> = None;
        let mut solution: Option<String> = None;
        let mut learnings: Vec<String> = Vec::new();
        let mut files_affected: Vec<String> = Vec::new();

        let sections: Vec<&str> = body.split("## ").collect();
        for section in sections {
            if section.trim().is_empty() {
                continue;
            }

            if section.starts_with("# ") {
                // Extract description from title
                if let Some(title) = section.lines().next() {
                    description = title
                        .trim_start_matches('#')
                        .split('-')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                }
            } else if section.starts_with("問題") {
                // Skip "## 問題" section as it duplicates description
                continue;
            } else if section.starts_with("根本原因") {
                let cause_text = section
                    .lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();
                if cause_text != "調査中" {
                    root_cause = Some(cause_text);
                }
            } else if section.starts_with("解決策") {
                let sol_text = section
                    .lines()
                    .skip(1)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();
                if sol_text != "未解決" {
                    solution = Some(sol_text);
                }
            } else if section.starts_with("学び") {
                for line in section.lines().skip(1) {
                    let line = line.trim();
                    if line.starts_with("- ") && !line.contains("記録なし") {
                        learnings.push(line.trim_start_matches("- ").to_string());
                    }
                }
            } else if section.starts_with("関連ファイル") {
                for line in section.lines().skip(1) {
                    let line = line.trim();
                    if line.starts_with("- `") && !line.contains("記録なし") {
                        files_affected.push(
                            line.trim_start_matches("- `")
                                .trim_end_matches('`')
                                .to_string(),
                        );
                    }
                }
            }
        }

        Ok(Self {
            id,
            session_type,
            timestamp,
            description,
            root_cause,
            solution,
            duration_minutes,
            tags,
            learnings,
            files_affected,
            steps_taken: Vec::new(), // Not stored in compact format
            resolved,
            metadata: HashMap::new(), // Not stored in compact format
        })
    }

    /// Load a session from disk (supports both .md and legacy .json)
    pub fn load(id: &str) -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;

        // Try .md first (new format)
        let md_filepath = Self::find_session_file(&sessions_dir, id, "md")?;
        if md_filepath.exists() {
            let content = fs::read_to_string(&md_filepath)?;
            return Self::from_markdown(&content);
        }

        // Fallback to .json (legacy format)
        let json_filepath = sessions_dir.join(format!("{}.json", id));
        if json_filepath.exists() {
            let content = fs::read_to_string(&json_filepath)?;
            let session: LearningSession = serde_json::from_str(&content)?;
            return Ok(session);
        }

        Err(CldevError::config(format!("Session not found: {}", id)))
    }

    /// Find session file by ID pattern
    fn find_session_file(sessions_dir: &PathBuf, id: &str, ext: &str) -> Result<PathBuf> {
        // For exact ID match
        let exact_path = sessions_dir.join(format!("{}.{}", id, ext));
        if exact_path.exists() {
            return Ok(exact_path);
        }

        // For pattern match (e.g., urgent_20251107_* matches any file starting with that)
        if let Ok(entries) = fs::read_dir(sessions_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                    // Check filename stem first
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem.starts_with(id) {
                            return Ok(path);
                        }
                    }

                    // For .md files, check frontmatter ID
                    if ext == "md" {
                        if let Ok(content) = fs::read_to_string(&path) {
                            // Quick check for frontmatter without full parsing
                            if let Some(id_line) = content.lines().find(|l| l.starts_with("id:")) {
                                let file_id = id_line.trim_start_matches("id:").trim();
                                if file_id == id {
                                    return Ok(path);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(exact_path) // Return non-existent path (will be checked by caller)
    }

    /// List all saved sessions (supports both .md and .json)
    pub fn list_all() -> Result<Vec<String>> {
        let sessions_dir = Self::sessions_dir()?;

        let mut sessions = Vec::new();

        for entry in fs::read_dir(sessions_dir)? {
            let entry = entry?;
            let path = entry.path();

            let ext = path.extension().and_then(|s| s.to_str());
            if ext == Some("md") || ext == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    // For .md files, extract the ID from filename (pattern: YYYY-MM-DD-type-slug)
                    // For .json files, use the stem directly
                    if ext == Some("md") {
                        // Extract session ID from Markdown frontmatter for accurate listing
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Some(id_line) = content.lines().find(|l| l.starts_with("id:")) {
                                let id = id_line.trim_start_matches("id:").trim().to_string();
                                sessions.push(id);
                            } else {
                                // Fallback: use filename stem
                                sessions.push(stem.to_string());
                            }
                        }
                    } else {
                        sessions.push(stem.to_string());
                    }
                }
            }
        }

        sessions.sort_by(|a, b| b.cmp(a)); // Most recent first

        Ok(sessions)
    }

    /// Find sessions by tag
    pub fn find_by_tag(tag: &str) -> Result<Vec<LearningSession>> {
        let session_ids = Self::list_all()?;
        let mut matching_sessions = Vec::new();

        for id in session_ids {
            if let Ok(session) = Self::load(&id) {
                if session.tags.contains(&tag.to_string()) {
                    matching_sessions.push(session);
                }
            }
        }

        Ok(matching_sessions)
    }

    /// Find sessions by type
    pub fn find_by_type(session_type: &str) -> Result<Vec<LearningSession>> {
        let session_ids = Self::list_all()?;
        let mut matching_sessions = Vec::new();

        for id in session_ids {
            if let Ok(session) = Self::load(&id) {
                if session.session_type == session_type {
                    matching_sessions.push(session);
                }
            }
        }

        Ok(matching_sessions)
    }
}

/// Builder for creating learning sessions with a fluent API
pub struct LearningSessionBuilder {
    session: LearningSession,
}

impl LearningSessionBuilder {
    /// Create a new builder
    pub fn new(session_type: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            session: LearningSession::new(session_type, description),
        }
    }

    /// Add a tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.session.add_tag(tag);
        self
    }

    /// Add multiple tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.session.add_tags(tags);
        self
    }

    /// Add a learning
    pub fn learning(mut self, learning: impl Into<String>) -> Self {
        self.session.add_learning(learning);
        self
    }

    /// Add a file
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.session.add_file(file);
        self
    }

    /// Add a step
    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.session.add_step(step);
        self
    }

    /// Set root cause
    pub fn root_cause(mut self, cause: impl Into<String>) -> Self {
        self.session.set_root_cause(cause);
        self
    }

    /// Set solution
    pub fn solution(mut self, solution: impl Into<String>) -> Self {
        self.session.set_solution(solution);
        self
    }

    /// Mark as resolved
    pub fn resolved(mut self, duration_minutes: Option<u32>) -> Self {
        self.session.mark_resolved(duration_minutes);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.session.add_metadata(key, value);
        self
    }

    /// Build the session
    pub fn build(self) -> LearningSession {
        self.session
    }

    /// Build and save the session
    pub fn save(self) -> Result<(LearningSession, PathBuf)> {
        let session = self.session;
        let path = session.save()?;
        Ok((session, path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = LearningSession::new("urgent", "Critical production bug");
        assert_eq!(session.session_type, "urgent");
        assert_eq!(session.description, "Critical production bug");
        assert!(!session.resolved);
    }

    #[test]
    fn test_session_builder() {
        let session = LearningSessionBuilder::new("debug", "Memory leak investigation")
            .tag("performance")
            .tag("memory")
            .learning("Always check resource cleanup")
            .file("src/main.rs")
            .step("Ran memory profiler")
            .resolved(Some(45))
            .build();

        assert_eq!(session.tags.len(), 2);
        assert_eq!(session.learnings.len(), 1);
        assert_eq!(session.files_affected.len(), 1);
        assert_eq!(session.steps_taken.len(), 1);
        assert!(session.resolved);
        assert_eq!(session.duration_minutes, Some(45));
    }

    #[test]
    fn test_markdown_roundtrip() {
        // Create a session with full data
        let original = LearningSessionBuilder::new("urgent", "JWT authentication failure")
            .tag("production")
            .tag("auth")
            .learning("Key rotation requires gradual rollout")
            .learning("Token refresh should be implemented from day 1")
            .file("src/auth/jwt.rs")
            .file("src/auth/middleware.rs")
            .root_cause("Old tokens verified with new key after rotation")
            .solution("Accept both old and new keys temporarily")
            .resolved(Some(45))
            .build();

        // Convert to Markdown
        let markdown = original.to_markdown();

        // Verify Markdown structure
        assert!(markdown.contains("---"));
        assert!(markdown.contains("id:"));
        assert!(markdown.contains("type: urgent"));
        assert!(markdown.contains("# JWT authentication failure"));
        assert!(markdown.contains("## 問題"));
        assert!(markdown.contains("## 根本原因"));
        assert!(markdown.contains("## 解決策"));
        assert!(markdown.contains("## 学び"));
        assert!(markdown.contains("## 関連ファイル"));

        // Parse back from Markdown
        let parsed = LearningSession::from_markdown(&markdown).expect("Failed to parse Markdown");

        // Verify key fields match
        assert_eq!(parsed.session_type, "urgent");
        assert_eq!(parsed.description, "JWT authentication failure");
        assert!(parsed.resolved);
        assert_eq!(parsed.duration_minutes, Some(45));
        assert_eq!(parsed.tags.len(), 2);
        assert!(parsed.tags.contains(&"production".to_string()));
        assert!(parsed.tags.contains(&"auth".to_string()));
        assert_eq!(parsed.learnings.len(), 2);
        assert_eq!(parsed.files_affected.len(), 2);
    }

    #[test]
    fn test_markdown_minimal_session() {
        // Create minimal session (no root cause, solution, learnings)
        let original = LearningSession::new("debug", "Investigation in progress");

        let markdown = original.to_markdown();
        let parsed = LearningSession::from_markdown(&markdown).expect("Failed to parse");

        assert_eq!(parsed.session_type, "debug");
        assert_eq!(parsed.description, "Investigation in progress");
        assert_eq!(parsed.root_cause, None);
        assert_eq!(parsed.solution, None);
        assert_eq!(parsed.learnings.len(), 0);
        assert_eq!(parsed.files_affected.len(), 0);
    }

    #[test]
    fn test_markdown_with_multiline_content() {
        let mut session = LearningSession::new("fix", "Memory leak in event handler");
        session.set_root_cause(
            "Event listeners not properly cleaned up\nMultiple subscriptions created",
        );
        session.set_solution(
            "Implement cleanup in componentWillUnmount\nUse single subscription pattern",
        );
        session.add_learning("Always cleanup event listeners");
        session.add_learning("Consider using useEffect cleanup");

        let markdown = session.to_markdown();
        let parsed = LearningSession::from_markdown(&markdown).expect("Failed to parse");

        assert!(parsed.root_cause.is_some());
        assert!(parsed.solution.is_some());
        assert_eq!(parsed.learnings.len(), 2);
    }
}
