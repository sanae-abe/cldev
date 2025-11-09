//! Session Context Tracking
//!
//! Automatically tracks session activity including commands, errors, todos, and file changes.
//! Used for auto-generating learning records with minimal user input.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Session context for tracking development activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub start_time: DateTime<Local>,
    pub command_history: Vec<CommandRecord>,
    pub todo_history: Vec<TodoSnapshot>,
    pub errors_encountered: Vec<ErrorCapture>,
    pub files_modified: Vec<FileModification>,
    pub tool_usage: Vec<ToolUsage>,
}

/// Command execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRecord {
    pub command: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
    pub timestamp: DateTime<Local>,
    pub working_dir: String,
}

/// Todo snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoSnapshot {
    pub timestamp: DateTime<Local>,
    pub action: TodoAction,
    pub content: String,
    pub status: TodoStatus,
}

/// Todo action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoAction {
    Created,
    Updated,
    Completed,
    Deleted,
}

/// Todo status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

/// Error capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCapture {
    pub timestamp: DateTime<Local>,
    pub error_type: String,
    pub message: String,
    pub context: Option<String>,
    pub resolved: bool,
}

/// File modification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileModification {
    pub file_path: String,
    pub modification_type: ModificationType,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub timestamp: DateTime<Local>,
}

/// Modification type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Created,
    Modified,
    Deleted,
    Renamed,
}

/// Tool usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsage {
    pub tool_name: String,
    pub timestamp: DateTime<Local>,
    pub duration_ms: u64,
    pub success: bool,
}

impl SessionContext {
    /// Create a new session context
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            start_time: Local::now(),
            command_history: Vec::new(),
            todo_history: Vec::new(),
            errors_encountered: Vec::new(),
            files_modified: Vec::new(),
            tool_usage: Vec::new(),
        }
    }

    /// Add a command record
    pub fn add_command(&mut self, record: CommandRecord) {
        self.command_history.push(record);
    }

    /// Add a todo snapshot
    pub fn add_todo(&mut self, snapshot: TodoSnapshot) {
        self.todo_history.push(snapshot);
    }

    /// Add an error capture
    pub fn add_error(&mut self, error: ErrorCapture) {
        self.errors_encountered.push(error);
    }

    /// Add a file modification
    pub fn add_file_modification(&mut self, modification: FileModification) {
        self.files_modified.push(modification);
    }

    /// Add tool usage
    pub fn add_tool_usage(&mut self, usage: ToolUsage) {
        self.tool_usage.push(usage);
    }

    /// Get session duration in minutes
    pub fn duration_minutes(&self) -> i64 {
        (Local::now() - self.start_time).num_minutes()
    }

    /// Count failed commands
    pub fn failed_commands_count(&self) -> usize {
        self.command_history
            .iter()
            .filter(|c| c.exit_code != 0)
            .count()
    }

    /// Count unresolved errors
    pub fn unresolved_errors_count(&self) -> usize {
        self.errors_encountered
            .iter()
            .filter(|e| !e.resolved)
            .count()
    }

    /// Count completed todos
    pub fn completed_todos_count(&self) -> usize {
        self.todo_history
            .iter()
            .filter(|t| matches!(t.status, TodoStatus::Completed))
            .count()
    }

    /// Get unique files modified
    pub fn unique_files_modified(&self) -> usize {
        use std::collections::HashSet;
        self.files_modified
            .iter()
            .map(|f| &f.file_path)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_context_creation() {
        let ctx = SessionContext::new("test-session".to_string());
        assert_eq!(ctx.session_id, "test-session");
        assert_eq!(ctx.command_history.len(), 0);
    }

    #[test]
    fn test_add_command() {
        let mut ctx = SessionContext::new("test".to_string());
        ctx.add_command(CommandRecord {
            command: "cargo test".to_string(),
            exit_code: 0,
            execution_time_ms: 1500,
            timestamp: Local::now(),
            working_dir: "/test".to_string(),
        });
        assert_eq!(ctx.command_history.len(), 1);
        assert_eq!(ctx.failed_commands_count(), 0);
    }

    #[test]
    fn test_failed_commands_count() {
        let mut ctx = SessionContext::new("test".to_string());
        ctx.add_command(CommandRecord {
            command: "cargo build".to_string(),
            exit_code: 1,
            execution_time_ms: 500,
            timestamp: Local::now(),
            working_dir: "/test".to_string(),
        });
        assert_eq!(ctx.failed_commands_count(), 1);
    }

    #[test]
    fn test_unresolved_errors() {
        let mut ctx = SessionContext::new("test".to_string());
        ctx.add_error(ErrorCapture {
            timestamp: Local::now(),
            error_type: "CompileError".to_string(),
            message: "test error".to_string(),
            context: None,
            resolved: false,
        });
        assert_eq!(ctx.unresolved_errors_count(), 1);
    }
}
