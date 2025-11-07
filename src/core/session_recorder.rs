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
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let id = format!(
            "{}_{}",
            session_type_str,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        );

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
        let home =
            dirs::home_dir().ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let sessions_path = home.join(".claude").join("learning-sessions");

        if !sessions_path.exists() {
            fs::create_dir_all(&sessions_path)?;
        }

        Ok(sessions_path)
    }

    /// Save session to disk
    pub fn save(&self) -> Result<PathBuf> {
        let sessions_dir = Self::sessions_dir()?;
        let filename = format!("{}.json", self.id);
        let filepath = sessions_dir.join(filename);

        let json = serde_json::to_string_pretty(self)?;

        fs::write(&filepath, json)?;

        Ok(filepath)
    }

    /// Load a session from disk
    pub fn load(id: &str) -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;
        let filepath = sessions_dir.join(format!("{}.json", id));

        if !filepath.exists() {
            return Err(CldevError::config(format!("Session not found: {}", id)));
        }

        let content = fs::read_to_string(&filepath)?;
        let session: LearningSession = serde_json::from_str(&content)?;

        Ok(session)
    }

    /// List all saved sessions
    pub fn list_all() -> Result<Vec<String>> {
        let sessions_dir = Self::sessions_dir()?;

        let mut sessions = Vec::new();

        for entry in fs::read_dir(sessions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    sessions.push(stem.to_string());
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
}
