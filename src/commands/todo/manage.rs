use crate::core::{CldevError, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// Task status (aligned with TaskFlow)
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl TaskStatus {
    fn from_checkbox(checkbox: &str) -> Self {
        match checkbox {
            "x" | "X" => TaskStatus::Completed,
            "~" => TaskStatus::InProgress,
            _ => TaskStatus::Pending,
        }
    }

    fn to_checkbox(&self) -> &str {
        match self {
            TaskStatus::Completed => "[x]",
            TaskStatus::InProgress => "[~]",
            TaskStatus::Pending => "[ ]",
        }
    }

    fn is_completed(&self) -> bool {
        matches!(self, TaskStatus::Completed)
    }
}

/// Priority levels for todo items
#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Priority::Critical,
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Medium,
        }
    }

    fn to_emoji(&self) -> &str {
        match self {
            Priority::Critical => "🔥",
            Priority::High => "⚠️",
            Priority::Medium => "📌",
            Priority::Low => "📝",
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Priority::Critical => "Critical",
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        }
    }
}

/// Todo item parsed from Markdown
#[derive(Debug, Clone)]
pub struct TodoItem {
    pub description: String,
    pub status: TaskStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub completed_at: Option<String>,
    #[allow(dead_code)]
    pub line_number: usize,
}

/// Todo list manager for Markdown format
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    file_path: PathBuf,
}

impl TodoList {
    /// Load todo list from Markdown file
    pub fn load() -> Result<Self> {
        let path = Self::get_todo_file_path()?;

        if !path.exists() {
            return Ok(Self {
                todos: Vec::new(),
                file_path: path,
            });
        }

        let content = fs::read_to_string(&path)?;
        let todos = Self::parse_markdown(&content);

        Ok(Self {
            todos,
            file_path: path,
        })
    }

    /// Parse Markdown content into todo items
    fn parse_markdown(content: &str) -> Vec<TodoItem> {
        let mut todos = Vec::new();
        let mut current_priority = Priority::Medium;

        // Regex patterns
        let checkbox_re = Regex::new(r"^- \[([ x~])\] (.+)").unwrap();
        let priority_re = Regex::new(r"^## (🔥|⚠️|📌|📝) (.+)").unwrap();
        let tag_re = Regex::new(r"#(\w+)").unwrap();
        let date_re = Regex::new(r"\(created: ([^,)]+)(?:, completed: ([^)]+))?\)").unwrap();

        for (line_no, line) in content.lines().enumerate() {
            // Check for priority section headers
            if let Some(cap) = priority_re.captures(line) {
                let emoji = cap.get(1).unwrap().as_str();
                current_priority = match emoji {
                    "🔥" => Priority::Critical,
                    "⚠️" => Priority::High,
                    "📌" => Priority::Medium,
                    "📝" => Priority::Low,
                    _ => Priority::Medium,
                };
                continue;
            }

            // Check for checkbox items
            if let Some(cap) = checkbox_re.captures(line) {
                let checkbox_char = cap.get(1).unwrap().as_str();
                let status = TaskStatus::from_checkbox(checkbox_char);
                let text = cap.get(2).unwrap().as_str();

                // Extract description (remove tags and dates)
                let mut description = text.to_string();

                // Extract dates
                let (created_at, completed_at) = if let Some(date_cap) = date_re.captures(text) {
                    let created = date_cap.get(1).map(|m| m.as_str().to_string());
                    let completed = date_cap.get(2).map(|m| m.as_str().to_string());

                    // Remove date info from description
                    description = date_re.replace(&description, "").trim().to_string();

                    (created, completed)
                } else {
                    (None, None)
                };

                // Extract tags
                let tags: Vec<String> = tag_re
                    .captures_iter(&description)
                    .map(|cap| cap.get(1).unwrap().as_str().to_string())
                    .collect();

                // Remove tags from description
                description = tag_re.replace_all(&description, "").trim().to_string();

                todos.push(TodoItem {
                    description,
                    status,
                    priority: current_priority.clone(),
                    tags,
                    created_at,
                    completed_at,
                    line_number: line_no + 1,
                });
            }
        }

        todos
    }

    /// Save todo list to Markdown file
    pub fn save(&self) -> Result<()> {
        let content = self.to_markdown();

        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(&self.file_path, content)?;
        Ok(())
    }

    /// Convert todo list to Markdown format
    fn to_markdown(&self) -> String {
        let mut content = String::new();
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        // Header and metadata
        content.push_str("# Personal TODOs\n\n");
        content.push_str("<!-- metadata -->\n");
        content.push_str(&format!("<!-- last_updated: {} -->\n", now));
        content.push_str(&format!("<!-- total_todos: {} -->\n\n", self.todos.len()));

        // Group by priority and completion status
        let priorities = vec![
            Priority::Critical,
            Priority::High,
            Priority::Medium,
            Priority::Low,
        ];

        // Active todos by priority
        for priority in &priorities {
            content.push_str(&format!(
                "## {} {}\n",
                priority.to_emoji(),
                priority.to_string()
            ));

            let items: Vec<&TodoItem> = self
                .todos
                .iter()
                .filter(|t| !t.status.is_completed() && t.priority == *priority)
                .collect();

            if items.is_empty() {
                content.push('\n');
                continue;
            }

            for item in items {
                let tags_str = if !item.tags.is_empty() {
                    format!(
                        " {}",
                        item.tags
                            .iter()
                            .map(|t| format!("#{}", t))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    String::new()
                };

                let date_str = if let Some(created) = &item.created_at {
                    format!(" (created: {})", created)
                } else {
                    String::new()
                };

                content.push_str(&format!(
                    "- {} {}{}{}\n",
                    item.status.to_checkbox(),
                    item.description,
                    tags_str,
                    date_str
                ));
            }

            content.push('\n');
        }

        // Completed todos
        content.push_str("## ✅ Completed\n");

        let completed: Vec<&TodoItem> = self
            .todos
            .iter()
            .filter(|t| t.status.is_completed())
            .collect();

        if !completed.is_empty() {
            for item in completed {
                let tags_str = if !item.tags.is_empty() {
                    format!(
                        " {}",
                        item.tags
                            .iter()
                            .map(|t| format!("#{}", t))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    String::new()
                };

                let created_str = item
                    .created_at
                    .as_ref()
                    .map(|d| format!("created: {}", d))
                    .unwrap_or_default();

                let completed_str = item
                    .completed_at
                    .as_ref()
                    .map(|d| format!("completed: {}", d))
                    .unwrap_or_default();

                let date_str = if !created_str.is_empty() || !completed_str.is_empty() {
                    let parts: Vec<String> = vec![created_str, completed_str]
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .collect();
                    format!(" ({})", parts.join(", "))
                } else {
                    String::new()
                };

                content.push_str(&format!(
                    "- {} {}{}{}\n",
                    item.status.to_checkbox(),
                    item.description,
                    tags_str,
                    date_str
                ));
            }
        }

        content.push('\n');
        content
    }

    /// Get todo file path
    fn get_todo_file_path() -> Result<PathBuf> {
        // Try project-level first
        let project_path = PathBuf::from(".cldev/TODO.md");
        if project_path.parent().map(|p| p.exists()).unwrap_or(false) {
            return Ok(project_path);
        }

        // Fall back to global
        let home =
            dirs::home_dir().ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let global_dir = home.join(".claude").join("todos");
        if !global_dir.exists() {
            fs::create_dir_all(&global_dir)?;
        }

        Ok(global_dir.join("TODO.md"))
    }

    /// Add a new todo item
    pub fn add_todo(&mut self, description: String, priority: Priority, tags: Vec<String>) {
        let created_at = chrono::Local::now().format("%Y-%m-%d").to_string();

        let todo = TodoItem {
            description,
            status: TaskStatus::Pending,
            priority,
            tags,
            created_at: Some(created_at),
            completed_at: None,
            line_number: 0,
        };

        self.todos.push(todo);
    }

    /// Mark todo as completed by index (0-based)
    pub fn complete_todo(&mut self, index: usize) -> Result<()> {
        let pending: Vec<usize> = self
            .todos
            .iter()
            .enumerate()
            .filter(|(_, t)| !t.status.is_completed())
            .map(|(i, _)| i)
            .collect();

        if index >= pending.len() {
            return Err(CldevError::command(format!(
                "Invalid todo index: {}",
                index
            )));
        }

        let todo_index = pending[index];
        let completed_at = chrono::Local::now().format("%Y-%m-%d").to_string();

        self.todos[todo_index].status = TaskStatus::Completed;
        self.todos[todo_index].completed_at = Some(completed_at);

        Ok(())
    }

    /// Get pending todos
    pub fn get_pending(&self) -> Vec<&TodoItem> {
        self.todos
            .iter()
            .filter(|t| !t.status.is_completed())
            .collect()
    }

    /// Sync with git commits to auto-complete todos
    pub fn sync_with_git(&mut self) -> Result<usize> {
        use std::process::Command;

        // Get recent commits
        let output = Command::new("git")
            .args(["log", "--oneline", "-20"])
            .output()?;

        if !output.status.success() {
            return Err(CldevError::command("Failed to read git log"));
        }

        let commits = String::from_utf8_lossy(&output.stdout);
        let mut completed_count = 0;

        // Auto-complete todos that match commit messages
        for line in commits.lines() {
            let commit_lower = line.to_lowercase();

            for todo in &mut self.todos {
                if !todo.status.is_completed() {
                    let desc_lower = todo.description.to_lowercase();

                    // Simple keyword matching
                    let desc_words: Vec<&str> = desc_lower.split_whitespace().collect();
                    let matches: usize = desc_words
                        .iter()
                        .filter(|word| word.len() > 3 && commit_lower.contains(*word))
                        .count();

                    // Auto-complete if enough keywords match
                    if matches >= (desc_words.len().min(3)) {
                        todo.status = TaskStatus::Completed;
                        todo.completed_at =
                            Some(chrono::Local::now().format("%Y-%m-%d").to_string());
                        completed_count += 1;
                    }
                }
            }
        }

        Ok(completed_count)
    }
}

/// Add a new todo
pub fn add_todo(description: Option<String>) -> Result<()> {
    println!("{}", "➕ Add New Todo".cyan().bold());

    let mut todo_list = TodoList::load()?;

    // Get description
    let desc = match description {
        Some(d) => d,
        None => Input::new()
            .with_prompt("Todo description")
            .interact_text()?,
    };

    // Get priority
    let priorities = vec!["📝 Low", "📌 Medium", "⚠️ High", "🔥 Critical"];
    let priority_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Priority")
        .items(&priorities)
        .default(1)
        .interact()?;

    let priority = match priority_idx {
        0 => Priority::Low,
        1 => Priority::Medium,
        2 => Priority::High,
        3 => Priority::Critical,
        _ => Priority::Medium,
    };

    // Get tags (optional)
    let tags_input: String = Input::new()
        .with_prompt("Tags (space-separated, optional)")
        .allow_empty(true)
        .interact_text()?;

    let tags: Vec<String> = tags_input
        .split_whitespace()
        .map(|s| s.trim_start_matches('#').to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Add todo
    todo_list.add_todo(desc.clone(), priority, tags);
    todo_list.save()?;

    println!("{}", "\n✅ Todo added successfully!".green());
    println!("{} Description: {}", "ℹ️".cyan(), desc);
    println!(
        "{} File: {}",
        "ℹ️".cyan(),
        todo_list.file_path.display().to_string().yellow()
    );

    Ok(())
}

/// List all todos
pub fn list_todos() -> Result<()> {
    println!("{}", "📋 Todo List".cyan().bold());

    let todo_list = TodoList::load()?;
    let pending = todo_list.get_pending();

    if pending.is_empty() {
        println!("{}", "\n✅ No pending todos!".green());
        return Ok(());
    }

    println!("\n{} {} pending todo(s)", "ℹ️".cyan(), pending.len());
    println!(
        "{} File: {}\n",
        "ℹ️".cyan(),
        todo_list.file_path.display().to_string().dimmed()
    );

    // Group by priority
    let priorities = vec![
        Priority::Critical,
        Priority::High,
        Priority::Medium,
        Priority::Low,
    ];

    for priority in priorities {
        let items: Vec<&TodoItem> = pending
            .iter()
            .filter(|t| t.priority == priority)
            .copied()
            .collect();

        if items.is_empty() {
            continue;
        }

        println!(
            "\n{} {} {}",
            priority.to_emoji(),
            priority.to_string().bold(),
            format!("({})", items.len()).dimmed()
        );

        for (i, todo) in items.iter().enumerate() {
            let tags_str = if !todo.tags.is_empty() {
                format!(
                    " {}",
                    todo.tags
                        .iter()
                        .map(|t| format!("#{}", t))
                        .collect::<Vec<_>>()
                        .join(" ")
                        .dimmed()
                )
            } else {
                String::new()
            };

            println!("  {}. {}{}", i + 1, todo.description, tags_str);

            if let Some(created) = &todo.created_at {
                println!("     {}", format!("created: {}", created).dimmed());
            }
        }
    }

    // Display completed summary
    let completed_count = todo_list
        .todos
        .iter()
        .filter(|t| t.status.is_completed())
        .count();
    if completed_count > 0 {
        println!("\n{} {} completed todo(s)", "✅".green(), completed_count);
    }

    Ok(())
}

/// Complete a todo
pub fn complete_todo() -> Result<()> {
    println!("{}", "✅ Complete Todo".cyan().bold());

    let mut todo_list = TodoList::load()?;
    let pending = todo_list.get_pending();

    if pending.is_empty() {
        println!("{}", "\n✅ No pending todos!".green());
        return Ok(());
    }

    // Create selection list
    let items: Vec<String> = pending
        .iter()
        .map(|t| {
            let tags_str = if !t.tags.is_empty() {
                format!(
                    " {}",
                    t.tags
                        .iter()
                        .map(|tag| format!("#{}", tag))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                String::new()
            };
            format!("{} {}{}", t.priority.to_emoji(), t.description, tags_str)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select todo to complete")
        .items(&items)
        .interact()?;

    let todo_desc = pending[selection].description.clone();

    todo_list.complete_todo(selection)?;
    todo_list.save()?;

    println!("\n{} Todo completed: {}", "✅".green(), todo_desc);

    Ok(())
}

/// Sync todos with git
pub fn sync_todos() -> Result<()> {
    println!("{}", "🔄 Syncing todos with git...".cyan().bold());

    let mut todo_list = TodoList::load()?;

    let completed_count = todo_list.sync_with_git()?;
    todo_list.save()?;

    println!("\n{} Sync completed!", "✅".green());
    println!(
        "{} {} todo(s) auto-completed based on git commits",
        "ℹ️".cyan(),
        completed_count
    );

    Ok(())
}

/// Interactive todo management mode
pub fn interactive_mode() -> Result<()> {
    println!("{}", "🎯 Interactive Todo Management".cyan().bold());

    loop {
        let actions = vec![
            "📋 List todos",
            "➕ Add todo",
            "✅ Complete todo",
            "🔄 Sync with git",
            "🚪 Exit",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(&actions)
            .interact()?;

        match choice {
            0 => list_todos()?,
            1 => add_todo(None)?,
            2 => complete_todo()?,
            3 => sync_todos()?,
            4 => {
                println!("{}", "👋 Goodbye!".cyan());
                break;
            }
            _ => {}
        }

        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let content = r#"# Personal TODOs

## 🔥 Critical

## ⚠️ High
- [ ] Learning Record性能改善 #rust #performance (created: 2025-01-09)

## 📌 Medium
- [ ] TF-IDF検索精度向上 #search (created: 2025-01-09)

## 📝 Low

## ✅ Completed
- [x] READMEのコマンド数修正 (created: 2025-01-09, completed: 2025-01-09)
"#;

        let todos = TodoList::parse_markdown(content);

        assert_eq!(todos.len(), 3);

        // Check first todo
        assert_eq!(todos[0].description, "Learning Record性能改善");
        assert_eq!(todos[0].status, TaskStatus::Pending);
        assert_eq!(todos[0].priority, Priority::High);
        assert_eq!(todos[0].tags, vec!["rust", "performance"]);

        // Check completed todo
        assert!(todos[2].status.is_completed());
        assert_eq!(todos[2].description, "READMEのコマンド数修正");
    }

    #[test]
    fn test_to_markdown() {
        let mut todo_list = TodoList {
            todos: Vec::new(),
            file_path: PathBuf::from("test.md"),
        };

        todo_list.add_todo(
            "Test todo".to_string(),
            Priority::High,
            vec!["test".to_string()],
        );

        let markdown = todo_list.to_markdown();

        assert!(markdown.contains("# Personal TODOs"));
        assert!(markdown.contains("## ⚠️ High"));
        assert!(markdown.contains("- [ ] Test todo #test"));
    }

    #[test]
    fn test_complete_todo() {
        let mut todo_list = TodoList {
            todos: Vec::new(),
            file_path: PathBuf::from("test.md"),
        };

        todo_list.add_todo("Test todo".to_string(), Priority::Medium, vec![]);

        todo_list.complete_todo(0).unwrap();

        assert_eq!(todo_list.todos[0].status, TaskStatus::Completed);
        assert!(todo_list.todos[0].completed_at.is_some());
    }
}
